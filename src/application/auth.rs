use crate::config::config;
use crate::domain::errors::config_error::ConfigError;

/// Centralized function to load JWT token with consistent error handling
/// This ensures all authenticated commands show the same user-friendly error message
pub fn get_auth_token() -> Result<String, ConfigError> {
    config::load_jwt_token()
}
