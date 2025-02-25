use crate::application::dto::deploy_dto::{CreateDeploymentRequest, UpdateDeploymentRequest};
use anyhow::{Context, Result};
use dotenvy::var;
use reqwest::Client;

/// handles API responses
async fn handle_api_response(response: reqwest::Response) -> Result<()> {
    match response.status() {
        reqwest::StatusCode::OK => {
            let text = response.text().await.unwrap_or_else(|_| "Success".to_string());
            println!("✅ Success: {}", text);
            Ok(())
        }
        reqwest::StatusCode::BAD_REQUEST => {
            let err_msg = response.text().await.unwrap_or("Invalid request data".to_string());
            Err(anyhow::anyhow!("❌ 400 Bad Request: {}", err_msg))
        }
        reqwest::StatusCode::FORBIDDEN => {
            Err(anyhow::anyhow!("❌ 403 Forbidden: Invalid API token or permissions"))
        }
        reqwest::StatusCode::SERVICE_UNAVAILABLE => {
            Err(anyhow::anyhow!("❌ 503 Service Unavailable: The API is currently down"))
        }
        _ => {
            let err_msg = response.text().await.unwrap_or("Unknown error".to_string());
            Err(anyhow::anyhow!("❌ Unexpected Error: {}", err_msg))
        }
    }
}

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

    handle_api_response(response).await
}
