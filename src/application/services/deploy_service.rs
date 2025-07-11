use anyhow::{Context, Result};
use reqwest::Client;

use crate::application::dto::deploy_dto::{
    CreateDeploymentRequest, CreateDeploymentResponse,GetDeploymentResponse, ListDeploymentsResponse,
    UpdateDeploymentRequest,
};
use crate::config::config::{self, Config};
use crate::domain::errors::deploy_error::{handle_api_response, DeployError};

pub async fn create_deployment(request: CreateDeploymentRequest, config: &Config) -> Result<CreateDeploymentResponse, DeployError> {
    let jwt_token = config::load_jwt_token()
        .map_err(|e| DeployError::SessionError(e.to_string()))?;
    let client = Client::new();
    let response = client
        .post(format!("{}/deploy", config.api_url))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .json(&request)
        .send()
        .await
        .map_err(DeployError::RequestFailed)?;

    if response.status().is_success() {
        response
            .json::<CreateDeploymentResponse>()
            .await
            .map_err(|e| DeployError::ParseError(e.to_string()))
    } else {
        Err(DeployError::from_response(response).await)
    }
}

pub async fn update_deployment(
    deployment_id: &str,
    request: UpdateDeploymentRequest,
    config: &Config,
) -> Result<()> {
    let jwt_token = config::load_jwt_token()
        .map_err(|e| DeployError::SessionError(e.to_string()))
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

pub async fn list_deployments(config: &Config) -> Result<Vec<ListDeploymentsResponse>, DeployError> {
    let jwt_token = config::load_jwt_token()
        .map_err(|e| DeployError::SessionError(e.to_string()))?;
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy", config.api_url))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
        .map_err(DeployError::RequestFailed)?;

    if response.status().is_success() {
        response
            .json::<Vec<ListDeploymentsResponse>>()
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
    let jwt_token = config::load_jwt_token()
        .map_err(|e| DeployError::SessionError(e.to_string()))?;
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