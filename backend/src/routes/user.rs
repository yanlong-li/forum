use axum::{Router, Json, routing::{get, patch}, extract::{Path, Query}};
use axum::extract::State;
use std::sync::Arc;

use crate::models::AppState;
use crate::extractors::AuthenticatedUser;
use crate::services::UserService;
use crate::models::user::{UserProfile, UpdateProfileRequest, UserPublic};
use crate::models::response::PaginationQuery;
use crate::models::follow::UserListResponse;
use crate::error::Result;

pub fn user_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/me", patch(update_me).get(get_me))
        .route("/search", get(search_users))
        .route("/{username}", get(get_user_profile))
        .route("/{username}/posts", get(get_user_posts))
        .route("/{username}/followers", get(get_followers))
        .route("/{username}/following", get(get_following))
}

async fn get_me(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<Json<UserPublic>> {
    let user_service = UserService::new(&state.db);
    user_service.get_user_by_id(&user.id).await.map(Json)
}

async fn search_users(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<SearchUsersResponse>> {
    let users = sqlx::query_as::<_, (String, Option<String>)>(
        "SELECT username, avatar_url FROM users WHERE username LIKE ? LIMIT 10"
    )
    .bind(format!("%{}%", query.q))
    .fetch_all(&state.db)
    .await
    .map_err(|_| crate::error::AppError::InternalError("Search failed".to_string()))?;

    Ok(Json(SearchUsersResponse {
        users: users.into_iter().map(|(username, avatar_url)| UserSearchResult { username, avatar_url }).collect()
    }))
}

#[derive(Debug, serde::Deserialize)]
struct SearchQuery {
    q: String,
    limit: Option<i64>,
}

#[derive(Debug, serde::Serialize)]
struct SearchUsersResponse {
    users: Vec<UserSearchResult>,
}

#[derive(Debug, serde::Serialize)]
struct UserSearchResult {
    username: String,
    avatar_url: Option<String>,
}

async fn update_me(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<Json<UserPublic>> {
    let user_service = UserService::new(&state.db);
    user_service.update_profile(&user.id, req).await.map(Json)
}

async fn get_user_profile(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
    Query(_query): Query<PaginationQuery>,
) -> Result<Json<UserProfile>> {
    let user_service = UserService::new(&state.db);
    user_service.get_profile(&username, None).await.map(Json)
}

async fn get_user_posts(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<crate::models::post::PostListResponse>> {
    let user_service = UserService::new(&state.db);
    let post_service = crate::services::PostService::new(&state.db);

    let user = user_service.get_user_by_username(&username).await?;
    post_service.list_user_posts(&user.id, query.page(), query.limit()).await.map(Json)
}

async fn get_followers(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<UserListResponse>> {
    let user_service = UserService::new(&state.db);
    let follow_service = crate::services::FollowService::new(&state.db);

    let user = user_service.get_user_by_username(&username).await?;
    follow_service.get_followers(&user.id, query.page(), query.limit()).await.map(Json)
}

async fn get_following(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<UserListResponse>> {
    let user_service = UserService::new(&state.db);
    let follow_service = crate::services::FollowService::new(&state.db);

    let user = user_service.get_user_by_username(&username).await?;
    follow_service.get_following(&user.id, query.page(), query.limit()).await.map(Json)
}
