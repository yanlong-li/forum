use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::AppError;
use crate::models::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: String,
    pub username: String,
    pub is_admin: bool,
}

pub struct AuthenticatedUser(pub AuthUser);

pub struct OptionalAuthenticatedUser(pub Option<AuthUser>);

impl FromRequestParts<Arc<AppState>> for AuthenticatedUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &Arc<AppState>) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("Missing authorization header".to_string()))?;

        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::Unauthorized("Invalid authorization format".to_string()));
        }

        let token = &auth_header[7..];

        let token_data = jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(state.config.jwt_secret.as_bytes()),
            &jsonwebtoken::Validation::default()
        )
        .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?;

        let user_id = token_data.claims.sub;
        let user = sqlx::query_as::<_, (String, String, bool)>(
            "SELECT id, username, is_admin FROM users WHERE id = ?"
        )
        .bind(&user_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| AppError::InternalError("Database error".to_string()))?
        .ok_or_else(|| AppError::Unauthorized("User not found".to_string()))?;

        Ok(AuthenticatedUser(AuthUser {
            id: user.0,
            username: user.1,
            is_admin: user.2,
        }))
    }
}

impl FromRequestParts<Arc<AppState>> for OptionalAuthenticatedUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &Arc<AppState>) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok());

        if let Some(auth_value) = auth_header {
            if auth_value.starts_with("Bearer ") {
                let token = &auth_value[7..];

                if let Ok(token_data) = jsonwebtoken::decode::<Claims>(
                    token,
                    &jsonwebtoken::DecodingKey::from_secret(state.config.jwt_secret.as_bytes()),
                    &jsonwebtoken::Validation::default()
                ) {
                    let user_id = token_data.claims.sub;
                    let user = sqlx::query_as::<_, (String, String, bool)>(
                        "SELECT id, username, is_admin FROM users WHERE id = ?"
                    )
                    .bind(&user_id)
                    .fetch_optional(&state.db)
                    .await
                    .map_err(|_| AppError::InternalError("Database error".to_string()))?;

                    if let Some((id, username, is_admin)) = user {
                        return Ok(OptionalAuthenticatedUser(Some(AuthUser { id, username, is_admin })));
                    }
                }
            }
        }

        Ok(OptionalAuthenticatedUser(None))
    }
}
