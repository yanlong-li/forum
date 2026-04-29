use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::error::{AppError, Result};

#[derive(Debug, Deserialize)]
pub struct GitHubUser {
    pub id: i64,
    pub login: String,
    pub avatar_url: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GitHubTokenRequest {
    pub client_id: String,
    pub client_secret: String,
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
}

pub struct GitHubOAuth {
    config: Config,
}

impl GitHubOAuth {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn get_auth_url(&self) -> String {
        format!(
            "https://github.com/login/oauth/authorize?client_id={}&scope=read:user,user:email",
            self.config.github_client_id
        )
    }

    pub async fn exchange_code(&self, code: &str) -> Result<GitHubTokenResponse> {
        let client = reqwest::Client::new();

        let params = GitHubTokenRequest {
            client_id: self.config.github_client_id.clone(),
            client_secret: self.config.github_client_secret.clone(),
            code: code.to_string(),
        };

        let response = client
            .post("https://github.com/login/oauth/access_token")
            .form(&params)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|_| AppError::InternalError("Failed to exchange code".to_string()))?;

        let token_response: GitHubTokenResponse = response
            .json()
            .await
            .map_err(|_| AppError::InternalError("Failed to parse token response".to_string()))?;

        Ok(token_response)
    }

    pub async fn get_user(&self, access_token: &str) -> Result<GitHubUser> {
        let client = reqwest::Client::new();

        let response = client
            .get("https://api.github.com/user")
            .header("Authorization", format!("Bearer {}", access_token))
            .header("User-Agent", "community-forum")
            .send()
            .await
            .map_err(|_| AppError::InternalError("Failed to get user".to_string()))?;

        let user: GitHubUser = response
            .json()
            .await
            .map_err(|_| AppError::InternalError("Failed to parse user response".to_string()))?;

        Ok(user)
    }
}
