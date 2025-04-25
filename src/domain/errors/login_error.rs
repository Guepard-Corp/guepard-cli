use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("API error: {0}")]
    ApiError(String),

    #[error("Session error: {0}")]
    SessionError(String),
}