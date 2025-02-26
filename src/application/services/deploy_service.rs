use crate::{
    application::dto::deploy_dto::{CreateDeploymentRequest, UpdateDeploymentRequest},
    domain::errors::deploy_error,
};
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

    deploy_error::handle_api_response(response).await
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

    deploy_error::handle_api_response(response).await
}
