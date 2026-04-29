use chrono::{Duration, Utc};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use crate::config::Config;
use crate::error::{AppError, Result};
use crate::models::user::{UserPublic, AuthResponse, RegisterRequest, LoginRequest};
use crate::models::response::MessageResponse;
use crate::repositories::UserRepository;
use uuid::Uuid;
use bcrypt::{hash, verify};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub token_type: String,
}

pub struct AuthService<'a> {
    pool: &'a SqlitePool,
    config: &'a Config,
}

impl<'a> AuthService<'a> {
    pub fn new(pool: &'a SqlitePool, config: &'a Config) -> Self {
        Self { pool, config }
    }

    pub async fn register(&self, req: RegisterRequest) -> Result<MessageResponse> {
        let user_repo = UserRepository::new(self.pool);

        if user_repo.find_by_email(&req.email).await?.is_some() {
            return Err(AppError::Conflict("Email already registered".to_string()));
        }

        if user_repo.find_by_username(&req.username).await?.is_some() {
            return Err(AppError::Conflict("Username already taken".to_string()));
        }

        let password_hash = hash(&req.password, 12)
            .map_err(|_| AppError::InternalError("Failed to hash password".to_string()))?;

        let user_id = Uuid::new_v4().to_string();
        user_repo.create(&user_id, &req.username, &req.email, &password_hash).await?;

        Ok(MessageResponse {
            message: "Registration successful. Please check your email to verify.".to_string()
        })
    }

    pub async fn login(&self, req: LoginRequest) -> Result<AuthResponse> {
        let user_repo = UserRepository::new(self.pool);

        let user = user_repo.find_by_email(&req.email).await?
            .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

        if user.is_locked {
            return Err(AppError::Forbidden("Account is locked".to_string()));
        }

        let password_hash = user.password_hash.as_ref()
            .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

        if !verify(&req.password, password_hash).unwrap_or(false) {
            user_repo.increment_failed_attempts(&user.id).await?;

            let attempts = user.failed_login_attempts + 1;
            if attempts >= 5 {
                let locked_until = (Utc::now() + Duration::minutes(30)).to_rfc3339();
                user_repo.lock_user(&user.id, &locked_until).await?;
                return Err(AppError::Forbidden("Account is locked due to too many failed attempts".to_string()));
            }

            return Err(AppError::Unauthorized("Invalid credentials".to_string()));
        }

        user_repo.reset_failed_attempts(&user.id).await?;

        let access_token = self.generate_token(&user.id, self.config.jwt_access_expires, "access")?;
        let refresh_token = self.generate_token(&user.id, self.config.jwt_refresh_expires, "refresh")?;

        Ok(AuthResponse {
            access_token,
            refresh_token,
            user: UserPublic {
                id: user.id,
                username: user.username,
                avatar_url: user.avatar_url,
                bio: user.bio,
                is_admin: user.is_admin,
                points: user.points,
                level: user.level,
                created_at: user.created_at,
            },
        })
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<AuthResponse> {
        let token_data = decode::<AccessTokenClaims>(
            refresh_token,
            &DecodingKey::from_secret(self.config.jwt_secret.as_bytes()),
            &Validation::default()
        ).map_err(|_| AppError::Unauthorized("Invalid refresh token".to_string()))?;

        if token_data.claims.token_type != "refresh" {
            return Err(AppError::Unauthorized("Invalid token type".to_string()));
        }

        let user_repo = UserRepository::new(self.pool);
        let user = user_repo.find_by_id(&token_data.claims.sub).await?
            .ok_or_else(|| AppError::Unauthorized("User not found".to_string()))?;

        let access_token = self.generate_token(&user.id, self.config.jwt_access_expires, "access")?;
        let new_refresh_token = self.generate_token(&user.id, self.config.jwt_refresh_expires, "refresh")?;

        Ok(AuthResponse {
            access_token,
            refresh_token: new_refresh_token,
            user: UserPublic {
                id: user.id,
                username: user.username,
                avatar_url: user.avatar_url,
                bio: user.bio,
                is_admin: user.is_admin,
                points: user.points,
                level: user.level,
                created_at: user.created_at,
            },
        })
    }

    fn generate_token(&self, user_id: &str, expires_in: i64, token_type: &str) -> Result<String> {
        let now = Utc::now();
        let exp = (now + Duration::seconds(expires_in)).timestamp() as usize;

        let claims = AccessTokenClaims {
            sub: user_id.to_string(),
            exp,
            iat: now.timestamp() as usize,
            token_type: token_type.to_string(),
        };

        encode(&Header::default(), &claims, &EncodingKey::from_secret(self.config.jwt_secret.as_bytes()))
            .map_err(|_| AppError::InternalError("Failed to generate token".to_string()))
    }
}
