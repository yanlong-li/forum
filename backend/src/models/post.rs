use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::user::UserPublic;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: String,
    pub author_id: String,
    pub title: String,
    pub content: String,
    pub is_deleted: bool,
    pub is_pinned: bool,
    pub is_featured: bool,
    pub view_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostWithAuthor {
    pub id: String,
    pub author_id: String,
    pub author: UserPublic,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub like_count: i64,
    pub dislike_count: i64,
    pub comment_count: i64,
    #[serde(default)]
    pub is_bookmarked: bool,
    #[serde(default)]
    pub is_liked: bool,
    #[serde(default)]
    pub is_pinned: bool,
    #[serde(default)]
    pub is_featured: bool,
    #[serde(default)]
    pub view_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostSummary {
    pub id: String,
    pub author_id: String,
    pub author: UserPublic,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub like_count: i64,
    pub comment_count: i64,
    #[serde(default)]
    pub is_bookmarked: bool,
    #[serde(default)]
    pub is_pinned: bool,
    #[serde(default)]
    pub is_featured: bool,
    #[serde(default)]
    pub view_count: i64,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostListResponse {
    pub posts: Vec<PostSummary>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}
