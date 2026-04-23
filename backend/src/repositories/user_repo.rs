use sqlx::SqlitePool;
use crate::error::{AppError, Result};
use crate::models::user::{User, UserProfile, UserPublic, OAuthAccount};

pub struct UserRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, id: &str, username: &str, email: &str, password_hash: &str) -> Result<User> {
        let now = chrono::Utc::now().to_rfc3339();
        let is_first = self.count().await? == 0;
        let is_admin = is_first;

        sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, username, email, password_hash, is_admin, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .bind(is_admin)
        .bind(&now)
        .bind(&now)
        .fetch_one(self.pool)
        .await
        .map_err(|_| AppError::Conflict("User already exists".to_string()))
    }

    pub async fn count(&self) -> Result<i64> {
        let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
            .fetch_one(self.pool)
            .await?;
        Ok(result.0)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = ?"
        )
        .bind(email)
        .fetch_optional(self.pool)
        .await?;
        Ok(user)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?;
        Ok(user)
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE username = ?"
        )
        .bind(username)
        .fetch_optional(self.pool)
        .await?;
        Ok(user)
    }

    pub async fn get_profile(&self, username: &str, current_user_id: Option<&str>) -> Result<Option<UserProfile>> {
        let user = sqlx::query_as::<_, (String, String, Option<String>, Option<String>, String, i64, i64, i64)>(
            r#"
            SELECT u.id, u.username, u.avatar_url, u.bio, u.created_at,
                   (SELECT COUNT(*) FROM posts WHERE author_id = u.id AND is_deleted = 0) as post_count,
                   (SELECT COUNT(*) FROM follows WHERE following_id = u.id) as follower_count,
                   (SELECT COUNT(*) FROM follows WHERE follower_id = u.id) as following_count
            FROM users u
            WHERE u.username = ?
            "#
        )
        .bind(username)
        .fetch_optional(self.pool)
        .await?;

        if let Some((id, username, avatar_url, bio, created_at, post_count, follower_count, following_count)) = user {
            let is_following = if let Some(current_id) = current_user_id {
                let count = sqlx::query_scalar::<_, i64>(
                    "SELECT COUNT(*) FROM follows WHERE follower_id = ? AND following_id = ?"
                )
                .bind(current_id)
                .bind(&id)
                .fetch_one(self.pool)
                .await?;
                count > 0
            } else {
                false
            };

            Ok(Some(UserProfile {
                id,
                username,
                avatar_url,
                bio,
                post_count,
                follower_count,
                following_count,
                is_following,
                created_at,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn update(&self, id: &str, username: Option<&str>, bio: Option<&str>, avatar_url: Option<&str>) -> Result<User> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET username = COALESCE(?, username),
                bio = COALESCE(?, bio),
                avatar_url = COALESCE(?, avatar_url),
                updated_at = ?
            WHERE id = ?
            RETURNING *
            "#
        )
        .bind(username)
        .bind(bio)
        .bind(avatar_url)
        .bind(&now)
        .bind(id)
        .fetch_one(self.pool)
        .await
        .map_err(|_| AppError::InternalError("Failed to update user".to_string()))
    }

    pub async fn create_oauth_account(&self, id: &str, user_id: &str, provider: &str, provider_user_id: &str) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query(
            r#"
            INSERT INTO oauth_accounts (id, user_id, provider, provider_user_id, created_at)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
        .bind(id)
        .bind(user_id)
        .bind(provider)
        .bind(provider_user_id)
        .bind(&now)
        .execute(self.pool)
        .await
        .map_err(|_| AppError::InternalError("Failed to create OAuth account".to_string()))?;
        Ok(())
    }

    pub async fn find_oauth_account(&self, provider: &str, provider_user_id: &str) -> Result<Option<OAuthAccount>> {
        let account = sqlx::query_as::<_, OAuthAccount>(
            "SELECT * FROM oauth_accounts WHERE provider = ? AND provider_user_id = ?"
        )
        .bind(provider)
        .bind(provider_user_id)
        .fetch_optional(self.pool)
        .await?;
        Ok(account)
    }

    pub async fn find_oauth_by_user_id(&self, user_id: &str) -> Result<Vec<OAuthAccount>> {
        let accounts = sqlx::query_as::<_, OAuthAccount>(
            "SELECT * FROM oauth_accounts WHERE user_id = ?"
        )
        .bind(user_id)
        .fetch_all(self.pool)
        .await?;
        Ok(accounts)
    }

    pub async fn lock_user(&self, id: &str, locked_until: &str) -> Result<()> {
        sqlx::query("UPDATE users SET is_locked = 1, locked_until = ? WHERE id = ?")
            .bind(locked_until)
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn unlock_user(&self, id: &str) -> Result<()> {
        sqlx::query("UPDATE users SET is_locked = 0, failed_login_attempts = 0, locked_until = NULL WHERE id = ?")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn increment_failed_attempts(&self, id: &str) -> Result<()> {
        sqlx::query("UPDATE users SET failed_login_attempts = failed_login_attempts + 1 WHERE id = ?")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn reset_failed_attempts(&self, id: &str) -> Result<()> {
        sqlx::query("UPDATE users SET failed_login_attempts = 0 WHERE id = ?")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn list_users(&self, page: i64, limit: i64, search: Option<&str>) -> Result<(Vec<UserPublic>, i64)> {
        let offset = (page - 1) * limit;

        let users = if let Some(search) = search {
            sqlx::query_as::<_, UserPublic>(
                r#"
                SELECT id, username, avatar_url, bio, is_admin, points, level, created_at
                FROM users
                WHERE username LIKE ? OR email LIKE ?
                ORDER BY created_at DESC
                LIMIT ? OFFSET ?
                "#
            )
            .bind(format!("%{}%", search))
            .bind(format!("%{}%", search))
            .bind(limit)
            .bind(offset)
            .fetch_all(self.pool)
            .await?
        } else {
            sqlx::query_as::<_, UserPublic>(
                r#"
                SELECT id, username, avatar_url, bio, is_admin, points, level, created_at
                FROM users
                ORDER BY created_at DESC
                LIMIT ? OFFSET ?
                "#
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(self.pool)
            .await?
        };

        let total: i64 = if let Some(search) = search {
            sqlx::query_scalar(
                "SELECT COUNT(*) FROM users WHERE username LIKE ? OR email LIKE ?"
            )
            .bind(format!("%{}%", search))
            .bind(format!("%{}%", search))
            .fetch_one(self.pool)
            .await?
        } else {
            sqlx::query_scalar("SELECT COUNT(*) FROM users")
                .fetch_one(self.pool)
                .await?
        };

        Ok((users, total))
    }
}
