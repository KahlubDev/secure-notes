use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use tower::ServiceExt;

use secure_notes::{
    database,
    routes,
    state::AppState,
};

async fn test_app() -> axum::Router {
    let db = database::connect()
        .await
        .expect("Test database connection failed");

    let state = AppState { db };

    routes::router().with_state(state)
}

#[tokio::test]
async fn create_note_accepts_valid_input() {
    let app = test_app().await;

    let payload = json!({
        "title": "Security Test",
        "content": "This note should be created."
    });

    let request = Request::builder()
        .method("POST")
        .uri("/notes")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn create_note_rejects_empty_title() {
    let app = test_app().await;

    let payload = json!({
        "title": "",
        "content": "This should be rejected."
    });

    let request = Request::builder()
        .method("POST")
        .uri("/notes")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn create_note_rejects_title_over_100_characters() {
    let app = test_app().await;

    let payload = json!({
        "title": "A".repeat(101),
        "content": "This should be rejected."
    });

    let request = Request::builder()
        .method("POST")
        .uri("/notes")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn create_note_rejects_content_over_5000_characters() {
    let app = test_app().await;

    let payload = json!({
        "title": "Large Content",
        "content": "A".repeat(5001)
    });

    let request = Request::builder()
        .method("POST")
        .uri("/notes")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}