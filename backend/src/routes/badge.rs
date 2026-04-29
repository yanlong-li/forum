use axum::{Router, Json, routing::{get, post}, extract::Path, extract::State};
use std::sync::Arc;

use crate::models::AppState;
use crate::extractors::AuthenticatedUser;
use crate::services::BadgeService;
use crate::error::Result;

pub fn badge_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_badges))
        .route("/user/{user_id}", get(get_user_badges))
        .route("/check", post(check_and_award_badges))
}

async fn get_all_badges(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<crate::services::badge_service::Badge>>> {
    let badge_service = BadgeService::new(&state.db);
    badge_service.get_all_badges().await.map(Json)
}

async fn get_user_badges(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
) -> Result<Json<Vec<crate::services::badge_service::UserBadgeWithDetails>>> {
    let badge_service = BadgeService::new(&state.db);
    badge_service.get_user_badges(&user_id).await.map(Json)
}

async fn check_and_award_badges(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<Json<Vec<crate::services::badge_service::Badge>>> {
    let badge_service = BadgeService::new(&state.db);
    badge_service.check_and_award_badges(&user.id).await.map(Json)
}