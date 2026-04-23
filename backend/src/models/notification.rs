use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::user::UserPublic;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Notification {
    pub id: String,
    pub user_id: String,
    pub notification_type: String,
    pub actor_id: String,
    pub post_id: Option<String>,
    pub comment_id: Option<String>,
    pub is_read: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct NotificationWithActor {
    pub id: String,
    #[serde(rename = "type")]
    pub notification_type: String,
    pub actor: UserPublic,
    pub post_id: Option<String>,
    pub comment_id: Option<String>,
    pub is_read: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct NotificationListResponse {
    pub notifications: Vec<NotificationWithActor>,
    pub unread_count: i64,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct NotificationMessage {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub payload: NotificationPayload,
}

#[derive(Debug, Clone, Serialize)]
pub struct NotificationPayload {
    pub id: String,
    pub notification_type: String,
    pub actor: UserPublic,
    pub post_id: Option<String>,
    pub post_title: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NewPostMessage {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub payload: NewPostPayload,
}

#[derive(Debug, Clone, Serialize)]
pub struct NewPostPayload {
    pub post_id: String,
    pub title: String,
    pub author: UserPublic,
}
