use sqlx::SqlitePool;
use crate::error::{AppError, Result};
use crate::models::post::{Post, PostSummary};
use crate::models::user::UserPublic;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct PostListRow {
    id: String,
    author_id: String,
    title: String,
    content: String,
    created_at: String,
    updated_at: String,
    is_pinned: bool,
    is_featured: bool,
    view_count: i64,
    u_id: String,
    username: String,
    avatar_url: Option<String>,
    bio: Option<String>,
    u_created_at: String,
    points: i64,
    level: i64,
    u_is_admin: bool,
    tags: Option<String>,
    like_count: i64,
    comment_count: i64,
}

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
            SELECT p.id, p.author_id, p.title, p.content, p.created_at, p.updated_at, p.is_pinned, p.is_featured, p.view_count,
                   u.id as u_id, u.username, u.avatar_url, u.bio, u.created_at as u_created_at, u.points, u.level, u.is_admin as u_is_admin,
                   (SELECT GROUP_CONCAT(t.name) FROM tags t JOIN post_tags pt ON t.id = pt.tag_id WHERE pt.post_id = p.id) as tags,
                   (SELECT COUNT(*) FROM votes WHERE post_id = p.id AND value = 1) as like_count,
                   (SELECT COUNT(*) FROM comments WHERE post_id = p.id AND is_deleted = 0) as comment_count
            FROM posts p
            JOIN users u ON p.author_id = u.id
            JOIN post_tags pt ON p.id = pt.post_id
            JOIN tags t ON pt.tag_id = t.id
            WHERE p.is_deleted = 0 AND t.name = ?
            ORDER BY p.is_pinned DESC, p.is_featured DESC, p.created_at DESC
            LIMIT ? OFFSET ?
            "#
        } else {
            r#"
            SELECT p.id, p.author_id, p.title, p.content, p.created_at, p.updated_at, p.is_pinned, p.is_featured, p.view_count,
                   u.id as u_id, u.username, u.avatar_url, u.bio, u.created_at as u_created_at, u.points, u.level, u.is_admin as u_is_admin,
                   (SELECT GROUP_CONCAT(t.name) FROM tags t JOIN post_tags pt ON t.id = pt.tag_id WHERE pt.post_id = p.id) as tags,
                   (SELECT COUNT(*) FROM votes WHERE post_id = p.id AND value = 1) as like_count,
                   (SELECT COUNT(*) FROM comments WHERE post_id = p.id AND is_deleted = 0) as comment_count
            FROM posts p
            JOIN users u ON p.author_id = u.id
            WHERE p.is_deleted = 0
            ORDER BY p.is_pinned DESC, p.is_featured DESC, p.created_at DESC
            LIMIT ? OFFSET ?
            "#
        };

        let rows: Vec<PostListRow> = if let Some(tag) = tag {
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

        let summaries: Vec<PostSummary> = rows.into_iter().map(|row| {
            let tags: Vec<String> = row.tags.map(|t| t.split(',').map(String::from).collect()).unwrap_or_default();
            PostSummary {
                id: row.id,
                author_id: row.author_id,
                author: UserPublic {
                    id: row.u_id,
                    username: row.username,
                    avatar_url: row.avatar_url,
                    bio: row.bio,
                    is_admin: row.u_is_admin,
                    points: row.points,
                    level: row.level,
                    created_at: row.u_created_at,
                },
                title: row.title,
                content: row.content,
                tags,
                like_count: row.like_count,
                comment_count: row.comment_count,
                is_bookmarked: false,
                is_pinned: row.is_pinned,
                is_featured: row.is_featured,
                view_count: row.view_count,
                created_at: row.created_at,
            }
        }).collect();

        Ok((summaries, total))
    }

    pub async fn list_by_author(&self, author_id: &str, page: i64, limit: i64) -> Result<(Vec<PostSummary>, i64)> {
        let offset = (page - 1) * limit;

        let rows: Vec<PostListRow> = sqlx::query_as(
            r#"
            SELECT p.id, p.author_id, p.title, p.content, p.created_at, p.updated_at, p.is_pinned, p.is_featured, p.view_count,
                   u.id as u_id, u.username, u.avatar_url, u.bio, u.created_at as u_created_at, u.points, u.level, u.is_admin as u_is_admin,
                   (SELECT GROUP_CONCAT(t.name) FROM tags t JOIN post_tags pt ON t.id = pt.tag_id WHERE pt.post_id = p.id) as tags,
                   (SELECT COUNT(*) FROM votes WHERE post_id = p.id AND value = 1) as like_count,
                   (SELECT COUNT(*) FROM comments WHERE post_id = p.id AND is_deleted = 0) as comment_count
            FROM posts p
            JOIN users u ON p.author_id = u.id
            WHERE p.author_id = ? AND p.is_deleted = 0
            ORDER BY p.is_pinned DESC, p.is_featured DESC, p.created_at DESC
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

        let summaries: Vec<PostSummary> = rows.into_iter().map(|row| {
            let tags: Vec<String> = row.tags.map(|t| t.split(',').map(String::from).collect()).unwrap_or_default();
            PostSummary {
                id: row.id,
                author_id: row.author_id,
                author: UserPublic {
                    id: row.u_id,
                    username: row.username,
                    avatar_url: row.avatar_url,
                    bio: row.bio,
                    is_admin: row.u_is_admin,
                    points: row.points,
                    level: row.level,
                    created_at: row.u_created_at,
                },
                title: row.title,
                content: row.content,
                tags,
                like_count: row.like_count,
                comment_count: row.comment_count,
                is_bookmarked: false,
                is_pinned: row.is_pinned,
                is_featured: row.is_featured,
                view_count: row.view_count,
                created_at: row.created_at,
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

    pub async fn set_pin_featured(&self, post_id: &str, is_pinned: bool, is_featured: bool) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query("UPDATE posts SET is_pinned = ?, is_featured = ?, updated_at = ? WHERE id = ?")
            .bind(is_pinned)
            .bind(is_featured)
            .bind(&now)
            .bind(post_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn increment_view_count(&self, post_id: &str) -> Result<()> {
        sqlx::query("UPDATE posts SET view_count = view_count + 1 WHERE id = ?")
            .bind(post_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn list_hot(&self, page: i64, limit: i64) -> Result<(Vec<PostSummary>, i64)> {
        let offset = (page - 1) * limit;

        let rows: Vec<PostListRow> = sqlx::query_as(
            r#"
            SELECT p.id, p.author_id, p.title, p.content, p.created_at, p.updated_at, p.is_pinned, p.is_featured, p.view_count,
                   u.id as u_id, u.username, u.avatar_url, u.bio, u.created_at as u_created_at, u.points, u.level, u.is_admin as u_is_admin,
                   (SELECT GROUP_CONCAT(t.name) FROM tags t JOIN post_tags pt ON t.id = pt.tag_id WHERE pt.post_id = p.id) as tags,
                   (SELECT COUNT(*) FROM votes WHERE post_id = p.id AND value = 1) as like_count,
                   (SELECT COUNT(*) FROM comments WHERE post_id = p.id AND is_deleted = 0) as comment_count
            FROM posts p
            JOIN users u ON p.author_id = u.id
            WHERE p.is_deleted = 0
            ORDER BY (SELECT COUNT(*) FROM votes WHERE post_id = p.id AND value = 1) * 3
                     + (SELECT COUNT(*) FROM comments WHERE post_id = p.id AND is_deleted = 0) * 2
                     + p.view_count DESC,
                     p.created_at DESC
            LIMIT ? OFFSET ?
            "#
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool)
        .await?;

        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM posts WHERE is_deleted = 0")
            .fetch_one(self.pool)
            .await?;

        let summaries: Vec<PostSummary> = rows.into_iter().map(|row| {
            let tags: Vec<String> = row.tags.map(|t| t.split(',').map(String::from).collect()).unwrap_or_default();
            PostSummary {
                id: row.id,
                author_id: row.author_id,
                author: UserPublic {
                    id: row.u_id,
                    username: row.username,
                    avatar_url: row.avatar_url,
                    bio: row.bio,
                    is_admin: row.u_is_admin,
                    points: row.points,
                    level: row.level,
                    created_at: row.u_created_at,
                },
                title: row.title,
                content: row.content,
                tags,
                like_count: row.like_count,
                comment_count: row.comment_count,
                is_bookmarked: false,
                is_pinned: row.is_pinned,
                is_featured: row.is_featured,
                view_count: row.view_count,
                created_at: row.created_at,
            }
        }).collect();

        Ok((summaries, total))
    }
}