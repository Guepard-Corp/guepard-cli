use crate::application::dto::bookmark_dto::{GetBookmarkResponse, CreateBookmarkRequest, CreateBookmarkResponse, CheckoutBookmarkResponse};
use crate::config::config::Config;
use crate::domain::errors::bookmark_error::BookmarkError;
use crate::application::dto::branch_dto::BranchRequest; 
use anyhow::Result;
use reqwest::Client;

pub async fn list_all_bookmarks(deployment_id: &str, config: &Config) -> Result<Vec<GetBookmarkResponse>, BookmarkError> {
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy/{}/snap", config.api_url, deployment_id))
        .header("Authorization", format!("Bearer {}", config.api_token))
        .send()
        .await
        .map_err(BookmarkError::RequestFailed)?;

    if response.status().is_success() {
        response.json::<Vec<GetBookmarkResponse>>()
            .await
            .map_err(|e| BookmarkError::ParseError(e.to_string()))
    } else {
        Err(BookmarkError::from_response(response).await)
    }
}

pub async fn list_bookmark(deployment_id: &str, clone_id: &str, config: &Config) -> Result<Vec<GetBookmarkResponse>, BookmarkError> {
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy/{}/{}/snap", config.api_url, deployment_id, clone_id))
        .header("Authorization", format!("Bearer {}", config.api_token))
        .send()
        .await
        .map_err(BookmarkError::RequestFailed)?;

    if response.status().is_success() {
        response.json::<Vec<GetBookmarkResponse>>()
            .await
            .map_err(|e| BookmarkError::ParseError(e.to_string()))
    } else {
        Err(BookmarkError::from_response(response).await)
    }
}

pub async fn create_bookmark(
    deployment_id: &str,
    clone_id: &str,
    request: CreateBookmarkRequest,
    config: &Config,
) -> Result<CreateBookmarkResponse, BookmarkError> {
    let client = Client::new();
    let response = client
        .put(format!("{}/deploy/{}/{}/snap", config.api_url, deployment_id, clone_id))
        .header("Authorization", format!("Bearer {}", config.api_token))
        .json(&request)
        .send()
        .await
        .map_err(BookmarkError::RequestFailed)?;

    if response.status().is_success() {
        response.json::<CreateBookmarkResponse>()
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
) -> Result<CheckoutBookmarkResponse, BookmarkError> { 
    let client = Client::new();
    let response = client
        .post(format!(
            "{}/deploy/{}/{}/{}/branch",
            config.api_url, deployment_id, clone_id, snapshot_id
        ))
        .header("Authorization", format!("Bearer {}", config.api_token))
        .json(&request)
        .send()
        .await
        .map_err(BookmarkError::RequestFailed)?;

    if response.status().is_success() {
        response.json::<CheckoutBookmarkResponse>()
            .await
            .map_err(|e| BookmarkError::ParseError(e.to_string()))
    } else {
        Err(BookmarkError::from_response(response).await)
    }
}