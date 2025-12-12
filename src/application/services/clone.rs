use crate::application::auth;
use crate::application::dto::clone::{CreateCloneRequest, CreateCloneResponse, ListClonesResponse};
use crate::config::config::Config;
use crate::domain::errors::deploy_error::DeployError;
use reqwest::Client;

pub async fn create_clone(deployment_id: &str, snapshot_id: &str, request: CreateCloneRequest, config: &Config) -> Result<CreateCloneResponse, DeployError> {
    let jwt_token = auth::get_auth_token()
        .map_err(|e| DeployError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    let url = format!("{}/deploy/{}/snapshot/{}/shadow", config.api_url, deployment_id, snapshot_id);
    
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", jwt_token))
        .json(&request)
        .send()
        .await
        .map_err(|e| DeployError::ApiError(format!("Network error: {}", e)))?;

    if !response.status().is_success() {
        return Err(DeployError::from_response(response).await);
    }

    response
        .json()
        .await
        .map_err(|e| DeployError::ApiError(format!("Failed to parse response: {}", e)))
}

pub async fn list_clones(deployment_id: &str, config: &Config) -> Result<ListClonesResponse, DeployError> {
    let jwt_token = auth::get_auth_token()
        .map_err(|e| DeployError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    let url = format!("{}/deploy/{}/shadow", config.api_url, deployment_id);
    
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
        .map_err(|e| DeployError::ApiError(format!("Network error: {}", e)))?;

    if !response.status().is_success() {
        return Err(DeployError::from_response(response).await);
    }

    response
        .json()
        .await
        .map_err(|e| DeployError::ApiError(format!("Failed to parse response: {}", e)))
}

