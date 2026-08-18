use axum::{
    routing::{get, post},
    Router,
};

use crate::{handlers, state::AppState};

async fn health_check() -> &'static str {
    "Secure Notes API is running."
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(health_check))
        .route("/notes", post(handlers::create_note))
        .route("/notes/search", get(handlers::search_notes))
}