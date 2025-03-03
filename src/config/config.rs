use anyhow::{Context, Result};
use dotenvy::dotenv;
use std::env;

// Configuration struct to hold API URL and token
#[derive(Debug, Clone)]
pub struct Config {
    pub api_url: String,
    pub api_token: String,
}

// Function to load and return the Config
pub fn load_config() -> Result<Config> {
    // Load .env file (optional, fails silently if not present)
    dotenv().ok();

    let api_url = env::var("PUBLIC_API")
        .context("Missing PUBLIC_API_DEPLOY in .env file")?;
    let api_token = env::var("API_TOKEN")
        .context("Missing API_TOKEN in .env file")?;

    Ok(Config {
        api_url,
        api_token,
    })
}