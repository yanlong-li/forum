use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::error::{AppError, Result};

#[derive(Debug, Deserialize)]
pub struct GoogleUser {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GoogleTokenRequest {
    pub code: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub grant_type: String,
}

#[derive(Debug, Deserialize)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
    pub refresh_token: Option<String>,
    pub id_token: String,
}

pub struct GoogleOAuth {
    config: Config,
}

impl GoogleOAuth {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn get_auth_url(&self) -> String {
        let redirect_uri = format!("{}/api/v1/auth/google/callback", self.config.frontend_url);
        format!(
            "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&response_type=code&scope=openid%20email%20profile&redirect_uri={}&state=random_state_string",
            self.config.google_client_id,
            redirect_uri
        )
    }

    pub async fn exchange_code(&self, code: &str) -> Result<GoogleTokenResponse> {
        let client = reqwest::Client::new();
        let redirect_uri = format!("{}/api/v1/auth/google/callback", self.config.frontend_url);

        let params = GoogleTokenRequest {
            code: code.to_string(),
            client_id: self.config.google_client_id.clone(),
            client_secret: self.config.google_client_secret.clone(),
            redirect_uri,
            grant_type: "authorization_code".to_string(),
        };

        let response = client
            .post("https://oauth2.googleapis.com/token")
            .form(&params)
            .send()
            .await
            .map_err(|_| AppError::InternalError("Failed to exchange code".to_string()))?;

        let token_response: GoogleTokenResponse = response
            .json()
            .await
            .map_err(|_| AppError::InternalError("Failed to parse token response".to_string()))?;

        Ok(token_response)
    }

    pub async fn get_user(&self, access_token: &str) -> Result<GoogleUser> {
        let client = reqwest::Client::new();

        let response = client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|_| AppError::InternalError("Failed to get user".to_string()))?;

        let user: GoogleUser = response
            .json()
            .await
            .map_err(|_| AppError::InternalError("Failed to parse user response".to_string()))?;

        Ok(user)
    }
}
