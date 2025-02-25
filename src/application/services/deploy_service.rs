use crate::application::dto::deploy_dto::{CreateDeploymentRequest, UpdateDeploymentRequest};
use anyhow::{Context, Result};
use dotenvy::var;
use reqwest::Client;

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

    if response.status().is_success() {
        Ok(())
    } else {
        let err_msg = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(anyhow::anyhow!("Deployment failed: {}", err_msg))
    }
}

pub async fn update_deployment(deployment_id: &str, request: UpdateDeploymentRequest) -> Result<()> {
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

    if response.status().is_success() {
        Ok(())
    } else {
        let err_msg = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(anyhow::anyhow!("Update failed: {}", err_msg))
    }
}
