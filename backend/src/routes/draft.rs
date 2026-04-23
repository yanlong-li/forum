use axum::{Router, Json, routing::{get, post, put, delete}, extract::{Path, State}};
use std::sync::Arc;

use crate::models::AppState;
use crate::extractors::AuthenticatedUser;
use crate::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Draft {
    pub id: String,
    pub user_id: String,
    pub draft_type: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SaveDraftRequest {
    pub draft_type: String,
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateDraftRequest {
    pub title: Option<String>,
    pub content: Option<String>,
}

pub fn draft_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_drafts).post(save_draft))
        .route("/{id}", put(update_draft).delete(delete_draft))
}

async fn list_drafts(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<Json<Vec<Draft>>> {
    let drafts: Vec<Draft> = sqlx::query_as(
        "SELECT * FROM drafts WHERE user_id = ? ORDER BY updated_at DESC"
    )
    .bind(&user.id)
    .fetch_all(&state.db)
    .await
    .map_err(|_| AppError::InternalError("Failed to fetch drafts".to_string()))?;

    Ok(Json(drafts))
}

async fn save_draft(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<SaveDraftRequest>,
) -> Result<Json<Draft>> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO drafts (id, user_id, draft_type, title, content, updated_at) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&user.id)
    .bind(&req.draft_type)
    .bind(&req.title)
    .bind(&req.content)
    .bind(&now)
    .execute(&state.db)
    .await
    .map_err(|_| AppError::InternalError("Failed to save draft".to_string()))?;

    Ok(Json(Draft {
        id,
        user_id: user.id,
        draft_type: req.draft_type,
        title: req.title,
        content: req.content,
        updated_at: now,
    }))
}

async fn update_draft(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<String>,
    Json(req): Json<UpdateDraftRequest>,
) -> Result<Json<Draft>> {
    let now = chrono::Utc::now().to_rfc3339();

    let result = sqlx::query_as::<_, Draft>(
        "UPDATE drafts SET title = COALESCE(?, title), content = COALESCE(?, content), updated_at = ? WHERE id = ? AND user_id = ? RETURNING *"
    )
    .bind(&req.title)
    .bind(&req.content)
    .bind(&now)
    .bind(&id)
    .bind(&user.id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| AppError::InternalError("Failed to update draft".to_string()))?;

    match result {
        Some(draft) => Ok(Json(draft)),
        None => Err(AppError::NotFound("Draft not found".to_string())),
    }
}

async fn delete_draft(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<Json<crate::models::response::MessageResponse>> {
    sqlx::query("DELETE FROM drafts WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(&user.id)
        .execute(&state.db)
        .await
        .map_err(|_| AppError::InternalError("Failed to delete draft".to_string()))?;

    Ok(Json(crate::models::response::MessageResponse {
        message: "Draft deleted".to_string()
    }))
}