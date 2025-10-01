use crate::config::config::{Config, save_session_id};
use crate::domain::errors::config_error::ConfigError;
use crate::domain::errors::link_error::LinkError;
use crate::application::dto::link::StartLoginResponse;
use reqwest::Client;

impl From<ConfigError> for LinkError {
    fn from(err: ConfigError) -> Self {
        LinkError::ApiError(format!("Session save error: {}", err))
    }
}

pub async fn start_login(config: &Config) -> Result<String, LinkError> {
    let client = Client::new();
    let response = client
        .post(&format!("{}/start-login", config.api_url))
        .send()
        .await
        .map_err(|e| LinkError::ApiError(format!("Network error: {}", e)))?;

    if !response.status().is_success() {
        return Err(LinkError::ApiError(format!(
            "API error: {} {}",
            response.status(),
            response.text().await.unwrap_or_default()
        )));
    }

    let result: StartLoginResponse = response
        .json()
        .await
        .map_err(|e| LinkError::ApiError(format!("Invalid response: {}", e)))?;

    // Extract session_id from URL
    let session_id = result
        .url
        .split("session_id=")
        .nth(1)
        .ok_or_else(|| LinkError::ApiError("Missing session_id in URL".to_string()))?;

    save_session_id(session_id)?;
    Ok(result.url)
}

