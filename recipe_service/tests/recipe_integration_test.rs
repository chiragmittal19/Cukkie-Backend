use axum::{
    body::Body,
    http::{header, Request, StatusCode},
};
use common::{create_jwt, DEFAULT_JWT_SECRET};
use recipe_service::repository::InMemoryRecipeRepository;
use recipe_service::service::RecipeService;
use recipe_service::{create_router, AppState};
use serde_json::{json, Value};
use std::sync::Arc;
use tower::ServiceExt;

fn setup_test_app() -> axum::Router {
    let repo = Arc::new(InMemoryRecipeRepository::new());
    let recipe_service = Arc::new(RecipeService::new(repo));
    let state = AppState { recipe_service };
    create_router(state)
}

#[tokio::test]
async fn test_recipe_service_health() {
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
async fn test_home_feed_and_language_header() {
    let app = setup_test_app();

    // 1. English home feed
    let req_en = Request::builder()
        .method("GET")
        .uri("/api/v1/recipes/home")
        .header(header::ACCEPT_LANGUAGE, "en-US,en;q=0.9")
        .body(Body::empty())
        .unwrap();

    let res_en = app.clone().oneshot(req_en).await.unwrap();
    assert_eq!(res_en.status(), StatusCode::OK);
    let bytes_en = axum::body::to_bytes(res_en.into_body(), usize::MAX)
        .await
        .unwrap();
    let home_en: Value = serde_json::from_slice(&bytes_en).unwrap();
    assert_eq!(home_en["headerTitle"], "Cukkie Recipes");
    assert!(home_en["dishes"].as_array().unwrap().len() >= 6);

    // 2. Hindi home feed via Accept-Language header
    let req_hi = Request::builder()
        .method("GET")
        .uri("/api/v1/recipes/home")
        .header(header::ACCEPT_LANGUAGE, "hi-IN,hi;q=0.9")
        .body(Body::empty())
        .unwrap();

    let res_hi = app.clone().oneshot(req_hi).await.unwrap();
    assert_eq!(res_hi.status(), StatusCode::OK);
    let bytes_hi = axum::body::to_bytes(res_hi.into_body(), usize::MAX)
        .await
        .unwrap();
    let home_hi: Value = serde_json::from_slice(&bytes_hi).unwrap();
    assert_eq!(home_hi["headerTitle"], "कुक्की रेसिपीज़");
}

#[tokio::test]
async fn test_search_and_single_filter_param() {
    let app = setup_test_app();

    // 1. Search for "paneer"
    let req_search = Request::builder()
        .method("GET")
        .uri("/api/v1/recipes/home?search=paneer")
        .body(Body::empty())
        .unwrap();

    let res_search = app.clone().oneshot(req_search).await.unwrap();
    assert_eq!(res_search.status(), StatusCode::OK);
    let bytes_search = axum::body::to_bytes(res_search.into_body(), usize::MAX)
        .await
        .unwrap();
    let json_search: Value = serde_json::from_slice(&bytes_search).unwrap();
    let dishes = json_search["dishes"].as_array().unwrap();
    assert_eq!(dishes.len(), 1);
    assert_eq!(dishes[0]["id"], "dish_pbm");

    // 2. Single structured filter parameter: "cuisines:Maharashtrian;dietary:Vegan"
    let req_filter = Request::builder()
        .method("GET")
        .uri("/api/v1/recipes/home?filters=cuisines:Maharashtrian;dietary:Vegan")
        .body(Body::empty())
        .unwrap();

    let res_filter = app.clone().oneshot(req_filter).await.unwrap();
    assert_eq!(res_filter.status(), StatusCode::OK);
    let bytes_filter = axum::body::to_bytes(res_filter.into_body(), usize::MAX)
        .await
        .unwrap();
    let json_filter: Value = serde_json::from_slice(&bytes_filter).unwrap();
    let filter_dishes = json_filter["dishes"].as_array().unwrap();
    assert_eq!(filter_dishes.len(), 1);
    assert_eq!(filter_dishes[0]["id"], "dish_misal");
}

#[tokio::test]
async fn test_dish_detail_query_param() {
    let app = setup_test_app();

    let req = Request::builder()
        .method("GET")
        .uri("/api/v1/recipes/detail?id=dish_pbm")
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
        .await
        .unwrap();
    let detail: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(detail["id"], "dish_pbm");
    assert_eq!(detail["name"], "Paneer Butter Masala");
    assert!(detail["steps"].as_array().unwrap().len() >= 5);
    assert!(detail["ingredients"].as_array().unwrap().len() >= 4);
}

#[tokio::test]
async fn test_favourites_and_feedback_flow() {
    let app = setup_test_app();
    let user_phone = "+91 91111 22222";
    let token = create_jwt(user_phone, DEFAULT_JWT_SECRET, 24).unwrap();

    // 1. Toggle favorite for dish_misal using query parameter ?id=...
    let fav_req = Request::builder()
        .method("POST")
        .uri("/api/v1/recipes/favourite?id=dish_misal")
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let fav_res = app.clone().oneshot(fav_req).await.unwrap();
    assert_eq!(fav_res.status(), StatusCode::OK);

    let fav_bytes = axum::body::to_bytes(fav_res.into_body(), usize::MAX)
        .await
        .unwrap();
    let fav_json: Value = serde_json::from_slice(&fav_bytes).unwrap();
    assert_eq!(fav_json["isFavourite"], true);

    // 2. Fetch user favourites
    let list_req = Request::builder()
        .method("GET")
        .uri("/api/v1/recipes/favourites")
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let list_res = app.clone().oneshot(list_req).await.unwrap();
    assert_eq!(list_res.status(), StatusCode::OK);

    let list_bytes = axum::body::to_bytes(list_res.into_body(), usize::MAX)
        .await
        .unwrap();
    let list_json: Value = serde_json::from_slice(&list_bytes).unwrap();
    let fav_list = list_json.as_array().unwrap();
    assert!(fav_list.iter().any(|d| d["id"] == "dish_misal"));

    // 3. Submit feedback using query parameter ?id=...
    let feedback_req = Request::builder()
        .method("POST")
        .uri("/api/v1/recipes/feedback?id=dish_misal")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            json!({
                "responses": {
                    "rating": 5,
                    "review": "Authentic spicy misal! Great recipe."
                }
            })
            .to_string(),
        ))
        .unwrap();

    let feedback_res = app.oneshot(feedback_req).await.unwrap();
    assert_eq!(feedback_res.status(), StatusCode::OK);
}
