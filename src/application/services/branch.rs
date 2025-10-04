use crate::application::auth;
use crate::application::dto::branch::{BranchRequest, BranchResponse, ListBranchesResponse, CheckoutResponse};
use crate::config::config::Config;
use crate::domain::errors::branch_error::BranchError;
use reqwest::Client;

pub async fn create_branch(
    deployment_id: &str,
    branch_id: &str,
    snapshot_id: &str,
    request: BranchRequest,
    config: &Config,
) -> Result<BranchResponse, BranchError> {
    let jwt_token = auth::get_auth_token()
        .map_err(|e| BranchError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    let response = client
        .post(format!(
            "{}/deploy/{}/{}/{}/branch",
            config.api_url, deployment_id, branch_id, snapshot_id
        ))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .json(&serde_json::json!({
            "branch_name": request.branch_name,
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
    let jwt_token = auth::get_auth_token()
        .map_err(|e| BranchError::SessionError(format!("{}", e)))?;
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
    let jwt_token = auth::get_auth_token()
        .map_err(|e| BranchError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    let response = client
        .post(format!(
            "{}/deploy/{}/{}/checkout",
            config.api_url, deployment_id, branch_id
        ))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
        .map_err(BranchError::RequestFailed)?;

    if response.status().is_success() {
        let text = response.text().await.unwrap_or_default();
        
        // Try to parse as CheckoutResponse first (success case)
        if let Ok(checkout_response) = serde_json::from_str::<CheckoutResponse>(&text) {
            
            // Check if the body contains a message (error case)
            if checkout_response.body.contains("already checked out") {
                // Return a mock response for already checked out case
                Ok(BranchResponse {
                    id: branch_id.to_string(),
                    account_id: "".to_string(),
                    label_name: "Already checked out".to_string(),
                    job_status: "ALREADY_CHECKED_OUT".to_string(),
                    compute_status: "RUNNING".to_string(),
                    deployment_id: deployment_id.to_string(),
                    branch_id: branch_id.to_string(),
                    performance_profile_id: "".to_string(),
                    updated_at: None,
                    created_at: None,
                    created_by: None,
                    updated_by: None,
                    port: 0,
                })
            } else {
                // Parse the body string as BranchResponse
                serde_json::from_str::<BranchResponse>(&checkout_response.body)
                    .map_err(|e| BranchError::ParseError(e.to_string()))
            }
        } else if text.contains("already checked out") {
            // Return a mock response for already checked out case
            Ok(BranchResponse {
                id: branch_id.to_string(),
                account_id: "".to_string(),
                label_name: "Already checked out".to_string(),
                job_status: "ALREADY_CHECKED_OUT".to_string(),
                compute_status: "RUNNING".to_string(),
                deployment_id: deployment_id.to_string(),
                branch_id: branch_id.to_string(),
                performance_profile_id: "".to_string(),
                updated_at: None,
                created_at: None,
                created_by: None,
                updated_by: None,
                port: 0,
            })
        } else {
            Err(BranchError::ParseError(format!("Unexpected response format: {}", text)))
        }
    } else {
        Err(BranchError::from_response(response).await)
    }
}