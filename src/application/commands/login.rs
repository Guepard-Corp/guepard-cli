use crate::config::config::Config;
use crate::application::services::login_service;
use anyhow::Result;
use colored::Colorize;

pub async fn execute(config: &Config, verification_code: &str) -> Result<()> {
    login_service::complete_login(config, verification_code).await?;
    println!(
        "{} {}",
        "Login successful.".green(),
        "Happy coding! 🐆".red()
    );
    Ok(())
}