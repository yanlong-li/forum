use axum::{Router, Json, routing::post, extract::State};
use std::sync::Arc;

use crate::models::AppState;
use crate::services::AuthService;
use crate::models::user::{RegisterRequest, LoginRequest, AuthResponse};
use crate::models::response::MessageResponse;
use crate::error::Result;

pub fn auth_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh_token))
}

async fn register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<MessageResponse>> {
    let auth_service = AuthService::new(&state.db, &state.config);
    auth_service.register(req).await.map(Json)
}

async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>> {
    let auth_service = AuthService::new(&state.db, &state.config);
    auth_service.login(req).await.map(Json)
}

async fn refresh_token(
    State(state): State<Arc<AppState>>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<AuthResponse>> {
    let refresh_token = req["refresh_token"].as_str()
        .ok_or_else(|| crate::error::AppError::ValidationError("refresh_token is required".to_string()))?;

    let auth_service = AuthService::new(&state.db, &state.config);
    auth_service.refresh_token(refresh_token).await.map(Json)
}
