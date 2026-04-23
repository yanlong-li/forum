use axum::{Router, Json, routing::{post, delete}, extract::{Path, State}};
use std::sync::Arc;

use crate::models::AppState;
use crate::extractors::AuthenticatedUser;
use crate::services::VoteService;
use crate::models::vote::{VoteRequest, VoteResponse};
use crate::error::Result;

pub fn vote_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_vote))
        .route("/{post_id}", delete(delete_vote))
}

async fn create_vote(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<VoteRequest>,
) -> Result<Json<VoteResponse>> {
    let vote_service = VoteService::new(&state.db);
    vote_service.vote(&user.id, req).await.map(Json)
}

async fn delete_vote(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(post_id): Path<String>,
) -> Result<Json<VoteResponse>> {
    let vote_service = VoteService::new(&state.db);
    vote_service.unvote(&user.id, &post_id).await.map(Json)
}
