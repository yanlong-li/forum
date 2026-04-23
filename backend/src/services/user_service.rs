use sqlx::SqlitePool;
use crate::error::{AppError, Result};
use crate::models::user::{UserProfile, UserPublic, UpdateProfileRequest};
use crate::models::follow::UserListResponse;
use crate::repositories::UserRepository;

pub struct UserService<'a> {
    pool: &'a SqlitePool,
}

impl<'a> UserService<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_profile(&self, username: &str, current_user_id: Option<&str>) -> Result<UserProfile> {
        let user_repo = UserRepository::new(self.pool);
        user_repo.get_profile(username, current_user_id).await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))
    }

    pub async fn update_profile(&self, user_id: &str, req: UpdateProfileRequest) -> Result<UserPublic> {
        let user_repo = UserRepository::new(self.pool);

        if let Some(username) = &req.username {
            if user_repo.find_by_username(username).await?.is_some() {
                return Err(AppError::Conflict("Username already taken".to_string()));
            }
        }

        let user = user_repo.update(user_id, req.username.as_deref(), req.bio.as_deref(), req.avatar_url.as_deref()).await?;

        Ok(UserPublic {
            id: user.id,
            username: user.username,
            avatar_url: user.avatar_url,
            bio: user.bio,
            is_admin: user.is_admin,
            points: user.points,
            level: user.level,
            created_at: user.created_at,
        })
    }

    pub async fn get_user_by_id(&self, id: &str) -> Result<UserPublic> {
        let user_repo = UserRepository::new(self.pool);
        let user = user_repo.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(UserPublic {
            id: user.id,
            username: user.username,
            avatar_url: user.avatar_url,
            bio: user.bio,
            is_admin: user.is_admin,
            points: user.points,
            level: user.level,
            created_at: user.created_at,
        })
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<UserPublic> {
        let user_repo = UserRepository::new(self.pool);
        let user = user_repo.find_by_username(username).await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(UserPublic {
            id: user.id,
            username: user.username,
            avatar_url: user.avatar_url,
            bio: user.bio,
            is_admin: user.is_admin,
            points: user.points,
            level: user.level,
            created_at: user.created_at,
        })
    }

    pub async fn list_users(&self, page: i64, limit: i64, search: Option<&str>) -> Result<UserListResponse> {
        let user_repo = UserRepository::new(self.pool);
        let (users, total) = user_repo.list_users(page, limit, search).await?;

        Ok(UserListResponse {
            users,
            total,
            page,
            limit,
        })
    }

    pub async fn lock_user(&self, user_id: &str, locked_until: &str) -> Result<()> {
        let user_repo = UserRepository::new(self.pool);
        user_repo.lock_user(user_id, locked_until).await
    }

    pub async fn unlock_user(&self, user_id: &str) -> Result<()> {
        let user_repo = UserRepository::new(self.pool);
        user_repo.unlock_user(user_id).await
    }
}
