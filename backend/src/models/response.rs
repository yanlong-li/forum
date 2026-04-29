use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct StatsResponse {
    pub daily_active_users: i64,
    pub total_posts: i64,
    pub total_comments: i64,
    pub total_users: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

impl PaginationQuery {
    pub fn page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(20).min(100).max(1)
    }

    pub fn offset(&self) -> i64 {
        (self.page() - 1) * self.limit()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostListQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub tag: Option<String>,
    pub sort: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
}
