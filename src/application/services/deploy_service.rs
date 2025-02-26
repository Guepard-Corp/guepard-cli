use crate::application::dto::deploy_dto::{
    CreateDeploymentRequest, ListDeploymentsResponse, UpdateDeploymentRequest,
};
use crate::domain::errors::deploy_error::handle_api_response;
use crate::domain::errors::deploy_error::DeployError;


use anyhow::{Context, Result};
use dotenvy::var;
use reqwest::Client;

/// Creates a new deployment
pub async fn create_deployment(request: CreateDeploymentRequest) -> Result<()> {
    let api_url = var("PUBLIC_API_DEPLOY").context("Missing API URL in .env")?;
    let token = var("API_TOKEN").context("Missing API token in .env")?;

    let client = Client::new();
    let response = client
        .post(format!("{}/deploy", api_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&request)
        .send()
        .await
        .context("Failed to send deployment request")?;

    handle_api_response(response).await
}

/// Updates an existing deployment
pub async fn update_deployment(
    deployment_id: &str,
    request: UpdateDeploymentRequest,
) -> Result<()> {
    let api_url = var("PUBLIC_API_DEPLOY").context("Missing API URL in .env")?;
    let token = var("API_TOKEN").context("Missing API token in .env")?;

    let client = Client::new();
    let response = client
        .put(format!("{}/deploy/{}", api_url, deployment_id))
        .header("Authorization", format!("Bearer {}", token))
        .json(&request)
        .send()
        .await
        .context("Failed to send update request")?;

    handle_api_response(response).await
}

pub async fn list_deployments() -> Result<Vec<ListDeploymentsResponse>, DeployError> {
    let api_url = var("PUBLIC_API_DEPLOY")
        .context("Missing API URL in .env")
        .map_err(|e| DeployError::ApiError(e.to_string()))?;
    let token = var("API_TOKEN")
        .context("Missing API token in .env")
        .map_err(|e| DeployError::ApiError(e.to_string()))?;

    let client = Client::new();
    let response = client
        .get(format!("{}/deploy", api_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(DeployError::RequestFailed)?;

        if response.status().is_success() {
            let deployments = response.json::<Vec<ListDeploymentsResponse>>()
                .await
                .map_err(|e| DeployError::ParseError(e.to_string()))?;
            println!("✅ Success: {} deployments retrieved", deployments.len());
            Ok(deployments)
        } else {
            Err(DeployError::from_response(response).await)
        }
}