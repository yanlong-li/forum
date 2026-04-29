use axum::{Router, Json, routing::{get, patch}, extract::{Path, Query, State}};
use std::sync::Arc;

use crate::models::AppState;
use crate::extractors::AuthenticatedUser;
use crate::services::AdminService;
use crate::models::response::PaginationQuery;
use crate::models::follow::UserListResponse;
use crate::error::{AppError, Result};

pub fn admin_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/stats", get(get_stats))
        .route("/users", get(list_users))
        .route("/users/{id}/status", patch(update_user_status))
        .route("/reports", get(list_reports))
        .route("/reports/{id}", patch(process_report))
        .route("/posts/{id}/pin", patch(set_pin))
        .route("/posts/{id}/feature", patch(set_featured))
}

async fn get_stats(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<Json<crate::models::response::StatsResponse>> {
    if !user.is_admin {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let admin_service = AdminService::new(&state.db);
    admin_service.get_stats().await.map(Json)
}

async fn list_users(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<UserListResponse>> {
    if !user.is_admin {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let user_service = crate::services::UserService::new(&state.db);
    user_service.list_users(query.page(), query.limit(), None).await.map(Json)
}

async fn update_user_status(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<String>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<crate::models::user::UserPublic>> {
    if !user.is_admin {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let user_service = crate::services::UserService::new(&state.db);

    if req["is_locked"].as_bool() == Some(true) {
        let locked_until = chrono::Utc::now() + chrono::Duration::minutes(30);
        user_service.lock_user(&id, &locked_until.to_rfc3339()).await?;
    } else {
        user_service.unlock_user(&id).await?;
    }

    user_service.get_user_by_id(&id).await.map(Json)
}

async fn list_reports(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<crate::models::report::ReportListResponse>> {
    if !user.is_admin {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let admin_service = AdminService::new(&state.db);
    admin_service.list_reports(query.page(), query.limit(), None).await.map(Json)
}

async fn process_report(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<String>,
    Json(req): Json<crate::models::report::ProcessReportRequest>,
) -> Result<Json<crate::models::response::MessageResponse>> {
    if !user.is_admin {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let admin_service = AdminService::new(&state.db);
    admin_service.process_report(&id, &user.id, req).await?;

    Ok(Json(crate::models::response::MessageResponse {
        message: "Report processed".to_string()
    }))
}

async fn set_pin(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(post_id): Path<String>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<crate::models::response::MessageResponse>> {
    if !user.is_admin {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let is_pinned = req["is_pinned"].as_bool().unwrap_or(false);
    let post_service = crate::services::PostService::new(&state.db);
    post_service.set_pin_featured(&post_id, is_pinned, false, true).await?;

    Ok(Json(crate::models::response::MessageResponse {
        message: if is_pinned { "Post pinned" } else { "Post unpinned" }.to_string()
    }))
}

async fn set_featured(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(post_id): Path<String>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<crate::models::response::MessageResponse>> {
    if !user.is_admin {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let is_featured = req["is_featured"].as_bool().unwrap_or(false);
    let post_service = crate::services::PostService::new(&state.db);
    post_service.set_pin_featured(&post_id, false, is_featured, true).await?;

    Ok(Json(crate::models::response::MessageResponse {
        message: if is_featured { "Post featured" } else { "Post unfeatured" }.to_string()
    }))
}
