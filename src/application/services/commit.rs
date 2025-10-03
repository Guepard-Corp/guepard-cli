use crate::application::auth;
use crate::application::dto::commit::{GetCommitResponse, CreateCommitRequest, CreateCommitResponse, CheckoutCommitResponse};
use crate::application::dto::branch::BranchRequest;
use crate::config::config::Config;
use crate::domain::errors::bookmark_error::BookmarkError;
use anyhow::Result;
use reqwest::Client;

pub async fn list_all_commits(deployment_id: &str, config: &Config) -> Result<Vec<GetCommitResponse>, BookmarkError> {
    let jwt_token = auth::get_auth_token()
        .map_err(|e| BookmarkError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy/{}/snap", config.api_url, deployment_id))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
        .map_err(BookmarkError::RequestFailed)?;

    if response.status().is_success() {
        response.json::<Vec<GetCommitResponse>>()
            .await
            .map_err(|e| BookmarkError::ParseError(e.to_string()))
    } else {
        Err(BookmarkError::from_response(response).await)
    }
}

pub async fn list_bookmark(deployment_id: &str, clone_id: &str, config: &Config) -> Result<Vec<GetCommitResponse>, BookmarkError> {
    let jwt_token = auth::get_auth_token()
        .map_err(|e| BookmarkError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy/{}/{}/snap", config.api_url, deployment_id, clone_id))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
        .map_err(BookmarkError::RequestFailed)?;

    if response.status().is_success() {
        response.json::<Vec<GetCommitResponse>>()
            .await
            .map_err(|e| BookmarkError::ParseError(e.to_string()))
    } else {
        Err(BookmarkError::from_response(response).await)
    }
}

pub async fn create_commit(
    deployment_id: &str,
    clone_id: &str,
    request: CreateCommitRequest,
    config: &Config,
) -> Result<CreateCommitResponse, BookmarkError> {
    let jwt_token = auth::get_auth_token()
        .map_err(|e| BookmarkError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    let response = client
        .put(format!("{}/deploy/{}/{}/snap", config.api_url, deployment_id, clone_id))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .json(&request)
        .send()
        .await
        .map_err(BookmarkError::RequestFailed)?;

    if response.status().is_success() {
        response.json::<CreateCommitResponse>()
            .await
            .map_err(|e| BookmarkError::ParseError(e.to_string()))
    } else {
        Err(BookmarkError::from_response(response).await)
    }
}

pub async fn checkout_bookmark(
    deployment_id: &str,
    clone_id: &str,
    snapshot_id: &str,
    request: BranchRequest,
    config: &Config,
) -> Result<CheckoutCommitResponse, BookmarkError> {
    let jwt_token = auth::get_auth_token()
        .map_err(|e| BookmarkError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    let response = client
        .post(format!(
            "{}/deploy/{}/{}/{}/branch",
            config.api_url, deployment_id, clone_id, snapshot_id
        ))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .json(&request)
        .send()
        .await
        .map_err(BookmarkError::RequestFailed)?;

    if response.status().is_success() {
        response.json::<CheckoutCommitResponse>()
            .await
            .map_err(|e| BookmarkError::ParseError(e.to_string()))
    } else {
        Err(BookmarkError::from_response(response).await)
    }
}