use sqlx::SqlitePool;
use crate::error::{AppError, Result};
use crate::models::report::{ProcessReportRequest, ReportListResponse};
use crate::models::response::StatsResponse;
use crate::repositories::ReportRepository;

pub struct AdminService<'a> {
    pool: &'a SqlitePool,
}

impl<'a> AdminService<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_stats(&self) -> Result<StatsResponse> {
        let daily_active_users: i64 = sqlx::query_scalar(
            "SELECT COUNT(DISTINCT user_id) FROM (SELECT author_id as user_id FROM posts WHERE created_at > datetime('now', '-1 day') UNION SELECT author_id as user_id FROM comments WHERE created_at > datetime('now', '-1 day'))"
        )
        .fetch_one(self.pool)
        .await?;

        let total_posts: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM posts WHERE is_deleted = 0")
            .fetch_one(self.pool)
            .await?;

        let total_comments: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM comments WHERE is_deleted = 0")
            .fetch_one(self.pool)
            .await?;

        let total_users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(self.pool)
            .await?;

        Ok(StatsResponse {
            daily_active_users,
            total_posts,
            total_comments,
            total_users,
        })
    }

    pub async fn process_report(&self, report_id: &str, admin_id: &str, req: ProcessReportRequest) -> Result<()> {
        let report_repo = ReportRepository::new(self.pool);

        let report = report_repo.find_by_id(report_id).await?
            .ok_or_else(|| AppError::NotFound("Report not found".to_string()))?;

        if report.status != "pending" {
            return Err(AppError::Conflict("Report already processed".to_string()));
        }

        let new_status = match req.action.as_str() {
            "dismiss" => "dismissed",
            "delete_content" => "resolved",
            _ => return Err(AppError::ValidationError("Invalid action".to_string())),
        };

        report_repo.update_status(report_id, new_status, admin_id).await
    }

    pub async fn list_reports(&self, page: i64, limit: i64, status: Option<&str>) -> Result<ReportListResponse> {
        let report_repo = ReportRepository::new(self.pool);
        let (reports, total) = report_repo.list(page, limit, status).await?;

        Ok(ReportListResponse {
            reports,
            total,
            page,
            limit,
        })
    }
}
