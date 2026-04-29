use axum::{Router, Json, routing::{get, post, patch, delete}, extract::{Path, State}};
use std::sync::Arc;
use uuid::Uuid;
use sqlx::FromRow;

use crate::models::AppState;
use crate::extractors::AuthenticatedUser;
use crate::error::{AppError, Result};

#[derive(Debug, serde::Deserialize)]
struct CreateFolderRequest {
    name: String,
}

#[derive(Debug, FromRow, serde::Serialize)]
struct FolderResponse {
    id: String,
    name: String,
    created_at: String,
}

pub fn folder_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_folders).post(create_folder))
        .route("/{id}", patch(update_folder).delete(delete_folder))
}

async fn list_folders(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<Json<Vec<FolderResponse>>> {
    let folders: Vec<FolderResponse> = sqlx::query_as(
        "SELECT id, name, created_at FROM folders WHERE user_id = ? ORDER BY created_at DESC"
    )
    .bind(&user.id)
    .fetch_all(&state.db)
    .await
    .map_err(|_| AppError::InternalError("Failed to fetch folders".to_string()))?;

    Ok(Json(folders))
}

async fn create_folder(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreateFolderRequest>,
) -> Result<Json<FolderResponse>> {
    if req.name.trim().is_empty() {
        return Err(AppError::ValidationError("Folder name is required".to_string()));
    }

    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO folders (id, user_id, name, created_at) VALUES (?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&user.id)
    .bind(&req.name)
    .bind(&now)
    .execute(&state.db)
    .await
    .map_err(|_| AppError::InternalError("Failed to create folder".to_string()))?;

    Ok(Json(FolderResponse {
        id,
        name: req.name,
        created_at: now,
    }))
}

async fn update_folder(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<String>,
    Json(req): Json<CreateFolderRequest>,
) -> Result<Json<FolderResponse>> {
    if req.name.trim().is_empty() {
        return Err(AppError::ValidationError("Folder name is required".to_string()));
    }

    let result = sqlx::query(
        "UPDATE folders SET name = ? WHERE id = ? AND user_id = ?"
    )
    .bind(&req.name)
    .bind(&id)
    .bind(&user.id)
    .execute(&state.db)
    .await
    .map_err(|_| AppError::InternalError("Failed to update folder".to_string()))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Folder not found".to_string()));
    }

    let folder: (String, String, String) = sqlx::query_as(
        "SELECT id, name, created_at FROM folders WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| AppError::InternalError("Failed to fetch folder".to_string()))?;

    Ok(Json(FolderResponse {
        id: folder.0,
        name: folder.1,
        created_at: folder.2,
    }))
}

async fn delete_folder(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<()> {
    sqlx::query("DELETE FROM folders WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(&user.id)
        .execute(&state.db)
        .await
        .map_err(|_| AppError::InternalError("Failed to delete folder".to_string()))?;

    sqlx::query("UPDATE bookmarks SET folder_id = NULL WHERE folder_id = ?")
        .bind(&id)
        .execute(&state.db)
        .await
        .map_err(|_| AppError::InternalError("Failed to update bookmarks".to_string()))?;

    Ok(())
}