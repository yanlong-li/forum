use axum::{Router, Json, routing::get, extract::{Path, Query, State}};
use std::sync::Arc;

use crate::models::AppState;
use crate::extractors::{AuthenticatedUser, OptionalAuthenticatedUser};
use crate::services::PostService;
use crate::models::post::{CreatePostRequest, UpdatePostRequest, PostListResponse};
use crate::models::response::PostListQuery;
use crate::error::Result;

pub fn post_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_posts).post(create_post))
        .route("/hot", get(list_hot_posts))
        .route("/{id}", get(get_post).patch(update_post).delete(delete_post))
}

async fn create_post(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreatePostRequest>,
) -> Result<Json<crate::models::post::Post>> {
    let post_service = PostService::new(&state.db);
    post_service.create_post(&user.id, req).await.map(Json)
}

async fn get_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    OptionalAuthenticatedUser(user): OptionalAuthenticatedUser,
) -> Result<Json<crate::models::post::PostWithAuthor>> {
    let post_service = PostService::new(&state.db);
    post_service.get_post(&id, user.map(|u| u.id).as_deref()).await.map(Json)
}

async fn update_post(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<String>,
    Json(req): Json<UpdatePostRequest>,
) -> Result<Json<crate::models::post::Post>> {
    let post_service = PostService::new(&state.db);
    post_service.update_post(&id, &user.id, req).await.map(Json)
}

async fn delete_post(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<()> {
    let post_service = PostService::new(&state.db);
    post_service.delete_post(&id, &user.id).await
}

async fn list_posts(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PostListQuery>,
) -> Result<Json<PostListResponse>> {
    let post_service = PostService::new(&state.db);
    post_service.list_posts(query.page.unwrap_or(1), query.limit.unwrap_or(20), query.tag.as_deref(), query.sort.as_deref()).await.map(Json)
}

async fn list_hot_posts(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PostListQuery>,
) -> Result<Json<PostListResponse>> {
    let post_service = PostService::new(&state.db);
    post_service.list_hot_posts(query.page.unwrap_or(1), query.limit.unwrap_or(20)).await.map(Json)
}
