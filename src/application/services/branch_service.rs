/// For API interactions.
use crate::application::dto::branch_dto::{BranchRequest, BranchResponse, ListBranchesResponse};
use crate::domain::errors::branch_error::BranchError;

use anyhow::{Context, Result};
use dotenvy::var;
use reqwest::Client;
/// Creates a new branch from a snapshot
pub async fn create_branch(
    deployment_id: &str,
    clone_id: &str,
    snapshot_id: &str,
    request: BranchRequest,
) -> Result<BranchResponse, BranchError> {
    let api_url = var("PUBLIC_API_DEPLOY")
        .context("Missing API URL in .env")
        .map_err(|e| BranchError::ApiError(e.to_string()))?;
    let token = var("API_TOKEN")
        .context("Missing API token in .env")
        .map_err(|e| BranchError::ApiError(e.to_string()))?;

    let client = Client::new();
    let response = client
        .post(format!(
            "{}/deploy/{}/{}/{}/branch",
            api_url, deployment_id, clone_id, snapshot_id
        ))
        .header("Authorization", format!("Bearer {}", token))
        .json(&request)
        .send()
        .await
        .map_err(BranchError::RequestFailed)?;

    if response.status().is_success() {
        response
            .json::<BranchResponse>()
            .await
            .map_err(|e| BranchError::ParseError(e.to_string()))
    } else {
        Err(BranchError::from_response(response).await) // Updated to use server message
    }
}

/// Lists all clones/branches for a deployment
pub async fn list_branches(deployment_id: &str) -> Result<Vec<ListBranchesResponse>, BranchError> {
    let api_url = var("PUBLIC_API_DEPLOY")
        .context("Missing API URL in .env")
        .map_err(|e| BranchError::ApiError(e.to_string()))?;
    let token = var("API_TOKEN")
        .context("Missing API token in .env")
        .map_err(|e| BranchError::ApiError(e.to_string()))?;

    let client = Client::new();
    let response = client
        .get(format!("{}/deploy/{}/clone", api_url, deployment_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(BranchError::RequestFailed)?;

    if response.status().is_success() {
        response
            .json::<Vec<ListBranchesResponse>>()
            .await
            .map_err(|e| BranchError::ParseError(e.to_string()))
    } else {
        Err(BranchError::from_response(response).await) 
    }
}

/// Checks out a branch
pub async fn checkout_branch(
    deployment_id: &str,
    clone_id: &str,
) -> Result<BranchResponse, BranchError> {
    let api_url = var("PUBLIC_API_DEPLOY")
        .context("Missing API URL in .env")
        .map_err(|e| BranchError::ApiError(e.to_string()))?;
    let token = var("API_TOKEN")
        .context("Missing API token in .env")
        .map_err(|e| BranchError::ApiError(e.to_string()))?;

    let client = Client::new();
    let response = client
        .post(format!(
            "{}/deploy/{}/{}/checkout",
            api_url, deployment_id, clone_id
        ))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(BranchError::RequestFailed)?;

    if response.status().is_success() {
        response
            .json::<BranchResponse>()
            .await
            .map_err(|e| BranchError::ParseError(e.to_string()))
    } else {
        Err(BranchError::from_response(response).await) // Updated error handling
    }
}

/// Updates an existing branch
pub async fn update_branch(
    deployment_id: &str,
    clone_id: &str,
    snapshot_id: &str,
    request: BranchRequest,
) -> Result<BranchResponse, BranchError> {
    let api_url = var("PUBLIC_API_DEPLOY")
        .context("Missing API URL in .env")
        .map_err(|e| BranchError::ApiError(e.to_string()))?;
    let token = var("API_TOKEN")
        .context("Missing API token in .env")
        .map_err(|e| BranchError::ApiError(e.to_string()))?;
    
    let client = Client::new();
    let response = client
        .post(format!(
            "{}/deploy/{}/{}/{}/branch",
            api_url, deployment_id, clone_id, snapshot_id
        ))
        .header("Authorization", format!("Bearer {}", token))
        .json(&request)
        .send()
        .await
        .map_err(BranchError::RequestFailed)?;

    if response.status().is_success() {
        response
            .json::<BranchResponse>()
            .await
            .map_err(|e| BranchError::ParseError(e.to_string()))
    } else {
        Err(BranchError::from_response(response).await)
    }
}
