use crate::application::dto::branch::{BranchRequest, BranchResponse, ListBranchesResponse};
use crate::config::config::{self, Config};
use crate::domain::errors::branch_error::BranchError;
use reqwest::Client;

pub async fn create_branch(
    deployment_id: &str,
    snapshot_id: &str,
    request: BranchRequest,
    config: &Config,
) -> Result<BranchResponse, BranchError> {
    let jwt_token = config::load_jwt_token()
        .map_err(|e| BranchError::SessionError(e.to_string()))?;
    let client = Client::new();
    let response = client
        .post(format!(
            "{}/deploy/{}/branch",
            config.api_url, deployment_id
        ))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .json(&serde_json::json!({
            "snapshot_id": snapshot_id,
            "discard_changes": request.discard_changes,
            "checkout": request.checkout,
            "ephemeral": request.ephemeral
        }))
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

pub async fn list_branches(deployment_id: &str, config: &Config) -> Result<Vec<ListBranchesResponse>, BranchError> {
    let jwt_token = config::load_jwt_token()
        .map_err(|e| BranchError::SessionError(e.to_string()))?;
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy/{}/branch", config.api_url, deployment_id))
        .header("Authorization", format!("Bearer {}", jwt_token))
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

pub async fn checkout_branch(deployment_id: &str, branch_id: &str, config: &Config) -> Result<BranchResponse, BranchError> {
    let jwt_token = config::load_jwt_token()
        .map_err(|e| BranchError::SessionError(e.to_string()))?;
    let client = Client::new();
    let response = client
        .post(format!(
            "{}/deploy/{}/checkout",
            config.api_url, deployment_id
        ))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .json(&serde_json::json!({
            "branch_id": branch_id
        }))
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