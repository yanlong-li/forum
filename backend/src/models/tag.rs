use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub post_count: i64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TagListResponse {
    pub tags: Vec<Tag>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}
