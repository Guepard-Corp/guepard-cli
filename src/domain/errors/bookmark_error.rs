use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BookmarkError {
    #[error("API configuration error: {0}")]
    ApiError(String),

    #[error("Request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Failed to parse response: {0}")]
    ParseError(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Server error: {0}")]
    InternalServerError(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Unexpected error: {0}")]
    Unexpected(String),

    #[error("{0}")]
    SessionError(String),
}

impl BookmarkError {
    fn extract_message(text: &str) -> String {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(text) {
            if let Some(msg) = v.get("message").and_then(|m| m.as_str()) {
                return msg.to_string();
            }
        }
        text.to_string()
    }

    pub async fn from_response(response: reqwest::Response) -> Self {
        let status = response.status();
        let text = response
            .text()
            .await
            .unwrap_or_else(|_| "No additional details provided by server".to_string());
        let message = Self::extract_message(&text);

        match status {
            StatusCode::BAD_REQUEST => BookmarkError::BadRequest(message),
            StatusCode::FORBIDDEN => BookmarkError::Forbidden(message),
            StatusCode::NOT_FOUND => BookmarkError::NotFound(message),
            StatusCode::INTERNAL_SERVER_ERROR => BookmarkError::InternalServerError(message),
            StatusCode::SERVICE_UNAVAILABLE => BookmarkError::ServiceUnavailable(message),
            _ => BookmarkError::Unexpected(format!("Status {}: {}", status, message)),
        }
    }
}