use crate::application::auth;
use crate::application::dto::commit::{CreateCommitRequest, CreateCommitResponse, GetCommitResponse, CheckoutCommitResponse};
use crate::application::dto::branch::BranchRequest;
use crate::config::config::Config;
use crate::domain::errors::bookmark_error::BookmarkError;
use anyhow::Result;
use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;

// Trait for dependency injection to make testing easier
#[cfg_attr(test, mockall::automock)]
pub trait AuthProvider {
    fn get_auth_token(&self) -> Result<String, crate::domain::errors::config_error::ConfigError>;
}

// Default implementation that uses the real auth module
pub struct DefaultAuthProvider;

impl AuthProvider for DefaultAuthProvider {
    fn get_auth_token(&self) -> Result<String, crate::domain::errors::config_error::ConfigError> {
        auth::get_auth_token()
    }
}

pub async fn list_all_commits_with_deps<A: AuthProvider>(deployment_id: &str, config: &Config, auth_provider: &A) -> Result<Vec<GetCommitResponse>, BookmarkError> {
    let jwt_token = auth_provider
        .get_auth_token()
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

pub async fn list_all_commits(deployment_id: &str, config: &Config) -> Result<Vec<GetCommitResponse>, BookmarkError> {
    let auth_provider = DefaultAuthProvider;
    list_all_commits_with_deps(deployment_id, config, &auth_provider).await
}

pub async fn list_bookmark_with_deps<A: AuthProvider>(deployment_id: &str, branch_id: &str, config: &Config, auth_provider: &A) -> Result<Vec<GetCommitResponse>, BookmarkError> {
    let jwt_token = auth_provider
        .get_auth_token()
        .map_err(|e| BookmarkError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy/{}/{}/snap", config.api_url, deployment_id, branch_id))
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

pub async fn list_bookmark(deployment_id: &str, branch_id: &str, config: &Config) -> Result<Vec<GetCommitResponse>, BookmarkError> {
    let auth_provider = DefaultAuthProvider;
    list_bookmark_with_deps(deployment_id, branch_id, config, &auth_provider).await
}

const POLL_INTERVAL: Duration = Duration::from_secs(3);
const POLL_TIMEOUT: Duration = Duration::from_secs(120);

fn is_bookmark_init_error(msg: &str) -> bool {
    msg.contains("Cannot create new bookmark")
        && msg.contains("INIT")
        && msg.contains("needs healing")
}

async fn wait_for_snapshots_ready<A: AuthProvider>(
    deployment_id: &str,
    branch_id: &str,
    config: &Config,
    auth_provider: &A,
) -> Result<(), BookmarkError> {
    let deadline = tokio::time::Instant::now() + POLL_TIMEOUT;
    while tokio::time::Instant::now() < deadline {
        let snapshots = list_bookmark_with_deps(deployment_id, branch_id, config, auth_provider).await?;
        let all_ready = snapshots.iter().all(|s| s.status == "CREATED" || s.status == "COMPLETED");
        if all_ready {
            return Ok(());
        }
        sleep(POLL_INTERVAL).await;
    }
    Err(BookmarkError::InternalServerError(
        "Timeout waiting for previous snapshot to complete. Please retry later.".to_string(),
    ))
}

pub async fn create_commit_with_deps<A: AuthProvider>(
    deployment_id: &str,
    branch_id: &str,
    request: CreateCommitRequest,
    config: &Config,
    auth_provider: &A,
) -> Result<CreateCommitResponse, BookmarkError> {
    let result = create_commit_once(deployment_id, branch_id, &request, config, auth_provider).await;
    match result {
        Ok(resp) => Ok(resp),
        Err(BookmarkError::InternalServerError(ref msg)) if is_bookmark_init_error(msg) => {
            wait_for_snapshots_ready(deployment_id, branch_id, config, auth_provider).await?;
            create_commit_once(deployment_id, branch_id, &request, config, auth_provider).await
        }
        Err(e) => Err(e),
    }
}

async fn create_commit_once<A: AuthProvider>(
    deployment_id: &str,
    branch_id: &str,
    request: &CreateCommitRequest,
    config: &Config,
    auth_provider: &A,
) -> Result<CreateCommitResponse, BookmarkError> {
    let jwt_token = auth_provider
        .get_auth_token()
        .map_err(|e| BookmarkError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    let response = client
        .post(format!("{}/deploy/{}/{}/snap", config.api_url, deployment_id, branch_id))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .json(request)
        .send()
        .await
        .map_err(BookmarkError::RequestFailed)?;

    if response.status().is_success() {
        response
            .json::<CreateCommitResponse>()
            .await
            .map_err(|e| BookmarkError::ParseError(e.to_string()))
    } else {
        Err(BookmarkError::from_response(response).await)
    }
}

pub async fn create_commit(
    deployment_id: &str,
    branch_id: &str,
    request: CreateCommitRequest,
    config: &Config,
) -> Result<CreateCommitResponse, BookmarkError> {
    let auth_provider = DefaultAuthProvider;
    create_commit_with_deps(deployment_id, branch_id, request, config, &auth_provider).await
}

pub async fn checkout_bookmark_with_deps<A: AuthProvider>(
    deployment_id: &str,
    branch_id: &str,
    snapshot_id: &str,
    request: BranchRequest,
    config: &Config,
    auth_provider: &A,
) -> Result<CheckoutCommitResponse, BookmarkError> {
    let jwt_token = auth_provider
        .get_auth_token()
        .map_err(|e| BookmarkError::SessionError(format!("{}", e)))?;
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
        .map_err(BookmarkError::RequestFailed)?;

    if response.status().is_success() {
        response.json::<CheckoutCommitResponse>()
            .await
            .map_err(|e| BookmarkError::ParseError(e.to_string()))
    } else {
        Err(BookmarkError::from_response(response).await)
    }
}

pub async fn checkout_bookmark(
    deployment_id: &str,
    branch_id: &str,
    snapshot_id: &str,
    request: BranchRequest,
    config: &Config,
) -> Result<CheckoutCommitResponse, BookmarkError> {
    let auth_provider = DefaultAuthProvider;
    checkout_bookmark_with_deps(deployment_id, branch_id, snapshot_id, request, config, &auth_provider).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_all_commits_session_error() {
        let config = Config { api_url: "https://api.guepard.run".to_string(), app_url: "https://app.guepard.run".to_string() };
        let mut auth = MockAuthProvider::new();
        auth
            .expect_get_auth_token()
            .times(1)
            .returning(|| Err(crate::domain::errors::config_error::ConfigError::SessionError(
                "You need to log in first! Run `guepard login` to get started. 🐆".to_string()
            )));

        let result = list_all_commits_with_deps("dep-1", &config, &auth).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            BookmarkError::SessionError(msg) => assert!(msg.contains("You need to log in first!")),
            _ => panic!("Expected SessionError"),
        }
    }

    #[tokio::test]
    async fn test_list_bookmark_create_checkout_network_or_api_errors() {
        let config = Config { api_url: "https://api.guepard.run".to_string(), app_url: "https://app.guepard.run".to_string() };
        let mut auth = MockAuthProvider::new();
        auth
            .expect_get_auth_token()
            .times(3)
            .returning(|| Ok("test-jwt-token".to_string()));

        let r1 = list_bookmark_with_deps("dep-1", "branch-1", &config, &auth).await;
        assert!(r1.is_err());

        let create_req = CreateCommitRequest { snapshot_comment: "test".to_string() };
        let r2 = create_commit_with_deps("dep-1", "branch-1", create_req, &config, &auth).await;
        assert!(r2.is_err());

        let branch_req = BranchRequest { 
            branch_name: Some("new-branch".to_string()),
            discard_changes: Some("true".to_string()),
            checkout: true,
            ephemeral: false,
        };
        let r3 = checkout_bookmark_with_deps("dep-1", "branch-1", "snap-1", branch_req, &config, &auth).await;
        assert!(r3.is_err());
    }
}