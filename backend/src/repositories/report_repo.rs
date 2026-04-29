use sqlx::SqlitePool;
use crate::error::{AppError, Result};
use crate::models::report::{Report, ReportWithDetails};
use crate::models::user::UserPublic;

pub struct ReportRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> ReportRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, id: &str, reporter_id: &str, post_id: Option<&str>, comment_id: Option<&str>, reason: &str) -> Result<Report> {
        let now = chrono::Utc::now().to_rfc3339();
        let report = sqlx::query_as::<_, Report>(
            r#"
            INSERT INTO reports (id, reporter_id, post_id, comment_id, reason, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(reporter_id)
        .bind(post_id)
        .bind(comment_id)
        .bind(reason)
        .bind(&now)
        .fetch_one(self.pool)
        .await
        .map_err(|_| AppError::InternalError("Failed to create report".to_string()))?;
        Ok(report)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Report>> {
        let report = sqlx::query_as::<_, Report>(
            "SELECT * FROM reports WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?;
        Ok(report)
    }

    pub async fn list(&self, page: i64, limit: i64, status: Option<&str>) -> Result<(Vec<ReportWithDetails>, i64)> {
        let offset = (page - 1) * limit;

        let where_clause = if let Some(status) = status {
            format!("r.status = '{}'", status)
        } else {
            "1=1".to_string()
        };

        let reports: Vec<(String, String, Option<String>, Option<String>, String, String, String, Option<String>, Option<String>, String, String, Option<String>, Option<String>, String, bool, Option<String>)> = sqlx::query_as(&format!(
            r#"
            SELECT r.id, r.reporter_id, r.post_id, r.comment_id, r.reason, r.status, r.created_at, r.processed_at, r.processed_by,
                   u.id as u_id, u.username, u.avatar_url, u.bio, u.created_at as u_created_at, u.is_admin as u_is_admin,
                   reported.username as rep_username
            FROM reports r
            JOIN users u ON r.reporter_id = u.id
            LEFT JOIN posts p ON r.post_id = p.id
            LEFT JOIN comments c ON r.comment_id = c.id
            LEFT JOIN users reported ON COALESCE(p.author_id, c.author_id) = reported.id
            WHERE {}
            ORDER BY r.created_at DESC
            LIMIT ? OFFSET ?
            "#, where_clause
        ))
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool)
        .await?;

        let total: i64 = sqlx::query_scalar(&format!(
            "SELECT COUNT(*) FROM reports WHERE {}",
            where_clause
        ))
        .fetch_one(self.pool)
        .await?;

        let result: Vec<ReportWithDetails> = reports.into_iter().map(|r| {
            ReportWithDetails {
                id: r.0.clone(),
                reporter: UserPublic {
                    id: r.9.clone(),
                    username: r.10.clone(),
                    avatar_url: r.11.clone(),
                    bio: r.12.clone(),
                    is_admin: r.14,
                    points: 0,
                    level: 1,
                    created_at: r.13.clone(),
                },
                reporter_username: r.10.clone(),
                reported_user_id: None,
                reported_username: r.15,
                post_id: r.2.clone(),
                comment_id: r.3.clone(),
                reason: r.4.clone(),
                status: r.5.clone(),
                created_at: r.6.clone(),
                processed_at: r.7.clone(),
            }
        }).collect();

        Ok((result, total))
    }

    pub async fn update_status(&self, id: &str, status: &str, processed_by: &str) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query("UPDATE reports SET status = ?, processed_at = ?, processed_by = ? WHERE id = ?")
            .bind(status)
            .bind(&now)
            .bind(processed_by)
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
