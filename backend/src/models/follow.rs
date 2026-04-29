use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::user::UserPublic;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Follow {
    pub id: String,
    pub follower_id: String,
    pub following_id: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct FollowResponse {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListResponse {
    pub users: Vec<UserPublic>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}
