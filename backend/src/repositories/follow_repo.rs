use sqlx::SqlitePool;
use crate::error::{AppError, Result};
use crate::models::follow::Follow;

pub struct FollowRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> FollowRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, id: &str, follower_id: &str, following_id: &str) -> Result<Follow> {
        let now = chrono::Utc::now().to_rfc3339();
        let follow = sqlx::query_as::<_, Follow>(
            r#"
            INSERT INTO follows (id, follower_id, following_id, created_at)
            VALUES (?, ?, ?, ?)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(follower_id)
        .bind(following_id)
        .bind(&now)
        .fetch_one(self.pool)
        .await
        .map_err(|_| AppError::Conflict("Already following".to_string()))?;
        Ok(follow)
    }

    pub async fn find(&self, follower_id: &str, following_id: &str) -> Result<Option<Follow>> {
        let follow = sqlx::query_as::<_, Follow>(
            "SELECT * FROM follows WHERE follower_id = ? AND following_id = ?"
        )
        .bind(follower_id)
        .bind(following_id)
        .fetch_optional(self.pool)
        .await?;
        Ok(follow)
    }

    pub async fn delete(&self, follower_id: &str, following_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM follows WHERE follower_id = ? AND following_id = ?")
            .bind(follower_id)
            .bind(following_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_followers(&self, user_id: &str, page: i64, limit: i64) -> Result<(Vec<String>, i64)> {
        let offset = (page - 1) * limit;

        let follower_ids: Vec<String> = sqlx::query_scalar(
            r#"
            SELECT follower_id FROM follows
            WHERE following_id = ?
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool)
        .await?;

        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM follows WHERE following_id = ?"
        )
        .bind(user_id)
        .fetch_one(self.pool)
        .await?;

        Ok((follower_ids, total))
    }

    pub async fn get_following(&self, user_id: &str, page: i64, limit: i64) -> Result<(Vec<String>, i64)> {
        let offset = (page - 1) * limit;

        let following_ids: Vec<String> = sqlx::query_scalar(
            r#"
            SELECT following_id FROM follows
            WHERE follower_id = ?
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool)
        .await?;

        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM follows WHERE follower_id = ?"
        )
        .bind(user_id)
        .fetch_one(self.pool)
        .await?;

        Ok((following_ids, total))
    }
}
