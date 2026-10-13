use axum::{
    body::Body,
    http::{header, Request, StatusCode},
};
use log_service::repository::InMemoryLogRepository;
use log_service::service::LogService;
use log_service::{create_router, AppState};
use serde_json::{json, Value};
use std::sync::Arc;
use tower::ServiceExt;

fn setup_test_app() -> axum::Router {
    let repo = Arc::new(InMemoryLogRepository::default());
    let log_service = Arc::new(LogService::new(repo));
    let state = AppState { log_service };
    create_router(state)
}

#[tokio::test]
async fn test_log_service_health() {
    let app = setup_test_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_unified_batch_log_ingestion() {
    let app = setup_test_app();

    let batch_req = Request::builder()
        .method("POST")
        .uri("/api/v1/logs")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            json!({
                "events": [
                    {
                        "type": "event",
                        "name": "dish_clicked",
                        "parameters": {
                            "dishId": "dish_pbm",
                            "screen": "home"
                        },
                        "timestamp": "2026-10-01T10:00:00Z"
                    },
                    {
                        "type": "metric",
                        "name": "recipe_load_time_ms",
                        "value": 142.8,
                        "parameters": {
                            "platform": "ios",
                            "version": "1.0.0"
                        },
                        "timestamp": "2026-10-01T10:00:01Z"
                    }
                ]
            })
            .to_string(),
        ))
        .unwrap();

    let response = app.oneshot(batch_req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_json: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body_json["success"], true);
    assert_eq!(body_json["ingested"], 2);
}
