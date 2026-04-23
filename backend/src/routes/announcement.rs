use axum::{Router, Json, routing::{get, post, patch, delete}, extract::{Path, State}};
use std::sync::Arc;
use uuid::Uuid;

use crate::models::AppState;
use crate::extractors::AuthenticatedUser;
use crate::error::{AppError, Result};

#[derive(Debug, serde::Deserialize)]
struct CreateAnnouncementRequest {
    title: String,
    content: String,
}

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
struct Announcement {
    id: String,
    title: String,
    content: String,
    is_active: bool,
    created_at: String,
}

pub fn announcement_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_announcements).post(create_announcement))
        .route("/{id}", patch(update_announcement).delete(delete_announcement))
}

async fn list_announcements(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Announcement>>> {
    let announcements: Vec<Announcement> = sqlx::query_as(
        "SELECT id, title, content, is_active, created_at FROM announcements WHERE is_active = 1 ORDER BY created_at DESC"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| AppError::InternalError("Failed to fetch announcements".to_string()))?;

    Ok(Json(announcements))
}

async fn create_announcement(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreateAnnouncementRequest>,
) -> Result<Json<Announcement>> {
    if !user.is_admin {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    if req.title.trim().is_empty() || req.content.trim().is_empty() {
        return Err(AppError::ValidationError("Title and content are required".to_string()));
    }

    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO announcements (id, title, content, is_active, created_at) VALUES (?, ?, ?, 1, ?)"
    )
    .bind(&id)
    .bind(&req.title)
    .bind(&req.content)
    .bind(&now)
    .execute(&state.db)
    .await
    .map_err(|_| AppError::InternalError("Failed to create announcement".to_string()))?;

    Ok(Json(Announcement {
        id,
        title: req.title,
        content: req.content,
        is_active: true,
        created_at: now,
    }))
}

async fn update_announcement(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<String>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<Announcement>> {
    if !user.is_admin {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let title = req["title"].as_str().unwrap_or("");
    let content = req["content"].as_str().unwrap_or("");
    let is_active = req["is_active"].as_bool().unwrap_or(true);

    sqlx::query(
        "UPDATE announcements SET title = ?, content = ?, is_active = ? WHERE id = ?"
    )
    .bind(title)
    .bind(content)
    .bind(is_active)
    .bind(&id)
    .execute(&state.db)
    .await
    .map_err(|_| AppError::InternalError("Failed to update announcement".to_string()))?;

    let announcement: (String, String, String, bool, String) = sqlx::query_as(
        "SELECT id, title, content, is_active, created_at FROM announcements WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| AppError::InternalError("Failed to fetch announcement".to_string()))?;

    Ok(Json(Announcement {
        id: announcement.0,
        title: announcement.1,
        content: announcement.2,
        is_active: announcement.3,
        created_at: announcement.4,
    }))
}

async fn delete_announcement(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<()> {
    if !user.is_admin {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    sqlx::query("DELETE FROM announcements WHERE id = ?")
        .bind(&id)
        .execute(&state.db)
        .await
        .map_err(|_| AppError::InternalError("Failed to delete announcement".to_string()))?;

    Ok(())
}