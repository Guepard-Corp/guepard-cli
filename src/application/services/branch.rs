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
        let text = response.text().await.unwrap_or_default();
        
        // Try to parse as CheckoutResponse first
        if let Ok(checkout_response) = serde_json::from_str::<CheckoutResponse>(&text) {
            // Check the internal status code
            if checkout_response.status_code >= 400 {
                return Err(BranchError::ApiError(checkout_response.body));
            }

            // Parse the body string as BranchResponse
            serde_json::from_str::<BranchResponse>(&checkout_response.body)
                .map_err(|e| BranchError::ParseError(format!("Failed to parse body as BranchResponse: {}. Body: {}", e, checkout_response.body)))
        } else {
            // Try to parse directly as BranchResponse (fallback)
            serde_json::from_str::<BranchResponse>(&text)
                .map_err(|e| BranchError::ParseError(e.to_string()))
        }
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

pub async fn checkout_branch(deployment_id: &str, branch_id: &str, request: Option<crate::application::dto::branch::CheckoutRequest>, config: &Config) -> Result<BranchResponse, BranchError> {
    let jwt_token = auth::get_auth_token()
        .map_err(|e| BranchError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    let mut builder = client
        .post(format!(
            "{}/deploy/{}/{}/checkout",
            config.api_url, deployment_id, branch_id
        ))
        .header("Authorization", format!("Bearer {}", jwt_token));

    if let Some(req) = request {
        builder = builder.json(&req);
    }

    let response = builder.send()
        .await
        .map_err(BranchError::RequestFailed)?;

    if response.status().is_success() {
        let text = response.text().await.unwrap_or_default();
        
        // Try to parse as CheckoutResponse first
        if let Ok(checkout_response) = serde_json::from_str::<CheckoutResponse>(&text) {
            // Check the internal status code
            if checkout_response.status_code >= 400 {
                return Err(BranchError::ApiError(checkout_response.body));
            }

            // Check if the body contains a message (already checked out case)
            if checkout_response.body.contains("already checked out") {
                return Ok(BranchResponse {
                    id: branch_id.to_string(),
                    account_id: None,
                    label_name: Some("Already checked out".to_string()),
                    job_status: Some("ALREADY_CHECKED_OUT".to_string()),
                    deployment_id: Some(deployment_id.to_string()),
                    branch_id: Some(branch_id.to_string()),
                    snapshot_id: None,
                    is_ephemeral: None,
                    is_masked: None,
                    is_purged: None,
                    updated_at: None,
                    created_at: None,
                    created_by: None,
                    updated_by: None,
                });
            }

            // Parse the body string as BranchResponse
            serde_json::from_str::<BranchResponse>(&checkout_response.body)
                .map_err(|e| BranchError::ParseError(format!("Failed to parse body as BranchResponse: {}. Body: {}", e, checkout_response.body)))
        } else if text.contains("already checked out") {
            // Return a mock response for already checked out case
            Ok(BranchResponse {
                id: branch_id.to_string(),
                account_id: None,
                label_name: Some("Already checked out".to_string()),
                job_status: Some("ALREADY_CHECKED_OUT".to_string()),
                deployment_id: Some(deployment_id.to_string()),
                branch_id: Some(branch_id.to_string()),
                snapshot_id: None,
                is_ephemeral: None,
                is_masked: None,
                is_purged: None,
                updated_at: None,
                created_at: None,
                created_by: None,
                updated_by: None,
            })
        } else {
            Err(BranchError::ParseError(format!("Unexpected response format: {}", text)))
        }
    } else {
        Err(BranchError::from_response(response).await)
    }
}

pub async fn checkout_snapshot(deployment_id: &str, branch_id: &str, snapshot_id: &str, config: &Config) -> Result<BranchResponse, BranchError> {
    let jwt_token = auth::get_auth_token()
        .map_err(|e| BranchError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    
    // User instruction: {{base_url}}/deploy/{{deployment_id}}/{{branch_id}}/{{snapshot_id}}/branch
    let response = client
        .post(format!(
            "{}/deploy/{}/{}/{}/branch",
            config.api_url, deployment_id, branch_id, snapshot_id
        ))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .json(&serde_json::json!({
            "checkout": true,
            "discard_changes": "false"
        }))
        .send()
        .await
        .map_err(BranchError::RequestFailed)?;

    if response.status().is_success() {
        let text = response.text().await.unwrap_or_default();
        
        if let Ok(checkout_response) = serde_json::from_str::<CheckoutResponse>(&text) {
            // Check the internal status code
            if checkout_response.status_code >= 400 {
                return Err(BranchError::ApiError(checkout_response.body));
            }

            serde_json::from_str::<BranchResponse>(&checkout_response.body)
                .map_err(|e| BranchError::ParseError(format!("Failed to parse body as BranchResponse: {}. Body: {}", e, checkout_response.body)))
        } else {
            serde_json::from_str::<BranchResponse>(&text)
                .map_err(|e| BranchError::ParseError(e.to_string()))
        }
    } else {
        Err(BranchError::from_response(response).await)
    }
}