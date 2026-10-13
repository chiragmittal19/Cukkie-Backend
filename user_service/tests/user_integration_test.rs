use axum::{
    body::Body,
    http::{header, Request, StatusCode},
};
use serde_json::{json, Value};
use std::sync::Arc;
use tower::ServiceExt;
use user_service::repository::{InMemoryConfigRepository, InMemoryUserRepository};
use user_service::service::{AuthService, ConfigService, UserService};
use user_service::{create_router, AppState};

fn setup_test_app() -> axum::Router {
    let user_repo = Arc::new(InMemoryUserRepository::new());
    let config_repo = Arc::new(InMemoryConfigRepository::new());

    let auth_service = Arc::new(AuthService::new(user_repo.clone()));
    let user_service = Arc::new(UserService::new(user_repo.clone()));
    let config_service = Arc::new(ConfigService::new(config_repo.clone()));

    let state = AppState {
        auth_service,
        user_service,
        config_service,
    };

    create_router(state)
}

#[tokio::test]
async fn test_health_check() {
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
async fn test_auth_and_user_lifecycle() {
    let app = setup_test_app();
    let phone = "+91 99999 88888";

    // 1. Send OTP
    let send_req = Request::builder()
        .method("POST")
        .uri("/api/v1/auth/send-otp")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json!({ "phoneNumber": phone }).to_string()))
        .unwrap();

    let send_res = app.clone().oneshot(send_req).await.unwrap();
    assert_eq!(send_res.status(), StatusCode::OK);

    // 2. Verify OTP
    let verify_req = Request::builder()
        .method("POST")
        .uri("/api/v1/auth/verify-otp")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            json!({
                "phoneNumber": phone,
                "otp": "1234"
            })
            .to_string(),
        ))
        .unwrap();

    let verify_res = app.clone().oneshot(verify_req).await.unwrap();
    assert_eq!(verify_res.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(verify_res.into_body(), usize::MAX)
        .await
        .unwrap();
    let user_json: Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(user_json["phoneNumber"], phone);
    assert_eq!(user_json["isLoggedIn"], true);
    let token = user_json["token"].as_str().unwrap();

    // 3. Get User Profile with Bearer token
    let profile_req = Request::builder()
        .method("GET")
        .uri("/api/v1/user/profile")
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let profile_res = app.clone().oneshot(profile_req).await.unwrap();
    assert_eq!(profile_res.status(), StatusCode::OK);

    // 4. Update User Preferences
    let update_req = Request::builder()
        .method("PUT")
        .uri("/api/v1/user/preferences")
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            json!({
                "cuisines": ["Italian", "Pan-Asian"],
                "dietaryChoices": ["Vegetarian"],
                "spiceLevel": "Spicy",
                "sweetnessLevel": "Low"
            })
            .to_string(),
        ))
        .unwrap();

    let update_res = app.clone().oneshot(update_req).await.unwrap();
    assert_eq!(update_res.status(), StatusCode::OK);

    let updated_bytes = axum::body::to_bytes(update_res.into_body(), usize::MAX)
        .await
        .unwrap();
    let prefs_json: Value = serde_json::from_slice(&updated_bytes).unwrap();
    assert_eq!(prefs_json["hasSavedPreferences"], true);
    assert_eq!(prefs_json["spiceLevel"], "Spicy");
}

#[tokio::test]
async fn test_multilingual_config_via_accept_language_header() {
    let app = setup_test_app();

    // 1. Get preferences in Hindi
    let req_hi = Request::builder()
        .method("GET")
        .uri("/api/v1/config/preferences")
        .header(header::ACCEPT_LANGUAGE, "hi-IN,hi;q=0.9")
        .body(Body::empty())
        .unwrap();

    let res_hi = app.clone().oneshot(req_hi).await.unwrap();
    assert_eq!(res_hi.status(), StatusCode::OK);
    let bytes_hi = axum::body::to_bytes(res_hi.into_body(), usize::MAX)
        .await
        .unwrap();
    let config_hi: Value = serde_json::from_slice(&bytes_hi).unwrap();
    assert_eq!(config_hi["title"], "आपकी भोजन प्राथमिकताएँ");

    // 2. Get strings in Arabic
    let req_ar = Request::builder()
        .method("GET")
        .uri("/api/v1/config/strings")
        .header(header::ACCEPT_LANGUAGE, "ar-SA,ar;q=0.8")
        .body(Body::empty())
        .unwrap();

    let res_ar = app.oneshot(req_ar).await.unwrap();
    assert_eq!(res_ar.status(), StatusCode::OK);
    let bytes_ar = axum::body::to_bytes(res_ar.into_body(), usize::MAX)
        .await
        .unwrap();
    let strings_ar: Value = serde_json::from_slice(&bytes_ar).unwrap();
    assert_eq!(strings_ar["appName"], "كوكي");
}
