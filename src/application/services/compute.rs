use crate::application::dto::compute::{ListComputeResponse, LogsResponse, StatusErrorResponse, ComputeStatusResponse};
use crate::config::config::{self, Config};
use crate::domain::errors::compute_error::ComputeError;

use anyhow::Result;
use reqwest::{Client, StatusCode};

pub async fn list_compute(deployment_id: &str, config: &Config) -> Result<ListComputeResponse, ComputeError> {
    let jwt_token = config::load_jwt_token()
        .map_err(|e| ComputeError::SessionError(e.to_string()))?;
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy/{}/compute", config.api_url, deployment_id))
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

pub async fn start_compute(deployment_id: &str, config: &Config) -> Result<(), ComputeError> {
    let jwt_token = config::load_jwt_token()
        .map_err(|e| ComputeError::SessionError(e.to_string()))?;
    let client = Client::new();
    let response = client
        .post(format!("{}/deploy/{}/compute/start", config.api_url, deployment_id))
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

pub async fn stop_compute(deployment_id: &str, config: &Config) -> Result<(), ComputeError> {
    let jwt_token = config::load_jwt_token()
        .map_err(|e| ComputeError::SessionError(e.to_string()))?;
    let client = Client::new();
    let response = client
        .post(format!("{}/deploy/{}/compute/stop", config.api_url, deployment_id))
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

pub async fn get_logs(deployment_id: &str, config: &Config) -> Result<LogsResponse, ComputeError> {
    let jwt_token = config::load_jwt_token()
        .map_err(|e| ComputeError::SessionError(e.to_string()))?;
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy/{}/compute/logs", config.api_url, deployment_id))
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

pub async fn get_status(deployment_id: &str, config: &Config) -> Result<ComputeStatusResponse, ComputeError> {
    let jwt_token = config::load_jwt_token()
        .map_err(|e| ComputeError::SessionError(e.to_string()))?;
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy/{}/compute/status", config.api_url, deployment_id))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
        .map_err(ComputeError::RequestFailed)?;

    match response.status() {
        StatusCode::OK => response
            .json::<ComputeStatusResponse>()
            .await
            .map_err(|e| ComputeError::ParseError(e.to_string())),
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