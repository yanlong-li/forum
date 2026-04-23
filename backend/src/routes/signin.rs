use axum::{Router, Json, routing::{get, post}, extract::State};
use std::sync::Arc;

use crate::models::AppState;
use crate::extractors::AuthenticatedUser;
use crate::services::SigninService;
use crate::error::Result;

pub fn signin_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/checkin", post(signin))
        .route("/status", get(get_signin_status))
}

async fn signin(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<Json<crate::services::signin_service::SigninResponse>> {
    let signin_service = SigninService::new(&state.db);
    signin_service.signin(&user.id).await.map(Json)
}

async fn get_signin_status(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<Json<crate::services::signin_service::SigninStatus>> {
    let signin_service = SigninService::new(&state.db);
    signin_service.get_signin_status(&user.id).await.map(Json)
}
