use chrono::{DateTime, Utc};
use dotenvy::dotenv;
#[cfg(feature = "keyring")]
use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{self, File};
use log::{debug, error};
use crate::domain::errors::config_error::ConfigError;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[derive(Debug, Clone)]
pub struct Config {
    pub api_url: String,
    // pub api_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct SessionData {
    session_id: String,
    created_at: String,
}

pub fn load_config() -> Result<Config, ConfigError> {
    dotenv().ok();

    let api_url = env::var("PUBLIC_API")
        .map_err(|_| ConfigError::MissingEnv("Missing PUBLIC_API in .env file".to_string()))?;
    // let api_token = env::var("API_TOKEN")
    //     .map_err(|_| ConfigError::MissingEnv("Missing API_TOKEN in .env file".to_string()))?;

    Ok(Config { api_url,})
}

pub fn save_session_id(session_id: &str) -> Result<(), ConfigError> {
    let path = dirs::home_dir()
        .ok_or_else(|| ConfigError::IoError("Home directory not found".to_string()))?
        .join(".guepard/session.json");

    fs::create_dir_all(path.parent().unwrap())
        .map_err(|e| ConfigError::IoError(format!("Failed to create .guepard directory: {}", e)))?;

    let data = SessionData {
        session_id: session_id.to_string(),
        created_at: Utc::now().to_rfc3339(),
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

pub fn save_jwt_token(jwt_token: &str) -> Result<(), ConfigError> {
    let path = dirs::home_dir()
        .ok_or_else(|| ConfigError::IoError("Home directory not found".to_string()))?
        .join(".guepard/session.json");

    if !path.exists() {
        return Err(ConfigError::SessionError(
            "No login session found. Run `guepard link` first.".to_string(),
        ));
    }

    #[cfg(feature = "keyring")]
    {
        let entry = Entry::new("guepard-cli", "session")
            .map_err(|e| ConfigError::KeyringError(format!("Failed to access keyring entry: {}", e)))?;
        entry
            .set_password(jwt_token)
            .map_err(|e| ConfigError::KeyringError(format!("Failed to store JWT token securely: {}", e)))?;
    }
    
    #[cfg(not(feature = "keyring"))]
    {
        // Fallback: store JWT in session file (less secure but works for cross-compilation)
        let mut data: SessionData = {
            let file = File::open(&path)
                .map_err(|e| ConfigError::IoError(format!("Failed to open session file: {}", e)))?;
            serde_json::from_reader(file)
                .map_err(|e| ConfigError::IoError(format!("Invalid session data: {}", e)))?
        };
        
        // Store JWT in a separate file for cross-compilation builds
        let jwt_path = path.with_extension("jwt");
        fs::write(&jwt_path, jwt_token)
            .map_err(|e| ConfigError::IoError(format!("Failed to write JWT file: {}", e)))?;
        
        #[cfg(unix)]
        fs::set_permissions(&jwt_path, fs::Permissions::from_mode(0o600))
            .map_err(|e| ConfigError::IoError(format!("Failed to set JWT file permissions: {}", e)))?;
    }

    Ok(())
}

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
        delete_jwt_token().ok();
        return Err(ConfigError::SessionError(
            "Session ID expired. Run `guepard link` to start a new login.".to_string(),
        ));
    }

    Ok(data.session_id)
}

pub fn load_jwt_token() -> Result<String, ConfigError> {
    #[cfg(feature = "keyring")]
    {
        let entry = Entry::new("guepard-cli", "session")
            .map_err(|e| ConfigError::KeyringError(format!("Failed to access keyring entry: {}", e)))?;
        entry
            .get_password()
            .map_err(|e| {
                ConfigError::SessionError(format!(
                    "No JWT token found. Run `guepard login` first. Error: {}",
                    e
                ))
            })
    }
    
    #[cfg(not(feature = "keyring"))]
    {
        let path = dirs::home_dir()
            .ok_or_else(|| ConfigError::IoError("Home directory not found".to_string()))?
            .join(".guepard/session.jwt");
            
        if !path.exists() {
            return Err(ConfigError::SessionError(
                "No JWT token found. Run `guepard login` first.".to_string(),
            ));
        }
        
        fs::read_to_string(&path)
            .map_err(|e| ConfigError::IoError(format!("Failed to read JWT file: {}", e)))
    }
}

pub fn delete_jwt_token() -> Result<(), ConfigError> {
    #[cfg(feature = "keyring")]
    {
        debug!("Creating keyring entry for guepard:session");
        let entry = Entry::new("guepard-cli", "session")
            .map_err(|e| ConfigError::KeyringError(format!("Failed to access keyring entry: {}", e)))?;
        debug!("Attempting to delete JWT from keyring");
        match entry.delete_credential() {
            Ok(_) => {
                debug!("Successfully deleted JWT from keyring");
                Ok(())
            }
            Err(e) => {
                error!("Failed to delete JWT from keyring: {}", e);
                Err(ConfigError::KeyringError(format!("Failed to delete JWT token: {}", e)))
            }
        }
    }
    
    #[cfg(not(feature = "keyring"))]
    {
        debug!("Deleting JWT file for cross-compilation build");
        let path = dirs::home_dir()
            .ok_or_else(|| ConfigError::IoError("Home directory not found".to_string()))?
            .join(".guepard/session.jwt");
            
        if path.exists() {
            fs::remove_file(&path)
                .map_err(|e| ConfigError::IoError(format!("Failed to remove JWT file: {}", e)))?;
            debug!("Successfully deleted JWT file");
        } else {
            debug!("No JWT file found");
        }
        Ok(())
    }
}

pub fn delete_session() -> Result<(), ConfigError> {
    debug!("Starting session deletion");

    let path = dirs::home_dir()
        .ok_or_else(|| ConfigError::IoError("Home directory not found".to_string()))?
        .join(".guepard/session.json");

    if path.exists() {
        fs::remove_file(&path)
            .map_err(|e| ConfigError::IoError(format!("Failed to remove session file: {}", e)))?;
        debug!("Removed session file: {}", path.display());
    } else {
        debug!("No session file found at {}", path.display());
    }

    debug!("Session deletion completed");
    Ok(())
}