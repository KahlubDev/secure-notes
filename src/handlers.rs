use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use serde::Deserialize;
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

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub title: String,
}

pub async fn search_notes(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Vec<Note>>, StatusCode> {
    let notes = sqlx::query_as::<_, (String, String, String, String)>(
    "SELECT id, title, content, created_at
     FROM notes
     WHERE title = ?",
)
.bind(&query.title)
.fetch_all(&state.db)
.await
.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let notes = notes
        .into_iter()
        .filter_map(|(id, title, content, created_at)| {
            Some(Note {
                id: Uuid::parse_str(&id).ok()?,
                title,
                content,
                created_at: chrono::DateTime::parse_from_rfc3339(&created_at)
                    .ok()?
                    .with_timezone(&Utc),
            })
        })
        .collect();

    Ok(Json(notes))
}