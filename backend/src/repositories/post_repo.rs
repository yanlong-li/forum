use sqlx::SqlitePool;
use crate::error::{AppError, Result};
use crate::models::post::{Post, PostSummary};
use crate::models::user::UserPublic;

pub struct PostRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> PostRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, id: &str, author_id: &str, title: &str, content: &str) -> Result<Post> {
        let now = chrono::Utc::now().to_rfc3339();
        let post = sqlx::query_as::<_, Post>(
            r#"
            INSERT INTO posts (id, author_id, title, content, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(author_id)
        .bind(title)
        .bind(content)
        .bind(&now)
        .bind(&now)
        .fetch_one(self.pool)
        .await
        .map_err(|_| AppError::InternalError("Failed to create post".to_string()))?;
        Ok(post)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Post>> {
        let post = sqlx::query_as::<_, Post>(
            "SELECT * FROM posts WHERE id = ? AND is_deleted = 0"
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?;
        Ok(post)
    }

    pub async fn update(&self, id: &str, title: Option<&str>, content: Option<&str>) -> Result<Post> {
        let now = chrono::Utc::now().to_rfc3339();
        let post = sqlx::query_as::<_, Post>(
            r#"
            UPDATE posts
            SET title = COALESCE(?, title),
                content = COALESCE(?, content),
                updated_at = ?
            WHERE id = ? AND is_deleted = 0
            RETURNING *
            "#
        )
        .bind(title)
        .bind(content)
        .bind(&now)
        .bind(id)
        .fetch_one(self.pool)
        .await
        .map_err(|_| AppError::NotFound("Post not found".to_string()))?;
        Ok(post)
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query("UPDATE posts SET is_deleted = 1, updated_at = ? WHERE id = ?")
            .bind(&now)
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn list(&self, page: i64, limit: i64, tag: Option<&str>, _sort: Option<&str>) -> Result<(Vec<PostSummary>, i64)> {
        let offset = (page - 1) * limit;

        let query = if tag.is_some() {
            r#"
            SELECT p.id, p.author_id, p.title, p.content, p.created_at, p.updated_at,
                   u.id as u_id, u.username, u.avatar_url, u.bio, u.created_at as u_created_at,
                   (SELECT GROUP_CONCAT(t.name) FROM tags t JOIN post_tags pt ON t.id = pt.tag_id WHERE pt.post_id = p.id) as tags,
                   (SELECT COUNT(*) FROM votes WHERE post_id = p.id AND value = 1) as like_count,
                   (SELECT COUNT(*) FROM comments WHERE post_id = p.id AND is_deleted = 0) as comment_count,
                   u.is_admin as u_is_admin
            FROM posts p
            JOIN users u ON p.author_id = u.id
            JOIN post_tags pt ON p.id = pt.post_id
            JOIN tags t ON pt.tag_id = t.id
            WHERE p.is_deleted = 0 AND t.name = ?
            ORDER BY p.created_at DESC
            LIMIT ? OFFSET ?
            "#
        } else {
            r#"
            SELECT p.id, p.author_id, p.title, p.content, p.created_at, p.updated_at,
                   u.id as u_id, u.username, u.avatar_url, u.bio, u.created_at as u_created_at,
                   (SELECT GROUP_CONCAT(t.name) FROM tags t JOIN post_tags pt ON t.id = pt.tag_id WHERE pt.post_id = p.id) as tags,
                   (SELECT COUNT(*) FROM votes WHERE post_id = p.id AND value = 1) as like_count,
                   (SELECT COUNT(*) FROM comments WHERE post_id = p.id AND is_deleted = 0) as comment_count,
                   u.is_admin as u_is_admin
            FROM posts p
            JOIN users u ON p.author_id = u.id
            WHERE p.is_deleted = 0
            ORDER BY p.created_at DESC
            LIMIT ? OFFSET ?
            "#
        };

        let posts: Vec<(String, String, String, String, String, String, String, String, Option<String>, Option<String>, String, Option<String>, i64, i64, bool)> = if let Some(tag) = tag {
            sqlx::query_as(query)
                .bind(tag)
                .bind(limit)
                .bind(offset)
                .fetch_all(self.pool)
                .await?
        } else {
            sqlx::query_as(query)
                .bind(limit)
                .bind(offset)
                .fetch_all(self.pool)
                .await?
        };

        let total: i64 = if let Some(tag) = tag {
            sqlx::query_scalar(
                "SELECT COUNT(DISTINCT p.id) FROM posts p JOIN post_tags pt ON p.id = pt.post_id JOIN tags t ON pt.tag_id = t.id WHERE p.is_deleted = 0 AND t.name = ?"
            )
            .bind(tag)
            .fetch_one(self.pool)
            .await?
        } else {
            sqlx::query_scalar("SELECT COUNT(*) FROM posts WHERE is_deleted = 0")
                .fetch_one(self.pool)
                .await?
        };

        let summaries: Vec<PostSummary> = posts.into_iter().map(|p| {
            let tags: Vec<String> = p.11.map(|t| t.split(',').map(String::from).collect()).unwrap_or_default();
            PostSummary {
                id: p.0,
                author_id: p.1,
                author: UserPublic {
                    id: p.6,
                    username: p.7,
                    avatar_url: p.8,
                    bio: p.9,
                    is_admin: p.14,
                    created_at: p.10,
                },
                title: p.2,
                content: p.3,
                tags,
                like_count: p.12,
                comment_count: p.13,
                is_bookmarked: false,
                created_at: p.4,
            }
        }).collect();

        Ok((summaries, total))
    }

    pub async fn list_by_author(&self, author_id: &str, page: i64, limit: i64) -> Result<(Vec<PostSummary>, i64)> {
        let offset = (page - 1) * limit;

        let posts: Vec<(String, String, String, String, String, String, String, String, Option<String>, Option<String>, String, Option<String>, i64, i64, bool)> = sqlx::query_as(
            r#"
            SELECT p.id, p.author_id, p.title, p.content, p.created_at, p.updated_at,
                   u.id as u_id, u.username, u.avatar_url, u.bio, u.created_at as u_created_at,
                   (SELECT GROUP_CONCAT(t.name) FROM tags t JOIN post_tags pt ON t.id = pt.tag_id WHERE pt.post_id = p.id) as tags,
                   (SELECT COUNT(*) FROM votes WHERE post_id = p.id AND value = 1) as like_count,
                   (SELECT COUNT(*) FROM comments WHERE post_id = p.id AND is_deleted = 0) as comment_count,
                   u.is_admin as u_is_admin
            FROM posts p
            JOIN users u ON p.author_id = u.id
            WHERE p.author_id = ? AND p.is_deleted = 0
            ORDER BY p.created_at DESC
            LIMIT ? OFFSET ?
            "#
        )
        .bind(author_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool)
        .await?;

        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM posts WHERE author_id = ? AND is_deleted = 0")
            .bind(author_id)
            .fetch_one(self.pool)
            .await?;

        let summaries: Vec<PostSummary> = posts.into_iter().map(|p| {
            let tags: Vec<String> = p.11.map(|t| t.split(',').map(String::from).collect()).unwrap_or_default();
            PostSummary {
                id: p.0,
                author_id: p.1,
                author: UserPublic {
                    id: p.6,
                    username: p.7,
                    avatar_url: p.8,
                    bio: p.9,
                    is_admin: p.14,
                    created_at: p.10,
                },
                title: p.2,
                content: p.3,
                tags,
                like_count: p.12,
                comment_count: p.13,
                is_bookmarked: false,
                created_at: p.4,
            }
        }).collect();

        Ok((summaries, total))
    }

    pub async fn add_tag(&self, post_id: &str, tag_id: &str) -> Result<()> {
        sqlx::query("INSERT INTO post_tags (post_id, tag_id) VALUES (?, ?)")
            .bind(post_id)
            .bind(tag_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn remove_tags(&self, post_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM post_tags WHERE post_id = ?")
            .bind(post_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
