use crate::config::config::{self, Config};
use crate::domain::errors::logout_error::LogoutError;
use crate::structure::LogoutArgs;
use anyhow::Result;
use colored::Colorize;

use crate::application::output::{OutputFormat, print_json};

pub async fn logout(_args: &LogoutArgs, _config: &Config, output_format: OutputFormat) -> Result<()> {
    // Check if user is already logged out
    if !config::is_logged_in() {
        if output_format == OutputFormat::Json {
            print_json(&serde_json::json!({ "status": "already_logged_out", "message": "You are already logged out!" }));
        } else {
            println!("{}", "You are already logged out! 🐆".yellow());
        }
        return Ok(());
    }

    config::delete_session().map_err(|e| LogoutError::ConfigError(e.to_string()))?;
    config::delete_jwt_token().map_err(|e| LogoutError::ConfigError(e.to_string()))?;
    
    if output_format == OutputFormat::Json {
        print_json(&serde_json::json!({ "status": "success", "message": "Logged out successfully" }));
    } else {
        println!("{}", "Logged out successfully! 🐆".green());
    }
    Ok(())
}