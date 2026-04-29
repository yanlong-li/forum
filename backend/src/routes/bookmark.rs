use axum::{Router, Json, routing::{get, delete, patch}, extract::{Path, Query, State}};
use std::sync::Arc;
use uuid::Uuid;
use sqlx::FromRow;

use crate::models::AppState;
use crate::extractors::AuthenticatedUser;
use crate::models::response::PaginationQuery;
use crate::models::post::PostListResponse;
use crate::error::{AppError, Result};

#[derive(Debug, FromRow)]
struct BookmarkListRow {
    id: String,
    author_id: String,
    title: String,
    content: String,
    created_at: String,
    updated_at: String,
    is_pinned: bool,
    is_featured: bool,
    view_count: i64,
    u_id: String,
    username: String,
    avatar_url: Option<String>,
    bio: Option<String>,
    u_created_at: String,
    points: i64,
    level: i64,
    u_is_admin: bool,
    tags: Option<String>,
    like_count: i64,
    comment_count: i64,
    folder_id: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct BookmarkQuery {
    page: Option<i64>,
    limit: Option<i64>,
    folder_id: Option<String>,
}

pub fn bookmark_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_bookmarks).post(add_bookmark))
        .route("/{post_id}", delete(remove_bookmark))
        .route("/{post_id}/move", patch(move_bookmark))
}

async fn list_bookmarks(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(query): Query<BookmarkQuery>,
) -> Result<Json<PostListResponse>> {
    let offset = (query.page.unwrap_or(1) - 1) * query.limit.unwrap_or(20);
    let folder_id = &query.folder_id;

    let (rows, total): (Vec<BookmarkListRow>, i64) = if let Some(fid) = folder_id {
        let rows = sqlx::query_as(
            r#"
            SELECT p.id, p.author_id, p.title, p.content, p.created_at, p.updated_at, p.is_pinned, p.is_featured, p.view_count,
                   u.id as u_id, u.username, u.avatar_url, u.bio, u.created_at as u_created_at, u.points, u.level, u.is_admin as u_is_admin,
                   (SELECT GROUP_CONCAT(t.name) FROM tags t JOIN post_tags pt ON t.id = pt.tag_id WHERE pt.post_id = p.id) as tags,
                   (SELECT COUNT(*) FROM votes WHERE post_id = p.id AND value = 1) as like_count,
                   (SELECT COUNT(*) FROM comments WHERE post_id = p.id AND is_deleted = 0) as comment_count,
                   b.folder_id
            FROM bookmarks b
            JOIN posts p ON b.post_id = p.id
            JOIN users u ON p.author_id = u.id
            WHERE b.user_id = ? AND p.is_deleted = 0 AND b.folder_id = ?
            ORDER BY b.created_at DESC
            LIMIT ? OFFSET ?
            "#
        )
        .bind(&user.id)
        .bind(fid)
        .bind(query.limit.unwrap_or(20))
        .bind(offset)
        .fetch_all(&state.db)
        .await
        .map_err(|_| AppError::InternalError("Failed to fetch bookmarks".to_string()))?;

        let total = sqlx::query_scalar(
            "SELECT COUNT(*) FROM bookmarks WHERE user_id = ? AND folder_id = ?"
        )
        .bind(&user.id)
        .bind(fid)
        .fetch_one(&state.db)
        .await
        .map_err(|_| AppError::InternalError("Failed to count bookmarks".to_string()))?;

        (rows, total)
    } else {
        let rows = sqlx::query_as(
            r#"
            SELECT p.id, p.author_id, p.title, p.content, p.created_at, p.updated_at, p.is_pinned, p.is_featured, p.view_count,
                   u.id as u_id, u.username, u.avatar_url, u.bio, u.created_at as u_created_at, u.points, u.level, u.is_admin as u_is_admin,
                   (SELECT GROUP_CONCAT(t.name) FROM tags t JOIN post_tags pt ON t.id = pt.tag_id WHERE pt.post_id = p.id) as tags,
                   (SELECT COUNT(*) FROM votes WHERE post_id = p.id AND value = 1) as like_count,
                   (SELECT COUNT(*) FROM comments WHERE post_id = p.id AND is_deleted = 0) as comment_count,
                   b.folder_id
            FROM bookmarks b
            JOIN posts p ON b.post_id = p.id
            JOIN users u ON p.author_id = u.id
            WHERE b.user_id = ? AND p.is_deleted = 0
            ORDER BY b.created_at DESC
            LIMIT ? OFFSET ?
            "#
        )
        .bind(&user.id)
        .bind(query.limit.unwrap_or(20))
        .bind(offset)
        .fetch_all(&state.db)
        .await
        .map_err(|_| AppError::InternalError("Failed to fetch bookmarks".to_string()))?;

        let total = sqlx::query_scalar(
            "SELECT COUNT(*) FROM bookmarks WHERE user_id = ?"
        )
        .bind(&user.id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| AppError::InternalError("Failed to count bookmarks".to_string()))?;

        (rows, total)
    };

    let result: Vec<crate::models::post::PostSummary> = rows.into_iter().map(|row| {
        let tags: Vec<String> = row.tags.map(|t| t.split(',').map(String::from).collect()).unwrap_or_default();
        crate::models::post::PostSummary {
            id: row.id,
            author_id: row.author_id,
            author: crate::models::user::UserPublic {
                id: row.u_id,
                username: row.username,
                avatar_url: row.avatar_url,
                bio: row.bio,
                is_admin: row.u_is_admin,
                points: row.points,
                level: row.level,
                created_at: row.u_created_at,
            },
            title: row.title,
            content: row.content,
            tags,
            like_count: row.like_count,
            comment_count: row.comment_count,
            is_bookmarked: true,
            is_pinned: row.is_pinned,
            is_featured: row.is_featured,
            view_count: row.view_count,
            created_at: row.created_at,
        }
    }).collect();

    Ok(Json(PostListResponse {
        posts: result,
        total,
        page: query.page.unwrap_or(1),
        limit: query.limit.unwrap_or(20),
    }))
}

async fn add_bookmark(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<crate::models::response::MessageResponse>> {
    let post_id = req["post_id"].as_str()
        .ok_or_else(|| AppError::ValidationError("post_id is required".to_string()))?;

    let folder_id = req["folder_id"].as_str();

    let existing: Option<(String,)> = sqlx::query_as(
        "SELECT id FROM bookmarks WHERE user_id = ? AND post_id = ?"
    )
    .bind(&user.id)
    .bind(post_id)
    .fetch_optional(&state.db)
    .await?;

    if existing.is_some() {
        return Err(AppError::Conflict("Already bookmarked".to_string()));
    }

    let now = chrono::Utc::now().to_rfc3339();
    let bookmark_id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO bookmarks (id, user_id, post_id, folder_id, created_at) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&bookmark_id)
    .bind(&user.id)
    .bind(post_id)
    .bind(folder_id)
    .bind(&now)
    .execute(&state.db)
    .await
    .map_err(|_| AppError::Conflict("Already bookmarked".to_string()))?;

    Ok(Json(crate::models::response::MessageResponse {
        message: "Post bookmarked".to_string()
    }))
}

async fn remove_bookmark(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(post_id): Path<String>,
) -> Result<()> {
    sqlx::query("DELETE FROM bookmarks WHERE user_id = ? AND post_id = ?")
        .bind(&user.id)
        .bind(&post_id)
        .execute(&state.db)
        .await
        .map_err(|_| AppError::InternalError("Failed to remove bookmark".to_string()))?;
    Ok(())
}

async fn move_bookmark(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(post_id): Path<String>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<crate::models::response::MessageResponse>> {
    let folder_id = req["folder_id"].as_str();

    sqlx::query("UPDATE bookmarks SET folder_id = ? WHERE user_id = ? AND post_id = ?")
        .bind(folder_id)
        .bind(&user.id)
        .bind(&post_id)
        .execute(&state.db)
        .await
        .map_err(|_| AppError::InternalError("Failed to move bookmark".to_string()))?;

    Ok(Json(crate::models::response::MessageResponse {
        message: "Bookmark moved".to_string()
    }))
}
