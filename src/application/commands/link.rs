use crate::config::config::Config;
use crate::domain::errors::link_error::LinkError;
use crate::application::services::link;
use colored::Colorize;

pub async fn execute(config: &Config) -> Result<(), LinkError> {
    let url = link::start_login(config).await?;
    println!("{} {}", "Open this URL in your browser:".green(), url);
    println!("{}", "Next, use the login command to enter the verification code provided after accessing the URL 🐆".yellow());
    Ok(())
}