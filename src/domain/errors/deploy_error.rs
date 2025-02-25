use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeployError {
    #[error("API error: {0}")]
    ApiError(String),
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
}