use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

use crate::{
    models::{CreateNoteRequest, Note},
    state::AppState,
};

pub async fn create_note(
    State(state): State<AppState>,
    Json(payload): Json<CreateNoteRequest>,
) -> Result<(StatusCode, Json<Note>), StatusCode> {
    payload
        .validate()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let id = Uuid::new_v4();
    let created_at = Utc::now();

    sqlx::query(
        "INSERT INTO notes (id, title, content, created_at)
         VALUES (?, ?, ?, ?)",
    )
    .bind(id.to_string())
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(created_at.to_rfc3339())
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let note = Note {
        id,
        title: payload.title,
        content: payload.content,
        created_at,
    };

    Ok((StatusCode::CREATED, Json(note)))
}