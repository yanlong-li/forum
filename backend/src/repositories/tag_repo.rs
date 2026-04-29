use sqlx::SqlitePool;
use crate::error::{AppError, Result};
use crate::models::tag::Tag;

pub struct TagRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> TagRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, id: &str, name: &str) -> Result<Tag> {
        let now = chrono::Utc::now().to_rfc3339();
        let tag = sqlx::query_as::<_, Tag>(
            r#"
            INSERT INTO tags (id, name, created_at)
            VALUES (?, ?, ?)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(name)
        .bind(&now)
        .fetch_one(self.pool)
        .await
        .map_err(|_| AppError::Conflict("Tag already exists".to_string()))?;
        Ok(tag)
    }

    pub async fn find_by_name(&self, name: &str) -> Result<Option<Tag>> {
        let tag = sqlx::query_as::<_, Tag>(
            "SELECT * FROM tags WHERE name = ?"
        )
        .bind(name)
        .fetch_optional(self.pool)
        .await?;
        Ok(tag)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Tag>> {
        let tag = sqlx::query_as::<_, Tag>(
            "SELECT * FROM tags WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?;
        Ok(tag)
    }

    pub async fn list(&self, page: i64, limit: i64, sort: Option<&str>) -> Result<(Vec<Tag>, i64)> {
        let offset = (page - 1) * limit;

        let order_by = match sort {
            Some("popular") => "post_count DESC",
            _ => "created_at DESC",
        };

        let tags: Vec<Tag> = sqlx::query_as(&format!(
            "SELECT * FROM tags ORDER BY {} LIMIT ? OFFSET ?",
            order_by
        ))
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool)
        .await?;

        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tags")
            .fetch_one(self.pool)
            .await?;

        Ok((tags, total))
    }

    pub async fn increment_post_count(&self, id: &str) -> Result<()> {
        sqlx::query("UPDATE tags SET post_count = post_count + 1 WHERE id = ?")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn decrement_post_count(&self, id: &str) -> Result<()> {
        sqlx::query("UPDATE tags SET post_count = post_count - 1 WHERE id = ? AND post_count > 0")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_or_create(&self, id: &str, name: &str) -> Result<Tag> {
        if let Some(tag) = self.find_by_name(name).await? {
            return Ok(tag);
        }
        self.create(id, name).await
    }
}
