use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::user::UserPublic;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: String,
    pub post_id: String,
    pub author_id: String,
    pub parent_id: Option<String>,
    pub content: String,
    pub is_deleted: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentWithAuthor {
    pub id: String,
    pub post_id: String,
    pub author_id: String,
    pub author: UserPublic,
    pub parent_id: Option<String>,
    pub content: String,
    pub is_deleted: bool,
    pub like_count: i64,
    #[serde(default)]
    pub is_liked: bool,
    pub reply_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCommentRequest {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentListResponse {
    pub comments: Vec<CommentWithAuthor>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}
