use axum::{Router, Json, routing::{get, post, delete}, extract::{Path, State}};
use std::sync::Arc;
use serde::Serialize;

use crate::models::AppState;
use crate::extractors::AuthenticatedUser;
use crate::error::{AppError, Result};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Block {
    pub id: String,
    pub user_id: String,
    pub blocked_user_id: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct BlockedUser {
    pub id: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub created_at: String,
}

pub fn block_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_blocks).post(block_user))
        .route("/{blocked_id}", delete(unblock_user))
}

async fn list_blocks(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<Json<Vec<BlockedUser>>> {
    let blocks: Vec<BlockedUser> = sqlx::query_as(
        r#"
        SELECT u.id, u.username, u.avatar_url, u.bio, u.created_at
        FROM user_blocks b
        JOIN users u ON b.blocked_user_id = u.id
        WHERE b.user_id = ?
        ORDER BY b.created_at DESC
        "#
    )
    .bind(&user.id)
    .fetch_all(&state.db)
    .await
    .map_err(|_| AppError::InternalError("Failed to fetch blocks".to_string()))?;

    Ok(Json(blocks))
}

async fn block_user(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(blocked_id): Path<String>,
) -> Result<Json<crate::models::response::MessageResponse>> {
    if user.id == blocked_id {
        return Err(AppError::ValidationError("Cannot block yourself".to_string()));
    }

    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO user_blocks (id, user_id, blocked_user_id, created_at) VALUES (?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&user.id)
    .bind(&blocked_id)
    .bind(&now)
    .execute(&state.db)
    .await
    .map_err(|_| AppError::Conflict("Already blocked".to_string()))?;

    Ok(Json(crate::models::response::MessageResponse {
        message: "User blocked".to_string()
    }))
}

async fn unblock_user(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(blocked_id): Path<String>,
) -> Result<Json<crate::models::response::MessageResponse>> {
    sqlx::query("DELETE FROM user_blocks WHERE user_id = ? AND blocked_user_id = ?")
        .bind(&user.id)
        .bind(&blocked_id)
        .execute(&state.db)
        .await
        .map_err(|_| AppError::InternalError("Failed to unblock user".to_string()))?;

    Ok(Json(crate::models::response::MessageResponse {
        message: "User unblocked".to_string()
    }))
}