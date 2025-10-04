use anyhow::Result;
use crate::config::config::Config;
use crate::structure::LogArgs;
use crate::application::auth;
use colored::Colorize;
use reqwest::Client;

pub async fn log(args: &LogArgs, config: &Config) -> Result<()> {
    let jwt_token = auth::get_auth_token()?;
    let client = Client::new();
    
    let response = client
        .get(format!("{}/deploy/{}/logs", config.api_url, args.deployment_id))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await?;

    if response.status().is_success() {
        let logs_text = response.text().await?;
        println!("{} Deployment Logs for: {}", "📋".blue(), args.deployment_id);
        println!("{}", "=".repeat(50).dimmed());
        println!("{}", logs_text);
    } else {
        let error_text = response.text().await.unwrap_or_default();
        println!("{} Failed to fetch logs: {}", "❌".red(), error_text);
    }
    
    Ok(())
}
