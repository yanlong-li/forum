use sqlx::SqlitePool;
use crate::error::Result;
use crate::models::notification::NotificationListResponse;
use crate::repositories::NotificationRepository;

pub struct NotificationService<'a> {
    pool: &'a SqlitePool,
}

impl<'a> NotificationService<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn list_notifications(&self, user_id: &str, page: i64, limit: i64, unread_only: bool) -> Result<NotificationListResponse> {
        let notification_repo = NotificationRepository::new(self.pool);
        let (notifications, total, unread_count) = notification_repo.list(user_id, page, limit, unread_only).await?;

        Ok(NotificationListResponse {
            notifications,
            unread_count,
            total,
            page,
            limit,
        })
    }

    pub async fn mark_read(&self, _user_id: &str, notification_id: &str) -> Result<()> {
        let notification_repo = NotificationRepository::new(self.pool);
        notification_repo.mark_read(notification_id).await
    }

    pub async fn mark_all_read(&self, user_id: &str) -> Result<()> {
        let notification_repo = NotificationRepository::new(self.pool);
        notification_repo.mark_all_read(user_id).await
    }
}
