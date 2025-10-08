use anyhow::{Context, Result};
use reqwest::Client;

use crate::application::auth;
use crate::application::dto::deploy::{
    CreateDeploymentRequest, CreateDeploymentResponse, GetDeploymentResponse, ListDeploymentsResponse,
    UpdateDeploymentRequest,
};
use crate::config::config::Config;
use crate::domain::errors::deploy_error::{handle_api_response, DeployError};

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

pub async fn create_deployment_with_deps<A: AuthProvider>(request: CreateDeploymentRequest, config: &Config, auth_provider: &A) -> Result<CreateDeploymentResponse, DeployError> {
    let jwt_token = auth_provider
        .get_auth_token()
        .map_err(|e| DeployError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    let response = client
        .post(format!("{}/deploy", config.api_url))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .json(&request)
        .send()
        .await
        .map_err(|e| DeployError::ApiError(format!("Network error: {}", e)))?;

    if !response.status().is_success() {
        return Err(DeployError::ApiError(format!(
            "API error: {} {}",
            response.status(),
            response.text().await.unwrap_or_default()
        )));
    }

    response
        .json()
        .await
        .map_err(|e| DeployError::ApiError(format!("Failed to parse response: {}", e)))
}

pub async fn create_deployment(request: CreateDeploymentRequest, config: &Config) -> Result<CreateDeploymentResponse, DeployError> {
    let auth_provider = DefaultAuthProvider;
    create_deployment_with_deps(request, config, &auth_provider).await
}

pub async fn update_deployment_with_deps<A: AuthProvider>(
    deployment_id: &str,
    request: UpdateDeploymentRequest,
    config: &Config,
    auth_provider: &A,
) -> Result<()> {
    let jwt_token = auth_provider
        .get_auth_token()
        .map_err(|e| DeployError::SessionError(format!("{}", e)))
        .context("Failed to load JWT token")?;
    let client = Client::new();
    let response = client
        .put(format!("{}/deploy/{}", config.api_url, deployment_id))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .json(&request)
        .send()
        .await
        .context("Failed to send update request")?;

    handle_api_response(response).await
}

pub async fn update_deployment(
    deployment_id: &str,
    request: UpdateDeploymentRequest,
    config: &Config,
) -> Result<()> {
    let auth_provider = DefaultAuthProvider;
    update_deployment_with_deps(deployment_id, request, config, &auth_provider).await
}

pub async fn list_deployments_with_deps<A: AuthProvider>(config: &Config, auth_provider: &A) -> Result<Vec<ListDeploymentsResponse>, DeployError> {
    let jwt_token = auth_provider
        .get_auth_token()
        .map_err(|e| DeployError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy", config.api_url))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
        .map_err(DeployError::RequestFailed)?;

    if response.status().is_success() {
        let deployments = response
            .json::<Vec<ListDeploymentsResponse>>()
            .await
            .map_err(|e| DeployError::ParseError(e.to_string()))?;
        println!("✅ Success: {} deployments retrieved", deployments.len());
        Ok(deployments)
    } else {
        Err(DeployError::from_response(response).await)
    }
}

pub async fn list_deployments(config: &Config) -> Result<Vec<ListDeploymentsResponse>, DeployError> {
    let auth_provider = DefaultAuthProvider;
    list_deployments_with_deps(config, &auth_provider).await
}

pub async fn get_deployment_with_deps<A: AuthProvider>(
    deployment_id: &str,
    config: &Config,
    auth_provider: &A,
) -> Result<GetDeploymentResponse, DeployError> {
    let jwt_token = auth_provider
        .get_auth_token()
        .map_err(|e| DeployError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy/{}", config.api_url, deployment_id))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
        .map_err(DeployError::RequestFailed)?;

    if response.status().is_success() {
        response
            .json::<GetDeploymentResponse>()
            .await
            .map_err(|e| DeployError::ParseError(e.to_string()))
    } else {
        Err(DeployError::from_response(response).await)
    }
}

pub async fn get_deployment(
    deployment_id: &str,
    config: &Config,
) -> Result<GetDeploymentResponse, DeployError> {
    let auth_provider = DefaultAuthProvider;
    get_deployment_with_deps(deployment_id, config, &auth_provider).await
}

pub async fn delete_deployment_with_deps<A: AuthProvider>(deployment_id: &str, config: &Config, auth_provider: &A) -> Result<()> {
    let jwt_token = auth_provider
        .get_auth_token()
        .map_err(|e| DeployError::SessionError(format!("{}", e)))
        .context("Failed to load JWT token")?;
    let client = Client::new();
    let response = client
        .delete(format!("{}/deploy/{}", config.api_url, deployment_id))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
        .context("Failed to send delete request")?;

    handle_api_response(response).await
}

pub async fn delete_deployment(deployment_id: &str, config: &Config) -> Result<()> {
    let auth_provider = DefaultAuthProvider;
    delete_deployment_with_deps(deployment_id, config, &auth_provider).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_deployments_session_error() {
        let config = Config { api_url: "https://api.guepard.run".to_string() };
        let mut auth = MockAuthProvider::new();
        auth
            .expect_get_auth_token()
            .times(1)
            .returning(|| Err(crate::domain::errors::config_error::ConfigError::SessionError(
                "You need to log in first! Run `guepard login` to get started. 🐆".to_string()
            )));

        let result = list_deployments_with_deps(&config, &auth).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            DeployError::SessionError(msg) => assert!(msg.contains("You need to log in first!")),
            _ => panic!("Expected SessionError"),
        }
    }

    #[tokio::test]
    async fn test_get_deployment_session_error() {
        let config = Config { api_url: "https://api.guepard.run".to_string() };
        let mut auth = MockAuthProvider::new();
        auth
            .expect_get_auth_token()
            .times(1)
            .returning(|| Err(crate::domain::errors::config_error::ConfigError::SessionError(
                "You need to log in first! Run `guepard login` to get started. 🐆".to_string()
            )));

        let result = get_deployment_with_deps("dep-1", &config, &auth).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            DeployError::SessionError(msg) => assert!(msg.contains("You need to log in first!")),
            _ => panic!("Expected SessionError"),
        }
    }

    #[tokio::test]
    async fn test_create_update_delete_network_or_api_error_paths() {
        let config = Config { api_url: "https://api.guepard.run".to_string() };
        let mut auth = MockAuthProvider::new();
        auth
            .expect_get_auth_token()
            .times(3)
            .returning(|| Ok("test-jwt-token".to_string()));

        // create
        let create_req = CreateDeploymentRequest {
            repository_name: "repo".to_string(),
            database_provider: "PostgreSQL".to_string(),
            database_version: "16".to_string(),
            deployment_type: "REPOSITORY".to_string(),
            region: "us-east".to_string(),
            datacenter: "iad1".to_string(),
            database_username: "user".to_string(),
            database_password: "pass".to_string(),
            performance_profile_id: "perf-1".to_string(),
        };
        let r1 = create_deployment_with_deps(create_req, &config, &auth).await;
        assert!(r1.is_err());

        // update
        let upd_req = UpdateDeploymentRequest { repository_name: "repo2".to_string() };
        let r2 = update_deployment_with_deps("dep-1", upd_req, &config, &auth).await;
        assert!(r2.is_err());

        // delete
        let r3 = delete_deployment_with_deps("dep-1", &config, &auth).await;
        assert!(r3.is_err());
    }
}