use crate::application::auth;
use crate::application::dto::compute::{ListComputeResponse, LogsResponse, StatusErrorResponse, ComputeStatusResponse};
use crate::config::config::Config;
use crate::domain::errors::compute_error::ComputeError;

use anyhow::Result;
use reqwest::{Client, StatusCode};

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

pub async fn list_compute_with_deps<A: AuthProvider>(deployment_id: &str, config: &Config, auth_provider: &A) -> Result<ListComputeResponse, ComputeError> {
    let jwt_token = auth_provider
        .get_auth_token()
        .map_err(|e| ComputeError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy/{}/compute", config.api_url, deployment_id))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
        .map_err(ComputeError::RequestFailed)?;

    match response.status() {
        StatusCode::OK => response
            .json::<ListComputeResponse>()
            .await
            .map_err(|e| ComputeError::ParseError(e.to_string())),
        status => {
            let text = response.text().await.unwrap_or("No details".to_string());
            Err(ComputeError::Unexpected(format!("Status {}: {}", status, text)))
        }
    }
}

pub async fn list_compute(deployment_id: &str, config: &Config) -> Result<ListComputeResponse, ComputeError> {
    let auth_provider = DefaultAuthProvider;
    list_compute_with_deps(deployment_id, config, &auth_provider).await
}

pub async fn start_compute_with_deps<A: AuthProvider>(deployment_id: &str, config: &Config, auth_provider: &A) -> Result<(), ComputeError> {
    let jwt_token = auth_provider
        .get_auth_token()
        .map_err(|e| ComputeError::SessionError(format!("{}", e)))?;
    
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy/{}/start", config.api_url, deployment_id))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(ComputeError::RequestFailed)?;

    match response.status() {
        StatusCode::OK => Ok(()),
        status => {
            let text = response.text().await.unwrap_or("No details".to_string());
            Err(ComputeError::Unexpected(format!("Status {}: {}", status, text)))
        }
    }
}

pub async fn start_compute(deployment_id: &str, config: &Config) -> Result<(), ComputeError> {
    let auth_provider = DefaultAuthProvider;
    start_compute_with_deps(deployment_id, config, &auth_provider).await
}

pub async fn stop_compute_with_deps<A: AuthProvider>(deployment_id: &str, config: &Config, auth_provider: &A) -> Result<(), ComputeError> {
    let jwt_token = auth_provider
        .get_auth_token()
        .map_err(|e| ComputeError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy/{}/stop", config.api_url, deployment_id))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
        .map_err(ComputeError::RequestFailed)?;

    match response.status() {
        StatusCode::OK => Ok(()),
        status => {
            let text = response.text().await.unwrap_or("No details".to_string());
            Err(ComputeError::Unexpected(format!("Status {}: {}", status, text)))
        }
    }
}

pub async fn stop_compute(deployment_id: &str, config: &Config) -> Result<(), ComputeError> {
    let auth_provider = DefaultAuthProvider;
    stop_compute_with_deps(deployment_id, config, &auth_provider).await
}

pub async fn get_logs_with_deps<A: AuthProvider>(deployment_id: &str, config: &Config, auth_provider: &A) -> Result<LogsResponse, ComputeError> {
    let jwt_token = auth_provider
        .get_auth_token()
        .map_err(|e| ComputeError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy/{}/compute/logs", config.api_url, deployment_id))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
        .map_err(ComputeError::RequestFailed)?;

    match response.status() {
        StatusCode::OK => response
            .json::<LogsResponse>()
            .await
            .map_err(|e| ComputeError::ParseError(e.to_string())),
        StatusCode::INTERNAL_SERVER_ERROR => {
            let text = response.text().await.unwrap_or("No details".to_string());
            Err(ComputeError::InternalServerError(text))
        }
        status => {
            let text = response.text().await.unwrap_or("No details".to_string());
            Err(ComputeError::Unexpected(format!("Status {}: {}", status, text)))
        }
    }
}

pub async fn get_logs(deployment_id: &str, config: &Config) -> Result<LogsResponse, ComputeError> {
    let auth_provider = DefaultAuthProvider;
    get_logs_with_deps(deployment_id, config, &auth_provider).await
}

pub async fn get_status_with_deps<A: AuthProvider>(deployment_id: &str, config: &Config, auth_provider: &A) -> Result<ComputeStatusResponse, ComputeError> {
    let jwt_token = auth_provider
        .get_auth_token()
        .map_err(|e| ComputeError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy/{}/status", config.api_url, deployment_id))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
        .map_err(ComputeError::RequestFailed)?;

    match response.status() {
        StatusCode::OK => response
            .json::<ComputeStatusResponse>()
            .await
            .map_err(|e| ComputeError::ParseError(e.to_string())),
        StatusCode::GONE => {
            let error = response
                .json::<StatusErrorResponse>()
                .await
                .map_err(|e| ComputeError::ParseError(e.to_string()))?;
            Err(ComputeError::NotHealthy(error.message))
        }
        status => {
            let text = response.text().await.unwrap_or("No details".to_string());
            Err(ComputeError::Unexpected(format!("Status {}: {}", status, text)))
        }
    }
}

pub async fn get_status(deployment_id: &str, config: &Config) -> Result<ComputeStatusResponse, ComputeError> {
    let auth_provider = DefaultAuthProvider;
    get_status_with_deps(deployment_id, config, &auth_provider).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_compute_session_error() {
        let config = Config { api_url: "https://api.guepard.run".to_string() };
        let mut auth = MockAuthProvider::new();
        auth
            .expect_get_auth_token()
            .times(1)
            .returning(|| Err(crate::domain::errors::config_error::ConfigError::SessionError(
                "You need to log in first! Run `guepard login` to get started. 🐆".to_string()
            )));

        let result = list_compute_with_deps("dep-1", &config, &auth).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            ComputeError::SessionError(msg) => assert!(msg.contains("You need to log in first!")),
            _ => panic!("Expected SessionError"),
        }
    }

    #[tokio::test]
    async fn test_start_stop_logs_status_network_or_api_errors() {
        let config = Config { api_url: "https://api.guepard.run".to_string() };
        let mut auth = MockAuthProvider::new();
        auth
            .expect_get_auth_token()
            .times(4)
            .returning(|| Ok("test-jwt-token".to_string()));

        let r1 = start_compute_with_deps("dep-1", &config, &auth).await;
        assert!(r1.is_err());

        let r2 = stop_compute_with_deps("dep-1", &config, &auth).await;
        assert!(r2.is_err());

        let r3 = get_logs_with_deps("dep-1", &config, &auth).await;
        assert!(r3.is_err());

        let r4 = get_status_with_deps("dep-1", &config, &auth).await;
        assert!(r4.is_err());
    }
}