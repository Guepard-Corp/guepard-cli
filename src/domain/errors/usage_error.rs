use thiserror::Error;

#[derive(Error, Debug)]
pub enum UsageError {
    #[error("Request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Failed to parse response: {0}")]
    ParseError(String),

    #[error("403 Forbidden: {0}")]
    Forbidden(String),

    #[error("500 Internal Server Error: {0}")]
    InternalServerError(String),

    #[error("Unexpected error: {0}")]
    Unexpected(String),

    #[error("{0}")]
    SessionError(String),
}