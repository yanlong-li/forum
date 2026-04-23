use sqlx::SqlitePool;
use crate::error::{AppError, Result};
use crate::models::follow::{FollowResponse, UserListResponse};
use crate::models::user::UserPublic;
use crate::repositories::{FollowRepository, UserRepository, NotificationRepository};
use uuid::Uuid;

pub struct FollowService<'a> {
    pool: &'a SqlitePool,
}

impl<'a> FollowService<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn follow(&self, follower_id: &str, following_id: &str) -> Result<FollowResponse> {
        if follower_id == following_id {
            return Err(AppError::ValidationError("Cannot follow yourself".to_string()));
        }

        let follow_repo = FollowRepository::new(self.pool);
        let user_repo = UserRepository::new(self.pool);

        user_repo.find_by_id(following_id).await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        if follow_repo.find(follower_id, following_id).await?.is_some() {
            return Err(AppError::Conflict("Already following".to_string()));
        }

        let follow_id = Uuid::new_v4().to_string();
        follow_repo.create(&follow_id, follower_id, following_id).await?;

        let notification_repo = NotificationRepository::new(self.pool);
        let notif_id = Uuid::new_v4().to_string();
        notification_repo.create(&notif_id, following_id, "follow", follower_id, None, None).await?;

        Ok(FollowResponse {
            message: "User followed".to_string()
        })
    }

    pub async fn unfollow(&self, follower_id: &str, following_id: &str) -> Result<()> {
        let follow_repo = FollowRepository::new(self.pool);

        follow_repo.delete(follower_id, following_id).await
    }

    pub async fn get_followers(&self, user_id: &str, page: i64, limit: i64) -> Result<UserListResponse> {
        let follow_repo = FollowRepository::new(self.pool);
        let user_repo = UserRepository::new(self.pool);

        let (follower_ids, total) = follow_repo.get_followers(user_id, page, limit).await?;

        let mut users = Vec::new();
        for id in follower_ids {
            if let Some(user) = user_repo.find_by_id(&id).await? {
                users.push(UserPublic {
                    id: user.id.clone(),
                    username: user.username.clone(),
                    avatar_url: user.avatar_url.clone(),
                    bio: user.bio.clone(),
                    is_admin: user.is_admin,
                    points: user.points,
                    level: user.level,
                    created_at: user.created_at.clone(),
                });
            }
        }

        Ok(UserListResponse {
            users,
            total,
            page,
            limit,
        })
    }

    pub async fn get_following(&self, user_id: &str, page: i64, limit: i64) -> Result<UserListResponse> {
        let follow_repo = FollowRepository::new(self.pool);
        let user_repo = UserRepository::new(self.pool);

        let (following_ids, total) = follow_repo.get_following(user_id, page, limit).await?;

        let mut users = Vec::new();
        for id in following_ids {
            if let Some(user) = user_repo.find_by_id(&id).await? {
                users.push(UserPublic {
                    id: user.id.clone(),
                    username: user.username.clone(),
                    avatar_url: user.avatar_url.clone(),
                    bio: user.bio.clone(),
                    is_admin: user.is_admin,
                    points: user.points,
                    level: user.level,
                    created_at: user.created_at.clone(),
                });
            }
        }

        Ok(UserListResponse {
            users,
            total,
            page,
            limit,
        })
    }
}
