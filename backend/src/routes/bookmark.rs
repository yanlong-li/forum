use axum::{Router, Json, routing::{get, delete}, extract::{Path, Query, State}};
use std::sync::Arc;
use uuid::Uuid;

use crate::models::AppState;
use crate::extractors::AuthenticatedUser;
use crate::models::response::PaginationQuery;
use crate::models::post::PostListResponse;
use crate::error::{AppError, Result};

pub fn bookmark_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_bookmarks).post(add_bookmark))
        .route("/{post_id}", delete(remove_bookmark))
}

async fn list_bookmarks(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<PostListResponse>> {
    let offset = (query.page.unwrap_or(1) - 1) * query.limit.unwrap_or(20);

    let posts: Vec<(String, String, String, String, String, String, String, String, Option<String>, Option<String>, String, Option<String>, i64, i64, bool)> = sqlx::query_as(
        r#"
        SELECT p.id, p.author_id, p.title, p.content, p.created_at, p.updated_at,
               u.id as u_id, u.username, u.avatar_url, u.bio, u.created_at as u_created_at,
               (SELECT GROUP_CONCAT(t.name) FROM tags t JOIN post_tags pt ON t.id = pt.tag_id WHERE pt.post_id = p.id) as tags,
               (SELECT COUNT(*) FROM votes WHERE post_id = p.id AND value = 1) as like_count,
               (SELECT COUNT(*) FROM comments WHERE post_id = p.id AND is_deleted = 0) as comment_count,
               u.is_admin as u_is_admin
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

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM bookmarks WHERE user_id = ?"
    )
    .bind(&user.id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| AppError::InternalError("Failed to count bookmarks".to_string()))?;

    let result: Vec<crate::models::post::PostSummary> = posts.into_iter().map(|p| {
        let tags: Vec<String> = p.11.map(|t| t.split(',').map(String::from).collect()).unwrap_or_default();
        crate::models::post::PostSummary {
            id: p.0,
            author_id: p.1,
            author: crate::models::user::UserPublic {
                id: p.6,
                username: p.7,
                avatar_url: p.8,
                bio: p.9,
                is_admin: p.14,
                created_at: p.10,
            },
            title: p.2,
            content: p.3,
            tags,
            like_count: p.12,
            comment_count: p.13,
            is_bookmarked: true,
            created_at: p.4,
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

    let now = chrono::Utc::now().to_rfc3339();
    let bookmark_id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO bookmarks (id, user_id, post_id, created_at) VALUES (?, ?, ?, ?)"
    )
    .bind(&bookmark_id)
    .bind(&user.id)
    .bind(post_id)
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
