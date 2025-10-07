use crate::application::auth;
use crate::application::dto::commit::{GetCommitResponse, CreateCommitRequest, CreateCommitResponse, CheckoutCommitResponse};
use crate::application::dto::branch::BranchRequest;
use crate::config::config::Config;
use crate::domain::errors::bookmark_error::BookmarkError;
use anyhow::Result;
use reqwest::Client;

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

pub async fn create_commit_with_deps<A: AuthProvider>(
    deployment_id: &str,
    branch_id: &str,
    request: CreateCommitRequest,
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
        let config = Config { api_url: "https://api.guepard.run".to_string() };
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
        let config = Config { api_url: "https://api.guepard.run".to_string() };
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