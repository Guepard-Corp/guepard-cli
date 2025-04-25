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

    #[error("Session error: {0}")]
    SessionError(String),
}