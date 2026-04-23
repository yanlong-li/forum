use axum::{Router, Json, routing::{get, patch, post}, extract::{Path, Query, State}};
use std::sync::Arc;

use crate::models::AppState;
use crate::extractors::AuthenticatedUser;
use crate::services::CommentService;
use crate::models::comment::{Comment, CreateCommentRequest, UpdateCommentRequest, CommentListResponse};
use crate::models::response::{PaginationQuery, MessageResponse};
use crate::error::{Result, AppError};

pub fn comment_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/posts/{post_id}/comments", get(list_comments).post(create_comment))
        .route("/comments/{id}", patch(update_comment).delete(delete_comment))
        .route("/comments/{id}/accept", post(accept_comment))
        .route("/user/{user_id}", get(list_user_comments))
}

async fn create_comment(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(post_id): Path<String>,
    Json(req): Json<CreateCommentRequest>,
) -> Result<Json<Comment>> {
    let comment_service = CommentService::new(&state.db);
    comment_service.create_comment(&post_id, &user.id, req).await.map(Json)
}

async fn list_comments(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<String>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<CommentListResponse>> {
    let comment_service = CommentService::new(&state.db);
    comment_service.list_comments(&post_id, query.page(), query.limit()).await.map(Json)
}

async fn update_comment(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<String>,
    Json(req): Json<UpdateCommentRequest>,
) -> Result<Json<Comment>> {
    let comment_service = CommentService::new(&state.db);
    comment_service.update_comment(&id, &user.id, req).await.map(Json)
}

async fn delete_comment(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<()> {
    let comment_service = CommentService::new(&state.db);
    comment_service.delete_comment(&id, &user.id).await
}

async fn list_user_comments(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<CommentListResponse>> {
    let comment_service = CommentService::new(&state.db);
    comment_service.list_user_comments(&user_id, query.page(), query.limit()).await.map(Json)
}

async fn accept_comment(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(comment_id): Path<String>,
) -> Result<Json<MessageResponse>> {
    let comment_service = CommentService::new(&state.db);
    comment_service.accept_comment(&comment_id, &user.id).await.map(Json)
}
