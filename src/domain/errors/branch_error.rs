use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BranchError {
    #[error("API configuration error: {0}")]
    ApiError(String),

    #[error("Request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Failed to parse response: {0}")]
    ParseError(String),

    #[error("400 Bad Request: {0}")]
    BadRequest(String),

    #[error("403 Forbidden: {0}")]
    Forbidden(String),

    #[error("404 Not Found: {0}")]
    NotFound(String),

    #[error("500 Internal Server Error: {0}")]
    InternalServerError(String),

    #[error("503 Service Unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Unexpected error: {0}")]
    Unexpected(String),

    #[error("{0}")]
    SessionError(String),
}

impl BranchError {
    pub async fn from_response(response: reqwest::Response) -> Self {
        let status = response.status();
        let text = response
            .text()
            .await
            .unwrap_or_else(|_| "No additional details provided by server".to_string());

        match status {
            StatusCode::BAD_REQUEST => BranchError::BadRequest(text),
            StatusCode::FORBIDDEN => BranchError::Forbidden(text),
            StatusCode::NOT_FOUND => BranchError::NotFound(text),
            StatusCode::INTERNAL_SERVER_ERROR => BranchError::InternalServerError(text),
            StatusCode::SERVICE_UNAVAILABLE => BranchError::ServiceUnavailable(text),
            _ => BranchError::Unexpected(format!("Status {}: {}", status, text)),
        }
    }
}