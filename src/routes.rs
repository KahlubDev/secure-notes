use axum::{
    routing::get,
    Router,
};

async fn health_check() -> &'static str {
    "Secure Notes API is running."
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(health_check))
}