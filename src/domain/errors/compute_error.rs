use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ComputeError {
    #[error("Request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Failed to parse response: {0}")]
    ParseError(String),

    #[error("500 Internal Server Error: {0}")]
    InternalServerError(String),

    #[error("Compute not healthy: {0}")]
    NotHealthy(String),

    #[error("Unexpected error: {0}")]
    Unexpected(String),

    #[error("Insufficient node resources: {0}")]
    InsufficientNodeResources(String),

    #[error("{0}")]
    SessionError(String),
}

impl ComputeError {
    pub fn from_status_and_body(status: StatusCode, text: &str) -> Self {
        let (message, code) =
            crate::domain::errors::deploy_error::parse_api_error_body(text);
        match status {
            StatusCode::UNAUTHORIZED => ComputeError::SessionError(message),
            StatusCode::CONFLICT if code.as_deref() == Some("INSUFFICIENT_NODE_RESOURCES") => {
                ComputeError::InsufficientNodeResources(message)
            }
            StatusCode::INTERNAL_SERVER_ERROR => ComputeError::InternalServerError(message),
            _ => ComputeError::Unexpected(format!("{}: {}", status, message)),
        }
    }
}
