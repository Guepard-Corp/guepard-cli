use crate::config::config::{load_config_data, save_config_data, ConfigData, is_logged_in, load_jwt_token, delete_session, delete_jwt_token};
use crate::domain::errors::config_error::ConfigError;
use crate::structure::ConfigArgs;
use colored::Colorize;
use base64;

use crate::application::output::{OutputFormat, print_json};

pub async fn config(args: &ConfigArgs, output_format: OutputFormat) -> Result<(), ConfigError> {
    if args.show || args.get {
        show_config(output_format).await
    } else if let Some(api_url) = &args.api_url {
        set_api_url(api_url, output_format).await
    } else {
        // Default behavior: show current config
        show_config(output_format).await
    }
}

async fn show_config(output_format: OutputFormat) -> Result<(), ConfigError> {
    let config_data = load_config_data()?;
    let logged_in = is_logged_in();
    let user = if logged_in {
        get_user_info_from_token().unwrap_or_else(|_| "Unknown".to_string())
    } else {
        "Not logged in".to_string()
    };

    if output_format == OutputFormat::Json {
        print_json(&serde_json::json!({
            "api_url": config_data.api_url,
            "logged_in": logged_in,
            "user": user
        }));
        return Ok(());
    }
    
    println!("⚙️  Current Configuration:");
    println!("   API URL: {}", config_data.api_url);
    
    // Show login status
    if logged_in {
        println!("   User: {} {}", "✓".green(), user);
    } else {
        println!("   User: {} Not logged in", "✗".red());
    }
    
    Ok(())
}

async fn set_api_url(api_url: &str, output_format: OutputFormat) -> Result<(), ConfigError> {
    // ... validation ...
    if !api_url.starts_with("http://") && !api_url.starts_with("https://") {
        return Err(ConfigError::IoError(
            "Invalid URL format. Must start with http:// or https://".to_string()
        ));
    }
    
    // Check if user is logged in and force logout if changing API URL
    if is_logged_in() {
        if output_format == OutputFormat::Table {
            println!("⚠️  Changing API URL requires re-authentication.");
            println!("   Logging out current session...");
        }
        
        delete_session().map_err(|e| ConfigError::IoError(format!("Failed to delete session: {}", e)))?;
        delete_jwt_token().map_err(|e| ConfigError::IoError(format!("Failed to delete JWT token: {}", e)))?;
        
        if output_format == OutputFormat::Table {
            println!("{}", "✓ Logged out successfully!".green());
            println!("   Please run `guepard login` to authenticate with the new API endpoint.");
        }
    }
    
    let config_data = ConfigData {
        api_url: api_url.to_string(),
    };
    
    save_config_data(&config_data)?;
    
    if output_format == OutputFormat::Json {
        print_json(&serde_json::json!({
            "status": "success",
            "api_url": api_url,
            "message": "Configuration updated successfully"
        }));
    } else {
        println!("✅ Configuration updated successfully!");
        println!("   API URL: {}", api_url);
        println!("   Note: Environment variable PUBLIC_API takes precedence over this setting.");
    }
    
    Ok(())
}

fn get_user_info_from_token() -> Result<String, ConfigError> {
    let token = load_jwt_token()?;
    
    // JWT tokens have 3 parts separated by dots: header.payload.signature
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(ConfigError::IoError("Invalid JWT token format".to_string()));
    }
    
    // Decode the payload (second part)
    let payload = parts[1];
    
    // Add padding if needed for base64 decoding
    let mut padded_payload = payload.to_string();
    while padded_payload.len() % 4 != 0 {
        padded_payload.push('=');
    }
    
    // Decode base64
    let decoded = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &padded_payload)
        .map_err(|e| ConfigError::IoError(format!("Failed to decode JWT payload: {}", e)))?;
    
    let payload_str = String::from_utf8(decoded)
        .map_err(|e| ConfigError::IoError(format!("Invalid UTF-8 in JWT payload: {}", e)))?;
    
    // Parse JSON to extract user info
    let payload_json: serde_json::Value = serde_json::from_str(&payload_str)
        .map_err(|e| ConfigError::IoError(format!("Invalid JSON in JWT payload: {}", e)))?;
    
    // Try to extract email or username from common JWT fields
    if let Some(email) = payload_json.get("email").and_then(|v| v.as_str()) {
        return Ok(email.to_string());
    }
    if let Some(sub) = payload_json.get("sub").and_then(|v| v.as_str()) {
        return Ok(sub.to_string());
    }
    if let Some(username) = payload_json.get("username").and_then(|v| v.as_str()) {
        return Ok(username.to_string());
    }
    if let Some(name) = payload_json.get("name").and_then(|v| v.as_str()) {
        return Ok(name.to_string());
    }
    
    // If no recognizable field, return a generic message
    Ok("User".to_string())
}
