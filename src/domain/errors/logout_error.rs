use thiserror::Error;

#[derive(Error, Debug)]
pub enum LogoutError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
}