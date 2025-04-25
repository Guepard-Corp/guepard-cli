use chrono::{DateTime, Utc};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{self, File, OpenOptions};

use crate::domain::errors::config_error::ConfigError;

// Configuration struct to hold API URL and token
#[derive(Debug, Clone)]
pub struct Config {
    pub api_url: String,
    pub api_token: String,
}

// Session data stored in ~/.guepard/session.json
#[derive(Serialize, Deserialize)]
pub struct SessionData {
    session_id: String,
    created_at: String,
    #[serde(default)]
    jwt_token: Option<String>,
}

// Function to load and return the Config
pub fn load_config() -> Result<Config, ConfigError> {
    dotenv().ok();

    let api_url = env::var("PUBLIC_API")
        .map_err(|_| ConfigError::MissingEnv("Missing PUBLIC_API in .env file".to_string()))?;
    let api_token = env::var("API_TOKEN")
        .map_err(|_| ConfigError::MissingEnv("Missing API_TOKEN in .env file".to_string()))?;

    Ok(Config {
        api_url,
        api_token,
    })
}

// Save session ID to ~/.guepard/session.json
pub fn save_session_id(session_id: &str) -> Result<(), ConfigError> {
    let path = dirs::home_dir()
        .ok_or_else(|| ConfigError::IoError("Home directory not found".to_string()))?
        .join(".guepard/session.json");

    fs::create_dir_all(path.parent().unwrap())
        .map_err(|e| ConfigError::IoError(format!("Failed to create .guepard directory: {}", e)))?;

    let data = SessionData {
        session_id: session_id.to_string(),
        created_at: Utc::now().to_rfc3339(),
        jwt_token: None,
    };

    let file = File::create(&path)
        .map_err(|e| ConfigError::IoError(format!("Failed to create session file: {}", e)))?;
    serde_json::to_writer(&file, &data)
        .map_err(|e| ConfigError::IoError(format!("Failed to write session file: {}", e)))?;

    #[cfg(unix)]
    fs::set_permissions(&path, fs::Permissions::from_mode(0o600))
        .map_err(|e| ConfigError::IoError(format!("Failed to set permissions: {}", e)))?;

    Ok(())
}

// Save JWT token to ~/.guepard/session.json
pub fn save_jwt_token(jwt_token: &str) -> Result<(), ConfigError> {
    let path = dirs::home_dir()
        .ok_or_else(|| ConfigError::IoError("Home directory not found".to_string()))?
        .join(".guepard/session.json");

    if !path.exists() {
        return Err(ConfigError::SessionError(
            "No login session found. Run `guepard link` first.".to_string(),
        ));
    }

    let file = File::open(&path)
        .map_err(|e| ConfigError::IoError(format!("Failed to open session file: {}", e)))?;
    let mut data: SessionData = serde_json::from_reader(file)
        .map_err(|e| ConfigError::IoError(format!("Invalid session data: {}", e)))?;

    data.jwt_token = Some(jwt_token.to_string());

    let  file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
        .map_err(|e| ConfigError::IoError(format!("Failed to create session file: {}", e)))?;
    serde_json::to_writer(&file, &data)
        .map_err(|e| ConfigError::IoError(format!("Failed to write session file: {}", e)))?;

    Ok(())
}

// Load session ID from ~/.guepard/session.json
pub fn load_session_id() -> Result<String, ConfigError> {
    let path = dirs::home_dir()
        .ok_or_else(|| ConfigError::IoError("Home directory not found".to_string()))?
        .join(".guepard/session.json");

    if !path.exists() {
        return Err(ConfigError::SessionError(
            "No login session found. Run `guepard link` first.".to_string(),
        ));
    }

    let file = File::open(&path)
        .map_err(|e| ConfigError::IoError(format!("Failed to open session file: {}", e)))?;
    let data: SessionData = serde_json::from_reader(file)
        .map_err(|e| ConfigError::IoError(format!("Invalid session data: {}", e)))?;

    let created = DateTime::parse_from_rfc3339(&data.created_at)
        .map_err(|e| ConfigError::IoError(format!("Invalid timestamp: {}", e)))?;
    if Utc::now().signed_duration_since(created).num_minutes() > 10 {
        fs::remove_file(&path)
            .map_err(|e| ConfigError::IoError(format!("Failed to remove expired session: {}", e)))?;
        return Err(ConfigError::SessionError(
            "Session ID expired. Run `guepard link` to start a new login.".to_string(),
        ));
    }

    Ok(data.session_id)
}

// Load JWT token from ~/.guepard/session.json
pub fn load_jwt_token() -> Result<String, ConfigError> {
    let path = dirs::home_dir()
        .ok_or_else(|| ConfigError::IoError("Home directory not found".to_string()))?
        .join(".guepard/session.json");
    let file = File::open(&path)
        .map_err(|e| ConfigError::IoError(format!("Failed to open session file: {}", e)))?;
    let data: SessionData = serde_json::from_reader(file)
        .map_err(|e| ConfigError::IoError(format!("Invalid session data: {}", e)))?;
    data.jwt_token
        .ok_or_else(|| ConfigError::SessionError("No JWT token found. Run `guepard login` first.".to_string()))
}