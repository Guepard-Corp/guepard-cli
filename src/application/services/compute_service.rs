use crate::application::dto::compute_dto::{ListComputeResponse, LogsResponse, StatusErrorResponse};
use crate::config::config::{self, Config};
use crate::domain::errors::compute_error::ComputeError;

use anyhow::Result;
use reqwest::{Client, StatusCode};

pub async fn list_compute(deployment_id: &str, compute_id: &str, config: &Config) -> Result<ListComputeResponse, ComputeError> {
    let jwt_token = config::load_jwt_token()
        .map_err(|e| ComputeError::SessionError(e.to_string()))?;
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy/{}/{}", config.api_url, deployment_id, compute_id))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
        .map_err(ComputeError::RequestFailed)?;

    match response.status() {
        StatusCode::OK => response
            .json::<ListComputeResponse>()
            .await
            .map_err(|e| ComputeError::ParseError(e.to_string())),
        status => {
            let text = response.text().await.unwrap_or("No details".to_string());
            Err(ComputeError::Unexpected(format!("Status {}: {}", status, text)))
        }
    }
}

pub async fn start_compute(deployment_id: &str, compute_id: &str, config: &Config) -> Result<(), ComputeError> {
    let jwt_token = config::load_jwt_token()
        .map_err(|e| ComputeError::SessionError(e.to_string()))?;
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy/{}/{}/start", config.api_url, deployment_id, compute_id))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
        .map_err(ComputeError::RequestFailed)?;

    match response.status() {
        StatusCode::OK => Ok(()),
        status => {
            let text = response.text().await.unwrap_or("No details".to_string());
            Err(ComputeError::Unexpected(format!("Status {}: {}", status, text)))
        }
    }
}

pub async fn stop_compute(deployment_id: &str, compute_id: &str, config: &Config) -> Result<(), ComputeError> {
    let jwt_token = config::load_jwt_token()
        .map_err(|e| ComputeError::SessionError(e.to_string()))?;
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy/{}/{}/stop", config.api_url, deployment_id, compute_id))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
        .map_err(ComputeError::RequestFailed)?;

    match response.status() {
        StatusCode::OK => Ok(()),
        status => {
            let text = response.text().await.unwrap_or("No details".to_string());
            Err(ComputeError::Unexpected(format!("Status {}: {}", status, text)))
        }
    }
}

pub async fn get_logs(deployment_id: &str, compute_id: &str, config: &Config) -> Result<LogsResponse, ComputeError> {
    let jwt_token = config::load_jwt_token()
        .map_err(|e| ComputeError::SessionError(e.to_string()))?;
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy/{}/{}/logs", config.api_url, deployment_id, compute_id))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
        .map_err(ComputeError::RequestFailed)?;

    match response.status() {
        StatusCode::OK => response
            .json::<LogsResponse>()
            .await
            .map_err(|e| ComputeError::ParseError(e.to_string())),
        StatusCode::INTERNAL_SERVER_ERROR => {
            let text = response.text().await.unwrap_or("No details".to_string());
            Err(ComputeError::InternalServerError(text))
        }
        status => {
            let text = response.text().await.unwrap_or("No details".to_string());
            Err(ComputeError::Unexpected(format!("Status {}: {}", status, text)))
        }
    }
}

pub async fn get_status(deployment_id: &str, compute_id: &str, config: &Config) -> Result<(), ComputeError> {
    let jwt_token = config::load_jwt_token()
        .map_err(|e| ComputeError::SessionError(e.to_string()))?;
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy/{}/{}/status", config.api_url, deployment_id, compute_id))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
        .map_err(ComputeError::RequestFailed)?;

    match response.status() {
        StatusCode::OK => Ok(()),
        StatusCode::GONE => {
            let error = response
                .json::<StatusErrorResponse>()
                .await
                .map_err(|e| ComputeError::ParseError(e.to_string()))?;
            Err(ComputeError::NotHealthy(error.message))
        }
        status => {
            let text = response.text().await.unwrap_or("No details".to_string());
            Err(ComputeError::Unexpected(format!("Status {}: {}", status, text)))
        }
    }
}