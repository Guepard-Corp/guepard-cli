use crate::config::config::Config;
use crate::domain::errors::link_error::LinkError;
use crate::application::services::link_service;
use colored::Colorize;

pub async fn execute(config: &Config) -> Result<(), LinkError> {
    let url = link_service::start_login(config).await?;
    println!("{} {}", "Open this URL in your browser:".green(), url);
    Ok(())
}