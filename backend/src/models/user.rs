use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub email_verified: bool,
    pub is_admin: bool,
    pub is_locked: bool,
    pub failed_login_attempts: i32,
    pub locked_until: Option<String>,
    pub points: i64,
    pub level: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserProfile {
    pub id: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub post_count: i64,
    pub follower_count: i64,
    pub following_count: i64,
    #[serde(default)]
    pub is_following: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserPublic {
    pub id: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub is_admin: bool,
    pub points: i64,
    pub level: i64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OAuthAccount {
    pub id: String,
    pub user_id: String,
    pub provider: String,
    pub provider_user_id: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserPublic,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateProfileRequest {
    pub username: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
}
