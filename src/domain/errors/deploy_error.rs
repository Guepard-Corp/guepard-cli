use anyhow::Result;
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeployError {
    #[error("API error: {0}")]
    ApiError(String),

    #[error("Request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Failed to parse response: {0}")]
    ParseError(String), 

    #[error("400 Bad Request: {0}")]
    BadRequest(String),

    #[error("403 Forbidden: Invalid API token or permissions")]
    Forbidden,

    #[error("404 Not Found: The requested resource was not found")]
    NotFound,

    #[error("500 Internal Server Error: Something went wrong on the server")]
    InternalServerError,

    #[error("503 Service Unavailable: The API is currently down")]
    ServiceUnavailable,

    #[error("Unexpected response: {0}")]
    Unexpected(String),
}

impl DeployError {
    pub async fn from_response(response: reqwest::Response) -> Self {
        let status = response.status();
        let text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());

        match status {
            StatusCode::BAD_REQUEST => DeployError::BadRequest(text),
            StatusCode::FORBIDDEN => DeployError::Forbidden,
            StatusCode::NOT_FOUND => DeployError::NotFound,
            StatusCode::INTERNAL_SERVER_ERROR => DeployError::InternalServerError,
            StatusCode::SERVICE_UNAVAILABLE => DeployError::ServiceUnavailable,
            _ => DeployError::Unexpected(format!("{}: {}", status, text)),
        }
    }
}

/// handles API responses
pub async fn handle_api_response(response: reqwest::Response) -> Result<()> {
    match response.status() {
        reqwest::StatusCode::OK => {
            Ok(())
        }
        reqwest::StatusCode::BAD_REQUEST => {
            let err_msg = response
                .text()
                .await
                .unwrap_or("Invalid request data".to_string());
            Err(anyhow::anyhow!("❌ 400 Bad Request: {}", err_msg))
        }
        reqwest::StatusCode::FORBIDDEN => Err(anyhow::anyhow!(
            "❌ 403 Forbidden: Invalid API token or permissions"
        )),
        reqwest::StatusCode::SERVICE_UNAVAILABLE => Err(anyhow::anyhow!(
            "❌ 503 Service Unavailable: The API is currently down"
        )),
        _ => {
            let err_msg = response.text().await.unwrap_or("Unknown error".to_string());
            Err(anyhow::anyhow!("❌ Unexpected Error: {}", err_msg))
        }
    }
}
