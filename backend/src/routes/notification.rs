use axum::{Router, Json, routing::{get, patch}, extract::{Path, Query, State}};
use std::sync::Arc;

use crate::models::AppState;
use crate::extractors::AuthenticatedUser;
use crate::services::NotificationService;
use crate::models::response::PaginationQuery;
use crate::error::Result;

pub fn notification_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_notifications))
        .route("/read-all", patch(mark_all_read))
        .route("/{id}/read", patch(mark_read))
}

async fn list_notifications(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<crate::models::notification::NotificationListResponse>> {
    let notification_service = NotificationService::new(&state.db);
    notification_service.list_notifications(&user.id, query.page(), query.limit(), false).await.map(Json)
}

async fn mark_read(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<Json<crate::models::notification::NotificationWithActor>> {
    let notification_service = NotificationService::new(&state.db);
    notification_service.mark_read(&user.id, &id).await?;

    let notification_repo = crate::repositories::NotificationRepository::new(&state.db);
    let notifications = notification_repo.list(&user.id, 1, 1, false).await?.0;

    Ok(Json(notifications.into_iter().find(|n| n.id == id)
        .unwrap_or(crate::models::notification::NotificationWithActor {
            id: id.clone(),
            notification_type: "read".to_string(),
            actor: crate::models::user::UserPublic {
                id: user.id.clone(),
                username: user.username.clone(),
                avatar_url: None,
                bio: None,
                is_admin: user.is_admin,
                created_at: chrono::Utc::now().to_rfc3339(),
            },
            post_id: None,
            comment_id: None,
            is_read: true,
            created_at: chrono::Utc::now().to_rfc3339(),
        })))
}

async fn mark_all_read(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<Json<crate::models::response::MessageResponse>> {
    let notification_service = NotificationService::new(&state.db);
    notification_service.mark_all_read(&user.id).await?;

    Ok(Json(crate::models::response::MessageResponse {
        message: "All notifications marked as read".to_string()
    }))
}
