use crate::config::config::{self, Config};
use crate::domain::errors::login_error::LoginError;
use crate::application::dto::login_dto::{CompleteLoginRequest, CompleteLoginResponse};
use anyhow::Context;
use reqwest::Client;

pub async fn complete_login(config: &Config, verification_code: &str) -> anyhow::Result<String> {
    let session_id = config::load_session_id()
        .map_err(|e| LoginError::SessionError(e.to_string()))
        .context("Failed to load session ID")?;

    let client = Client::new();
    let request = CompleteLoginRequest {
        session_id,
        verification_code: verification_code.to_string(),
    };

    let response = client
        .post(&format!("{}/end-login", config.api_url))
        .json(&request)
        .send()
        .await
        .map_err(|e| LoginError::ApiError(format!("Network error: {}", e)))?;

    if !response.status().is_success() {
        return Err(LoginError::ApiError(format!(
            "API error: {} {}",
            response.status(),
            response.text().await.unwrap_or_default()
        ))
        .into());
    }

    let result: CompleteLoginResponse = response
        .json()
        .await
        .map_err(|e| LoginError::ApiError(format!("Invalid response: {}", e)))?;

    config::save_jwt_token(&result.token)
        .map_err(|e| LoginError::SessionError(e.to_string()))
        .context("Failed to save JWT token")?;
    Ok(result.token)
}
