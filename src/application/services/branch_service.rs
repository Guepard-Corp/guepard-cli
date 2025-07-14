use crate::application::dto::branch_dto::{BranchRequest, BranchResponse, ListBranchesResponse};
use crate::config::config::{self, Config};
use crate::domain::errors::branch_error::BranchError;
use reqwest::Client;

pub async fn create_branch(
    deployment_id: &str,
    branch_id: &str,
    snapshot_id: &str,
    request: BranchRequest,
    config: &Config,
) -> Result<BranchResponse, BranchError> {
    let jwt_token = config::load_jwt_token()
        .map_err(|e| BranchError::SessionError(e.to_string()))?;
    let client = Client::new();
    let response = client
        .post(format!(
            "{}/deploy/{}/{}/{}/branch",
            config.api_url, deployment_id, branch_id, snapshot_id
        ))
        .header("Authorization", format!("Bearer {}", jwt_token))
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

pub async fn checkout_branch(
    deployment_id: &str,
    clone_id: &str,
    config: &Config,
) -> Result<BranchResponse, BranchError> {
    let jwt_token = config::load_jwt_token()
        .map_err(|e| BranchError::SessionError(e.to_string()))?;
    let client = Client::new();
    let response = client
        .post(format!(
            "{}/deploy/{}/{}/checkout",
            config.api_url, deployment_id, clone_id
        ))
        .header("Authorization", format!("Bearer {}", jwt_token))
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