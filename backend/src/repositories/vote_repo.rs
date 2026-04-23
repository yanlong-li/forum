use sqlx::SqlitePool;
use crate::error::{AppError, Result};
use crate::models::vote::Vote;

pub struct VoteRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> VoteRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, id: &str, user_id: &str, post_id: Option<&str>, comment_id: Option<&str>, value: i16) -> Result<Vote> {
        let now = chrono::Utc::now().to_rfc3339();
        let vote = sqlx::query_as::<_, Vote>(
            r#"
            INSERT INTO votes (id, user_id, post_id, comment_id, value, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(user_id)
        .bind(post_id)
        .bind(comment_id)
        .bind(value)
        .bind(&now)
        .fetch_one(self.pool)
        .await
        .map_err(|_| AppError::InternalError("Failed to create vote".to_string()))?;
        Ok(vote)
    }

    pub async fn find_user_post_vote(&self, user_id: &str, post_id: &str) -> Result<Option<Vote>> {
        let vote = sqlx::query_as::<_, Vote>(
            "SELECT * FROM votes WHERE user_id = ? AND post_id = ?"
        )
        .bind(user_id)
        .bind(post_id)
        .fetch_optional(self.pool)
        .await?;
        Ok(vote)
    }

    pub async fn find_user_comment_vote(&self, user_id: &str, comment_id: &str) -> Result<Option<Vote>> {
        let vote = sqlx::query_as::<_, Vote>(
            "SELECT * FROM votes WHERE user_id = ? AND comment_id = ?"
        )
        .bind(user_id)
        .bind(comment_id)
        .fetch_optional(self.pool)
        .await?;
        Ok(vote)
    }

    pub async fn update(&self, id: &str, value: i16) -> Result<Vote> {
        let vote = sqlx::query_as::<_, Vote>(
            "UPDATE votes SET value = ? WHERE id = ? RETURNING *"
        )
        .bind(value)
        .bind(id)
        .fetch_one(self.pool)
        .await
        .map_err(|_| AppError::InternalError("Failed to update vote".to_string()))?;
        Ok(vote)
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM votes WHERE id = ?")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_by_post(&self, user_id: &str, post_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM votes WHERE user_id = ? AND post_id = ?")
            .bind(user_id)
            .bind(post_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_by_comment(&self, user_id: &str, comment_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM votes WHERE user_id = ? AND comment_id = ?")
            .bind(user_id)
            .bind(comment_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_post_counts(&self, post_id: &str) -> Result<(i64, i64)> {
        let like_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM votes WHERE post_id = ? AND value = 1"
        )
        .bind(post_id)
        .fetch_one(self.pool)
        .await?;

        let dislike_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM votes WHERE post_id = ? AND value = -1"
        )
        .bind(post_id)
        .fetch_one(self.pool)
        .await?;

        Ok((like_count, dislike_count))
    }

    pub async fn get_comment_counts(&self, comment_id: &str) -> Result<(i64, i64)> {
        let like_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM votes WHERE comment_id = ? AND value = 1"
        )
        .bind(comment_id)
        .fetch_one(self.pool)
        .await?;

        let dislike_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM votes WHERE comment_id = ? AND value = -1"
        )
        .bind(comment_id)
        .fetch_one(self.pool)
        .await?;

        Ok((like_count, dislike_count))
    }
}
