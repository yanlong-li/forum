use sqlx::SqlitePool;
use crate::error::{AppError, Result};
use crate::models::comment::{Comment, CommentWithAuthor};
use crate::models::user::UserPublic;

pub struct CommentRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> CommentRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, id: &str, post_id: &str, author_id: &str, parent_id: Option<&str>, content: &str) -> Result<Comment> {
        let now = chrono::Utc::now().to_rfc3339();
        let comment = sqlx::query_as::<_, Comment>(
            r#"
            INSERT INTO comments (id, post_id, author_id, parent_id, content, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(post_id)
        .bind(author_id)
        .bind(parent_id)
        .bind(content)
        .bind(&now)
        .bind(&now)
        .fetch_one(self.pool)
        .await
        .map_err(|_| AppError::InternalError("Failed to create comment".to_string()))?;
        Ok(comment)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Comment>> {
        let comment = sqlx::query_as::<_, Comment>(
            "SELECT * FROM comments WHERE id = ? AND is_deleted = 0"
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?;
        Ok(comment)
    }

    pub async fn update(&self, id: &str, content: &str) -> Result<Comment> {
        let now = chrono::Utc::now().to_rfc3339();
        let comment = sqlx::query_as::<_, Comment>(
            r#"
            UPDATE comments SET content = ?, updated_at = ?
            WHERE id = ? AND is_deleted = 0
            RETURNING *
            "#
        )
        .bind(content)
        .bind(&now)
        .bind(id)
        .fetch_one(self.pool)
        .await
        .map_err(|_| AppError::NotFound("Comment not found".to_string()))?;
        Ok(comment)
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query("UPDATE comments SET is_deleted = 1, updated_at = ? WHERE id = ?")
            .bind(&now)
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn accept(&self, id: &str) -> Result<()> {
        sqlx::query("UPDATE comments SET is_accepted = 1 WHERE id = ?")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn unaccept_all_for_post(&self, post_id: &str) -> Result<()> {
        sqlx::query("UPDATE comments SET is_accepted = 0 WHERE post_id = ? AND is_accepted = 1")
            .bind(post_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn list_by_post(&self, post_id: &str, page: i64, limit: i64) -> Result<(Vec<CommentWithAuthor>, i64)> {
        let offset = (page - 1) * limit;

        let comments: Vec<(String, String, String, Option<String>, String, bool, String, String, String, String, Option<String>, Option<String>, String, i64, i64, bool)> = sqlx::query_as(
            r#"
            SELECT c.id, c.post_id, c.author_id, c.parent_id, c.content, c.is_deleted, c.created_at, c.updated_at,
                   u.id as u_id, u.username, u.avatar_url, u.bio, u.created_at as u_created_at,
                   (SELECT COUNT(*) FROM votes WHERE comment_id = c.id AND value = 1) as like_count,
                   (SELECT COUNT(*) FROM comments WHERE parent_id = c.id AND is_deleted = 0) as reply_count,
                   u.is_admin as u_is_admin
            FROM comments c
            JOIN users u ON c.author_id = u.id
            WHERE c.post_id = ? AND c.parent_id IS NULL AND c.is_deleted = 0
            ORDER BY c.created_at ASC
            LIMIT ? OFFSET ?
            "#
        )
        .bind(post_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool)
        .await?;

        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM comments WHERE post_id = ? AND parent_id IS NULL AND is_deleted = 0"
        )
        .bind(post_id)
        .fetch_one(self.pool)
        .await?;

        let result: Vec<CommentWithAuthor> = comments.into_iter().map(|c| {
            CommentWithAuthor {
                id: c.0,
                post_id: c.1,
                author_id: c.2,
                author: UserPublic {
                    id: c.8,
                    username: c.9,
                    avatar_url: c.10,
                    bio: c.11,
                    is_admin: c.15,
                    points: 0,
                    level: 1,
                    created_at: c.12,
                },
                parent_id: c.3,
                content: c.4,
                is_deleted: c.5,
                like_count: c.13,
                is_liked: false,
                reply_count: c.14,
                created_at: c.6,
                updated_at: c.7,
            }
        }).collect();

        Ok((result, total))
    }

    pub async fn list_replies(&self, parent_id: &str) -> Result<Vec<CommentWithAuthor>> {
        let comments: Vec<(String, String, String, Option<String>, String, bool, String, String, String, String, Option<String>, Option<String>, String, i64, i64, bool)> = sqlx::query_as(
            r#"
            SELECT c.id, c.post_id, c.author_id, c.parent_id, c.content, c.is_deleted, c.created_at, c.updated_at,
                   u.id as u_id, u.username, u.avatar_url, u.bio, u.created_at as u_created_at,
                   (SELECT COUNT(*) FROM votes WHERE comment_id = c.id AND value = 1) as like_count,
                   (SELECT COUNT(*) FROM comments WHERE parent_id = c.id AND is_deleted = 0) as reply_count,
                   u.is_admin as u_is_admin
            FROM comments c
            JOIN users u ON c.author_id = u.id
            WHERE c.parent_id = ? AND c.is_deleted = 0
            ORDER BY c.created_at ASC
            "#
        )
        .bind(parent_id)
        .fetch_all(self.pool)
        .await?;

        let result: Vec<CommentWithAuthor> = comments.into_iter().map(|c| {
            CommentWithAuthor {
                id: c.0,
                post_id: c.1,
                author_id: c.2,
                author: UserPublic {
                    id: c.8,
                    username: c.9,
                    avatar_url: c.10,
                    bio: c.11,
                    is_admin: c.15,
                    points: 0,
                    level: 1,
                    created_at: c.12,
                },
                parent_id: c.3,
                content: c.4,
                is_deleted: c.5,
                like_count: c.13,
                is_liked: false,
                reply_count: c.14,
                created_at: c.6,
                updated_at: c.7,
            }
        }).collect();

        Ok(result)
    }

    pub async fn list_by_author(&self, author_id: &str, page: i64, limit: i64) -> Result<(Vec<CommentWithAuthor>, i64)> {
        let offset = (page - 1) * limit;

        let comments: Vec<(String, String, String, Option<String>, String, bool, String, String, String, String, Option<String>, Option<String>, String, i64, i64, bool)> = sqlx::query_as(
            r#"
            SELECT c.id, c.post_id, c.author_id, c.parent_id, c.content, c.is_deleted, c.created_at, c.updated_at,
                   u.id as u_id, u.username, u.avatar_url, u.bio, u.created_at as u_created_at,
                   (SELECT COUNT(*) FROM votes WHERE comment_id = c.id AND value = 1) as like_count,
                   (SELECT COUNT(*) FROM comments WHERE parent_id = c.id AND is_deleted = 0) as reply_count,
                   u.is_admin as u_is_admin
            FROM comments c
            JOIN users u ON c.author_id = u.id
            WHERE c.author_id = ? AND c.is_deleted = 0
            ORDER BY c.created_at DESC
            LIMIT ? OFFSET ?
            "#
        )
        .bind(author_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool)
        .await?;

        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM comments WHERE author_id = ? AND is_deleted = 0"
        )
        .bind(author_id)
        .fetch_one(self.pool)
        .await?;

        let result: Vec<CommentWithAuthor> = comments.into_iter().map(|c| {
            CommentWithAuthor {
                id: c.0,
                post_id: c.1,
                author_id: c.2,
                author: UserPublic {
                    id: c.8,
                    username: c.9,
                    avatar_url: c.10,
                    bio: c.11,
                    is_admin: c.15,
                    points: 0,
                    level: 1,
                    created_at: c.12,
                },
                parent_id: c.3,
                content: c.4,
                is_deleted: c.5,
                like_count: c.13,
                is_liked: false,
                reply_count: c.14,
                created_at: c.6,
                updated_at: c.7,
            }
        }).collect();

        Ok((result, total))
    }
}
