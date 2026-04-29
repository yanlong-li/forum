use sqlx::SqlitePool;
use crate::error::{AppError, Result};
use crate::models::notification::{Notification, NotificationWithActor};
use crate::models::user::UserPublic;

pub struct NotificationRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> NotificationRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, id: &str, user_id: &str, notification_type: &str, actor_id: &str, post_id: Option<&str>, comment_id: Option<&str>) -> Result<Notification> {
        let now = chrono::Utc::now().to_rfc3339();
        let notification = sqlx::query_as::<_, Notification>(
            r#"
            INSERT INTO notifications (id, user_id, type, actor_id, post_id, comment_id, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(user_id)
        .bind(notification_type)
        .bind(actor_id)
        .bind(post_id)
        .bind(comment_id)
        .bind(&now)
        .fetch_one(self.pool)
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to create notification: {}", e)))?;
        Ok(notification)
    }

    pub async fn list(&self, user_id: &str, page: i64, limit: i64, unread_only: bool) -> Result<(Vec<NotificationWithActor>, i64, i64)> {
        let offset = (page - 1) * limit;

        let where_clause = if unread_only {
            "n.user_id = ? AND n.is_read = 0"
        } else {
            "n.user_id = ?"
        };

        let notifications: Vec<(String, String, String, Option<String>, Option<String>, bool, String, String, String, Option<String>, Option<String>, String, bool)> = sqlx::query_as(&format!(
            r#"
            SELECT n.id, n.type, n.actor_id, n.post_id, n.comment_id, n.is_read, n.created_at,
                   u.id as u_id, u.username, u.avatar_url, u.bio, u.created_at as u_created_at, u.is_admin as u_is_admin
            FROM notifications n
            JOIN users u ON n.actor_id = u.id
            WHERE {}
            ORDER BY n.created_at DESC
            LIMIT ? OFFSET ?
            "#, where_clause
        ))
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool)
        .await?;

        let total: i64 = sqlx::query_scalar(&format!(
            "SELECT COUNT(*) FROM notifications n WHERE {}",
            where_clause.replace("?", "$1")
        ))
        .bind(user_id)
        .fetch_one(self.pool)
        .await?;

        let unread_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM notifications WHERE user_id = ? AND is_read = 0"
        )
        .bind(user_id)
        .fetch_one(self.pool)
        .await?;

        let result: Vec<NotificationWithActor> = notifications.into_iter().map(|n| {
            NotificationWithActor {
                id: n.0,
                notification_type: n.1,
                actor: UserPublic {
                    id: n.7,
                    username: n.8,
                    avatar_url: n.9,
                    bio: n.10,
                    is_admin: n.12,
                    points: 0,
                    level: 1,
                    created_at: n.11,
                },
                post_id: n.3,
                comment_id: n.4,
                is_read: n.5,
                created_at: n.6,
            }
        }).collect();

        Ok((result, total, unread_count))
    }

    pub async fn mark_read(&self, id: &str) -> Result<()> {
        sqlx::query("UPDATE notifications SET is_read = 1 WHERE id = ?")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn mark_all_read(&self, user_id: &str) -> Result<()> {
        sqlx::query("UPDATE notifications SET is_read = 1 WHERE user_id = ?")
            .bind(user_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
