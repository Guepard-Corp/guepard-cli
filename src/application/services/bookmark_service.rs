use crate::application::dto::bookmark_dto::{GetBookmarkResponse, CreateBookmarkRequest, CreateBookmarkResponse, CheckoutBookmarkResponse};
use crate::application::dto::branch_dto::BranchRequest;
use crate::config::config::{self, Config};
use crate::domain::errors::bookmark_error::BookmarkError;
use anyhow::Result;
use reqwest::Client;

pub async fn list_all_bookmarks(deployment_id: &str, config: &Config) -> Result<Vec<GetBookmarkResponse>, BookmarkError> {
    let jwt_token = config::load_jwt_token()
        .map_err(|e| BookmarkError::SessionError(e.to_string()))?;
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy/{}/snap", config.api_url, deployment_id))
        .header("Authorization", format!("Bearer {}", jwt_token))
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
    let jwt_token = config::load_jwt_token()
        .map_err(|e| BookmarkError::SessionError(e.to_string()))?;
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy/{}/{}/snap", config.api_url, deployment_id, clone_id))
        .header("Authorization", format!("Bearer {}", jwt_token))
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
    branch_id: &str,
    request: CreateBookmarkRequest,
    config: &Config,
) -> Result<CreateBookmarkResponse, BookmarkError> {
    let jwt_token = config::load_jwt_token()
        .map_err(|e| BookmarkError::SessionError(e.to_string()))?;
    let client = Client::new();
    let response = client
        .post(format!("{}/deploy/{}/{}/snap", config.api_url, deployment_id, branch_id))
        .header("Authorization", format!("Bearer {}", jwt_token))
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
    let jwt_token = config::load_jwt_token()
        .map_err(|e| BookmarkError::SessionError(e.to_string()))?;
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
        response.json::<CheckoutBookmarkResponse>()
            .await
            .map_err(|e| BookmarkError::ParseError(e.to_string()))
    } else {
        Err(BookmarkError::from_response(response).await)
    }
}