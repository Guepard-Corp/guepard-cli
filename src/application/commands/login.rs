use crate::config::config::Config;
use crate::application::services::login;
use anyhow::Result;
use colored::Colorize;

pub async fn execute(config: &Config, verification_code: &str) -> Result<()> {
    login::complete_login(config, verification_code).await?;
    println!(
        "{} {}",
        "Login successful.".green(),
        "Happy coding! 🐆".yellow().bold()
    );
    println!(
        "You can now use the Guepard CLI to interact with your Guepard account.🐆"
    );
    println!(
        "{}",
        "To get started, run: `gprd help`".yellow()
    );
    println!(
        "{}",
        "To log out, use the `logout` command.".red()
    );
    Ok(())
}