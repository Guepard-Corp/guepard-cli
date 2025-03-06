/// For API interactions.
use crate::application::dto::branch_dto::{BranchRequest, BranchResponse, ListBranchesResponse};
use crate::domain::errors::branch_error::BranchError;


use crate::config::config::Config;

use reqwest::Client;
/// Creates a new branch from a snapshot
pub async fn create_branch(
    deployment_id: &str,
    clone_id: &str,
    snapshot_id: &str,
    request: BranchRequest,
    config: &Config,
) -> Result<BranchResponse, BranchError> {


    let client = Client::new();
    let response = client
        .post(format!(
            "{}/deploy/{}/{}/{}/branch",
            config.api_url,deployment_id, clone_id, snapshot_id
        ))
        .header("Authorization", format!("Bearer {}", config.api_token))
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
pub async fn list_branches(deployment_id: &str, config: &Config) -> Result<Vec<ListBranchesResponse>, BranchError> {
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy/{}/clone", config.api_url,deployment_id))
        .header("Authorization", format!("Bearer {}", config.api_token))
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
    config: &Config,
) -> Result<BranchResponse, BranchError> {
    
    let client = Client::new();
    let response = client
        .post(format!(
            "{}/deploy/{}/{}/checkout",
            config.api_url, deployment_id, clone_id
        ))
        .header("Authorization", format!("Bearer {}", config.api_token))
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

