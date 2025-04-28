use crate::config::config::{self, Config};
use crate::domain::errors::logout_error::LogoutError;
use anyhow::Result;
use colored::Colorize;

pub async fn logout(_config: &Config) -> Result<()> {
    config::delete_session().map_err(|e| LogoutError::ConfigError(e.to_string()))?;
    config::delete_jwt_token().map_err(|e| LogoutError::ConfigError(e.to_string()))?;
    println!("{}", "Logged out successfully! 🐆".green());
    Ok(())
}