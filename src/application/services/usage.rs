use crate::application::auth;
use crate::application::dto::usage::UsageResponse;
use crate::config::config::Config;
use crate::domain::errors::usage_error::UsageError;
use anyhow::Result;
use reqwest::{Client, StatusCode};

#[cfg(test)]
use mockall::predicate::*;

// Trait for dependency injection to make testing easier
#[cfg_attr(test, mockall::automock)]
pub trait AuthProvider {
    fn get_auth_token(&self) -> Result<String, crate::domain::errors::config_error::ConfigError>;
}

// Default implementation that uses the real auth module
pub struct DefaultAuthProvider;

impl AuthProvider for DefaultAuthProvider {
    fn get_auth_token(&self) -> Result<String, crate::domain::errors::config_error::ConfigError> {
        auth::get_auth_token()
    }
}

// Main function that uses dependency injection
pub async fn get_usage_with_deps<A: AuthProvider>(
    config: &Config,
    auth_provider: &A,
) -> Result<UsageResponse, UsageError> {
    let jwt_token = auth_provider
        .get_auth_token()
        .map_err(|e| UsageError::SessionError(format!("{}", e)))?;
    
    let client = Client::new();
    let response = client
        .get(format!("{}/usage", config.api_url))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
        .map_err(UsageError::RequestFailed)?;

    match response.status() {
        StatusCode::OK => response
            .json::<UsageResponse>()
            .await
            .map_err(|e| UsageError::ParseError(e.to_string())),
        StatusCode::FORBIDDEN => {
            let text = response.text().await.unwrap_or("No details".to_string());
            Err(UsageError::Forbidden(text))
        }
        StatusCode::INTERNAL_SERVER_ERROR => {
            let text = response.text().await.unwrap_or("No details".to_string());
            Err(UsageError::InternalServerError(text))
        }
        status => {
            let text = response.text().await.unwrap_or("No details".to_string());
            Err(UsageError::Unexpected(format!("Status {}: {}", status, text)))
        }
    }
}

// Public function that maintains the original API
pub async fn get_usage(config: &Config) -> Result<UsageResponse, UsageError> {
    let auth_provider = DefaultAuthProvider;
    get_usage_with_deps(config, &auth_provider).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::errors::config_error::ConfigError;

    #[tokio::test]
    async fn test_get_usage_session_error() {
        let config = Config {
            api_url: "https://api.guepard.run".to_string(),
        };

        let mut mock_auth = MockAuthProvider::new();
        mock_auth
            .expect_get_auth_token()
            .times(1)
            .returning(|| {
                Err(ConfigError::SessionError(
                    "You need to log in first! Run `guepard login` to get started. 🐆".to_string(),
                ))
            });

        let result = get_usage_with_deps(&config, &mock_auth).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            UsageError::SessionError(msg) => {
                assert!(msg.contains("You need to log in first!"));
            }
            _ => panic!("Expected SessionError"),
        }
    }

    #[tokio::test]
    async fn test_get_usage_success_auth_provider() {
        let config = Config {
            api_url: "https://api.guepard.run".to_string(),
        };

        let mut mock_auth = MockAuthProvider::new();
        mock_auth
            .expect_get_auth_token()
            .times(1)
            .returning(|| Ok("test-jwt-token".to_string()));

        // This test will fail with network error since we're not mocking the HTTP client,
        // but it verifies that the auth provider is called correctly
        let result = get_usage_with_deps(&config, &mock_auth).await;

        // We expect an error since we're not mocking the HTTP call
        assert!(result.is_err());
        let error = result.unwrap_err();
        match error {
            UsageError::RequestFailed(_) | UsageError::SessionError(_) | UsageError::Forbidden(_) => {
                // All are acceptable - RequestFailed for network issues, SessionError for auth issues, Forbidden for invalid token
            }
            _ => panic!("Expected RequestFailed, SessionError, or Forbidden error due to network call, got: {:?}", error),
        }
    }

    // Integration test that would require a mock HTTP server
    // For now, we'll test the error handling logic through the auth provider
    #[tokio::test]
    async fn test_auth_provider_integration() {
        let config = Config {
            api_url: "https://api.guepard.run".to_string(),
        };

        // Test with a mock auth provider that returns different error types
        let mut mock_auth = MockAuthProvider::new();
        mock_auth
            .expect_get_auth_token()
            .times(1)
            .returning(|| {
                Err(ConfigError::IoError("File not found".to_string()))
            });

        let result = get_usage_with_deps(&config, &mock_auth).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            UsageError::SessionError(msg) => {
                assert!(msg.contains("File not found"));
            }
            _ => panic!("Expected SessionError with IoError message"),
        }
    }

    // Test the original public API still works
    #[tokio::test]
    async fn test_get_usage_public_api() {
        let config = Config {
            api_url: "https://api.guepard.run".to_string(),
        };

        // This will fail with auth error since we don't have a real session
        let result = get_usage(&config).await;

        // We expect either a SessionError (no auth) or RequestFailed (network)
        assert!(result.is_err());
        match result.unwrap_err() {
            UsageError::SessionError(_) | UsageError::RequestFailed(_) => {
                // Both are acceptable depending on the environment
            }
            _ => panic!("Expected SessionError or RequestFailed"),
        }
    }
}