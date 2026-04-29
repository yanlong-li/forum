use axum::{Router, Json, routing::{get, post}, extract::{Path, State}};
use std::sync::Arc;
use uuid::Uuid;

use crate::models::AppState;
use crate::extractors::AuthenticatedUser;
use crate::error::{AppError, Result};
use crate::models::report::CreateReportRequest;

pub fn report_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_report))
        .route("/{id}", get(get_report))
}

async fn create_report(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreateReportRequest>,
) -> Result<Json<crate::models::response::MessageResponse>> {
    if req.post_id.is_none() && req.comment_id.is_none() {
        return Err(AppError::ValidationError("Either post_id or comment_id is required".to_string()));
    }

    if req.reason.trim().is_empty() {
        return Err(AppError::ValidationError("Reason is required".to_string()));
    }

    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO reports (id, reporter_id, post_id, comment_id, reason, status, created_at) VALUES (?, ?, ?, ?, ?, 'pending', ?)"
    )
    .bind(&id)
    .bind(&user.id)
    .bind(&req.post_id)
    .bind(&req.comment_id)
    .bind(&req.reason)
    .bind(&now)
    .execute(&state.db)
    .await
    .map_err(|_| AppError::InternalError("Failed to create report".to_string()))?;

    Ok(Json(crate::models::response::MessageResponse {
        message: "Report submitted".to_string()
    }))
}

async fn get_report(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::models::report::Report>> {
    let report = sqlx::query_as::<_, crate::models::report::Report>(
        "SELECT * FROM reports WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Report not found".to_string()))?;

    Ok(Json(report))
}