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
#[tokio::test]
async fn search_treats_sql_like_input_as_data() {
    let app = test_app().await;

    let payload = json!({
        "title": "Safe Note",
        "content": "Private content"
    });

    let create_request = Request::builder()
        .method("POST")
        .uri("/notes")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let create_response = app.clone().oneshot(create_request).await.unwrap();

    assert_eq!(create_response.status(), StatusCode::CREATED);

    let malicious_title = "' OR '1'='1";

    let search_request = Request::builder()
        .method("GET")
        .uri(format!(
            "/notes/search?title={}",
            urlencoding::encode(malicious_title)
        ))
        .body(Body::empty())
        .unwrap();

    let search_response = app.oneshot(search_request).await.unwrap();

    assert_eq!(search_response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(
        search_response.into_body(),
        1024 * 1024,
    )
    .await
    .unwrap();

    let results: Vec<serde_json::Value> =
        serde_json::from_slice(&body).unwrap();

    assert!(
        results.is_empty(),
        "SQL-like input must not return unrelated notes"
    );
}