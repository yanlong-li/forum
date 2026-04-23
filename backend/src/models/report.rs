use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Report {
    pub id: String,
    pub reporter_id: String,
    pub post_id: Option<String>,
    pub comment_id: Option<String>,
    pub reason: String,
    pub status: String,
    pub created_at: String,
    pub processed_at: Option<String>,
    pub processed_by: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateReportRequest {
    pub post_id: Option<String>,
    pub comment_id: Option<String>,
    pub reason: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProcessReportRequest {
    pub status: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReportWithDetails {
    pub id: String,
    pub reporter: crate::models::user::UserPublic,
    pub post_id: Option<String>,
    pub comment_id: Option<String>,
    pub reason: String,
    pub status: String,
    pub created_at: String,
    pub processed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReportListResponse {
    pub reports: Vec<ReportWithDetails>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}
