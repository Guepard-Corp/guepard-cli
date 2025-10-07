use crate::config::config::{self, Config};
use crate::domain::errors::login_error::LoginError;
use crate::application::dto::login::{CompleteLoginRequest, CompleteLoginResponse, StartLoginResponse};
use anyhow::Context;
use reqwest::Client;

// Dependency injection for session storage to make testing easier
#[cfg_attr(test, mockall::automock)]
pub trait SessionStore {
    fn save_session_id(&self, session_id: &str) -> Result<(), crate::domain::errors::config_error::ConfigError>;
    fn load_session_id(&self) -> Result<String, crate::domain::errors::config_error::ConfigError>;
    fn save_jwt_token(&self, token: &str) -> Result<(), crate::domain::errors::config_error::ConfigError>;
}

pub struct DefaultSessionStore;

impl SessionStore for DefaultSessionStore {
    fn save_session_id(&self, session_id: &str) -> Result<(), crate::domain::errors::config_error::ConfigError> {
        config::save_session_id(session_id)
    }

    fn load_session_id(&self) -> Result<String, crate::domain::errors::config_error::ConfigError> {
        config::load_session_id()
    }

    fn save_jwt_token(&self, token: &str) -> Result<(), crate::domain::errors::config_error::ConfigError> {
        config::save_jwt_token(token)
    }
}

// Pure helper to extract the session id from the start-login URL
pub fn extract_session_id(url: &str) -> Result<String, LoginError> {
    let session_id = url
        .split("session_id=")
        .nth(1)
        .ok_or_else(|| LoginError::ApiError("Missing session_id in URL".to_string()))?
        .split('&')
        .next()
        .unwrap_or("")
        .trim()
        .to_string();
    if session_id.is_empty() {
        return Err(LoginError::ApiError("Missing session_id in URL".to_string()));
    }
    Ok(session_id)
}

pub async fn start_login_with_deps<S: SessionStore>(config: &Config, session_store: &S) -> Result<String, LoginError> {
    let client = Client::new();
    let response = client
        .post(&format!("{}/start-login", config.api_url))
        .send()
        .await
        .map_err(|e| LoginError::ApiError(format!("Network error: {}", e)))?;

    if !response.status().is_success() {
        return Err(LoginError::ApiError(format!(
            "API error: {} {}",
            response.status(),
            response.text().await.unwrap_or_default()
        )));
    }

    let result: StartLoginResponse = response
        .json()
        .await
        .map_err(|e| LoginError::ApiError(format!("Invalid response: {}", e)))?;

    // Extract session_id from URL
    let session_id = extract_session_id(&result.url)?;

    session_store
        .save_session_id(&session_id)
        .map_err(|e| LoginError::SessionError(e.to_string()))?;
    
    Ok(result.url)
}

pub async fn start_login(config: &Config) -> Result<String, LoginError> {
    let store = DefaultSessionStore;
    start_login_with_deps(config, &store).await
}

pub async fn complete_login_with_deps<S: SessionStore>(config: &Config, verification_code: &str, session_store: &S) -> anyhow::Result<String> {
    let session_id = session_store
        .load_session_id()
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

    session_store
        .save_jwt_token(&result.token)
        .map_err(|e| LoginError::SessionError(e.to_string()))
        .context("Failed to save JWT token")?;
    Ok(result.token)
}

pub async fn complete_login(config: &Config, verification_code: &str) -> anyhow::Result<String> {
    let store = DefaultSessionStore;
    complete_login_with_deps(config, verification_code, &store).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_session_id_ok() {
        let url = "https://example.com/callback?session_id=abc123&x=1";
        let id = extract_session_id(url).unwrap();
        assert_eq!(id, "abc123");
    }

    #[test]
    fn test_extract_session_id_missing() {
        let url = "https://example.com/callback?x=1";
        let err = extract_session_id(url).unwrap_err();
        match err {
            LoginError::ApiError(msg) => assert!(msg.contains("Missing session_id")),
            _ => panic!("Expected ApiError"),
        }
    }

    #[tokio::test]
    async fn test_complete_login_session_error() {
        let config = Config { api_url: "https://api.guepard.run".to_string() };

        let mut store = MockSessionStore::new();
        store
            .expect_load_session_id()
            .times(1)
            .returning(|| Err(crate::domain::errors::config_error::ConfigError::SessionError(
                "You need to log in first! Run `guepard login` to get started. 🐆".to_string()
            )));

        let result = complete_login_with_deps(&config, "0000", &store).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = format!("{}", err);
        assert!(msg.contains("Failed to load session ID"));
    }

    #[tokio::test]
    async fn test_complete_login_network_or_api_error_after_session_ok() {
        let config = Config { api_url: "https://api.guepard.run".to_string() };

        let mut store = MockSessionStore::new();
        store
            .expect_load_session_id()
            .times(1)
            .returning(|| Ok("session-xyz".to_string()));

        // We don't mock HTTP; expect an error from network or API. The concrete
        // error type is wrapped in anyhow, so just assert it's an Err.
        let result = complete_login_with_deps(&config, "0000", &store).await;
        assert!(result.is_err());
    }
}
