use thiserror::Error;

#[derive(Error, Debug)]
pub enum TenetError {
    #[error("Request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Failed to parse response: {0}")]
    ParseError(String),

    #[error("Unexpected error: {0}")]
    Unexpected(String),

    #[error("{0}")]
    SessionError(String),

    #[error("{0}")]
    IoError(String),
}
