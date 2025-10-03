use crate::application::auth;
use crate::application::dto::usage::UsageResponse;
use crate::config::config::Config;
use crate::domain::errors::usage_error::UsageError;
use anyhow::Result;
use reqwest::{Client, StatusCode};

pub async fn get_usage(config: &Config) -> Result<UsageResponse, UsageError> {
    let jwt_token = auth::get_auth_token()
        .map_err(|e| UsageError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    let response = client
        .get(format!("{}/usage", config.api_url))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
        .map_err(UsageError::RequestFailed)?;

    match response.status() {
        StatusCode::OK => response
            .json::<UsageResponse>()
            .await
            .map_err(|e| UsageError::ParseError(e.to_string())),
        StatusCode::FORBIDDEN => {
            let text = response.text().await.unwrap_or("No details".to_string());
            Err(UsageError::Forbidden(text))
        }
        StatusCode::INTERNAL_SERVER_ERROR => {
            let text = response.text().await.unwrap_or("No details".to_string());
            Err(UsageError::InternalServerError(text))
        }
        status => {
            let text = response.text().await.unwrap_or("No details".to_string());
            Err(UsageError::Unexpected(format!("Status {}: {}", status, text)))
        }
    }
}