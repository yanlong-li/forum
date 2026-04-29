use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_access_expires: i64,
    pub jwt_refresh_expires: i64,
    pub host: String,
    pub port: String,
    pub github_client_id: String,
    pub github_client_secret: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub frontend_url: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Config {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite:forum.db?mode=rwc".to_string()),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-super-secret-jwt-key-change-in-production".to_string()),
            jwt_access_expires: std::env::var("JWT_ACCESS_EXPIRES")
                .unwrap_or_else(|_| "86400".to_string())
                .parse()?,

            jwt_refresh_expires: std::env::var("JWT_REFRESH_EXPIRES")
                .unwrap_or_else(|_| "604800".to_string())
                .parse()?,
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("PORT").unwrap_or_else(|_| "3000".to_string()),
            github_client_id: std::env::var("GITHUB_CLIENT_ID").unwrap_or_default(),
            github_client_secret: std::env::var("GITHUB_CLIENT_SECRET").unwrap_or_default(),
            google_client_id: std::env::var("GOOGLE_CLIENT_ID").unwrap_or_default(),
            google_client_secret: std::env::var("GOOGLE_CLIENT_SECRET").unwrap_or_default(),
            frontend_url: std::env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:5173".to_string()),
        })
    }
}
