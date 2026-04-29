use axum::{Router, Json, routing::{post}, extract::{Path, State}};
use std::sync::Arc;

use crate::models::AppState;
use crate::extractors::AuthenticatedUser;
use crate::services::FollowService;
use crate::models::follow::FollowResponse;
use crate::error::Result;

pub fn follow_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/{user_id}", post(follow_user).delete(unfollow_user))
}

async fn follow_user(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(user_id): Path<String>,
) -> Result<Json<FollowResponse>> {
    let follow_service = FollowService::new(&state.db);
    follow_service.follow(&user.id, &user_id).await.map(Json)
}

async fn unfollow_user(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(user_id): Path<String>,
) -> Result<()> {
    let follow_service = FollowService::new(&state.db);
    follow_service.unfollow(&user.id, &user_id).await
}
