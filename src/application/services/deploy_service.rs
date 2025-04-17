use anyhow::{Context, Result};
use reqwest::Client;

use crate::application::dto::deploy_dto::{
    CreateDeploymentRequest, GetDeploymentResponse, ListDeploymentsResponse,
    UpdateDeploymentRequest,
};
use crate::config::config::Config;
use crate::domain::errors::deploy_error::{handle_api_response, DeployError};

pub async fn create_deployment(request: CreateDeploymentRequest, config: &Config) -> Result<()> {
    let client = Client::new();
    let response = client
        .post(format!("{}/deploy", config.api_url))
        .header("Authorization", format!("Bearer {}", config.api_token))
        .json(&request)
        .send()
        .await?;

    handle_api_response(response).await
}

pub async fn update_deployment(
    deployment_id: &str,
    request: UpdateDeploymentRequest,
    config: &Config,
) -> Result<()> {
    let client = Client::new();
    let response = client
        .put(format!("{}/deploy/{}", config.api_url, deployment_id))
        .header("Authorization", format!("Bearer {}", config.api_token))
        .json(&request)
        .send()
        .await
        .context("Failed to send update request")?;

    handle_api_response(response).await
}

pub async fn list_deployments(config: &Config) -> Result<Vec<ListDeploymentsResponse>, DeployError> {
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy", config.api_url))
        .header("Authorization", format!("Bearer {}", config.api_token))
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

pub async fn get_deployment(
    deployment_id: &str,
    config: &Config,
) -> Result<GetDeploymentResponse, DeployError> {
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy/{}", config.api_url, deployment_id))
        .header("Authorization", format!("Bearer {}", config.api_token))
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
