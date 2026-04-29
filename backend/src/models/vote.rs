use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Vote {
    pub id: String,
    pub user_id: String,
    pub post_id: Option<String>,
    pub comment_id: Option<String>,
    pub value: i16,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VoteRequest {
    pub post_id: Option<String>,
    pub comment_id: Option<String>,
    pub value: i16,
}

#[derive(Debug, Clone, Serialize)]
pub struct VoteResponse {
    pub post_id: Option<String>,
    pub comment_id: Option<String>,
    pub like_count: i64,
    pub dislike_count: i64,
}
