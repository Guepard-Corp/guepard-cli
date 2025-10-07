use crate::application::auth;
use crate::config::config::Config;
use crate::domain::errors::deploy_error::DeployError;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PerformanceProfile {
    pub id: String,
    pub label_name: String,
    pub description_text: String,
    pub database_provider: String,
    pub database_version: String,
    pub min_cpu: i32,
    pub min_memory: i32,
    pub is_default: bool,
    pub is_active: bool,
}

// Trait for dependency injection to make testing easier
#[cfg_attr(test, mockall::automock)]
pub trait AuthProvider {
    fn get_auth_token(&self) -> Result<String, crate::domain::errors::config_error::ConfigError>;
}

// Default implementation that uses the real auth module
pub struct DefaultAuthProvider;

impl AuthProvider for DefaultAuthProvider {
    fn get_auth_token(&self) -> Result<String, crate::domain::errors::config_error::ConfigError> {
        auth::get_auth_token()
    }
}

pub async fn list_performance_profiles_with_deps<A: AuthProvider>(
    config: &Config,
    auth_provider: &A,
) -> Result<Vec<PerformanceProfile>, DeployError> {
    let token = auth_provider
        .get_auth_token()
        .map_err(|e| DeployError::SessionError(format!("{}", e)))?;

    let client = Client::new();
    let response = client
        .get(&format!("{}/performance", config.api_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| DeployError::ApiError(format!("Network error: {}", e)))?;

    if !response.status().is_success() {
        return Err(DeployError::ApiError(format!(
            "API error: {} {}",
            response.status(),
            response.text().await.unwrap_or_default()
        )));
    }

    let profiles: Vec<PerformanceProfile> = response
        .json()
        .await
        .map_err(|e| DeployError::ApiError(format!("Invalid response: {}", e)))?;

    Ok(profiles)
}

// Public function that maintains the original API
pub async fn list_performance_profiles(config: &Config) -> Result<Vec<PerformanceProfile>, DeployError> {
    let auth_provider = DefaultAuthProvider;
    list_performance_profiles_with_deps(config, &auth_provider).await
}

pub async fn get_performance_profile_by_label(
    label_name: &str,
    database_provider: &str,
    database_version: &str,
    config: &Config,
) -> Result<String, DeployError> {
    let profiles = list_performance_profiles(config).await?;
    
    if let Some(id) = select_profile_id(&profiles, label_name, database_provider, database_version) {
        return Ok(id);
    }
    
    Err(DeployError::ApiError(format!(
        "No performance profile found for label '{}' with provider '{}' version '{}'",
        label_name, database_provider, database_version
    )))
}

// Pure selector to make the matching logic unit-testable without HTTP
pub(crate) fn select_profile_id(
    profiles: &Vec<PerformanceProfile>,
    label_name: &str,
    database_provider: &str,
    database_version: &str,
) -> Option<String> {
    // First try to find exact match with provider and version
    if let Some(profile) = profiles.iter().find(|p| {
        p.label_name == label_name &&
        p.database_provider == database_provider &&
        p.database_version == database_version
    }) {
        return Some(profile.id.clone());
    }
    
    // If no exact match, try to find by label name only
    if let Some(profile) = profiles.iter().find(|p| p.label_name == label_name) {
        return Some(profile.id.clone());
    }
    
    // If still no match, try to find default profile for the provider/version
    if let Some(profile) = profiles.iter().find(|p| {
        p.is_default &&
        p.database_provider == database_provider &&
        p.database_version == database_version
    }) {
        return Some(profile.id.clone());
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_performance_profiles_session_error() {
        let config = Config { api_url: "https://api.guepard.run".to_string() };

        let mut mock_auth = MockAuthProvider::new();
        mock_auth
            .expect_get_auth_token()
            .times(1)
            .returning(|| {
                Err(crate::domain::errors::config_error::ConfigError::SessionError(
                    "You need to log in first! Run `guepard login` to get started. 🐆".to_string(),
                ))
            });

        let result = list_performance_profiles_with_deps(&config, &mock_auth).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            DeployError::SessionError(msg) => assert!(msg.contains("You need to log in first!")),
            _ => panic!("Expected SessionError"),
        }
    }

    #[tokio::test]
    async fn test_list_performance_profiles_network_or_auth_error() {
        let config = Config { api_url: "https://api.guepard.run".to_string() };

        let mut mock_auth = MockAuthProvider::new();
        mock_auth
            .expect_get_auth_token()
            .times(1)
            .returning(|| Ok("test-jwt-token".to_string()));

        // We are not mocking HTTP; depending on environment this may be a network error
        // or an API error like 403. Accept both as valid outcomes for this test.
        let result = list_performance_profiles_with_deps(&config, &mock_auth).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            DeployError::ApiError(_) | DeployError::SessionError(_) => {}
            other => panic!("Unexpected error: {:?}", other),
        }
    }

    #[test]
    fn test_select_profile_id_matching_logic() {
        let profiles = vec![
            PerformanceProfile {
                id: "1".to_string(),
                label_name: "small".to_string(),
                description_text: "Small profile".to_string(),
                database_provider: "PostgreSQL".to_string(),
                database_version: "16".to_string(),
                min_cpu: 1,
                min_memory: 512,
                is_default: false,
                is_active: true,
            },
            PerformanceProfile {
                id: "2".to_string(),
                label_name: "medium".to_string(),
                description_text: "Medium profile".to_string(),
                database_provider: "PostgreSQL".to_string(),
                database_version: "16".to_string(),
                min_cpu: 2,
                min_memory: 1024,
                is_default: true,
                is_active: true,
            },
            PerformanceProfile {
                id: "3".to_string(),
                label_name: "small".to_string(),
                description_text: "Small MySQL".to_string(),
                database_provider: "mysql".to_string(),
                database_version: "8".to_string(),
                min_cpu: 1,
                min_memory: 512,
                is_default: false,
                is_active: true,
            },
        ];

        // Exact match
        assert_eq!(
            select_profile_id(&profiles, "small", "PostgreSQL", "16"),
            Some("1".to_string())
        );

        // Fallback to label-only
        assert_eq!(
            select_profile_id(&profiles, "medium", "mysql", "8"),
            Some("2".to_string())
        );

        // Fallback to provider/version default
        assert_eq!(
            select_profile_id(&profiles, "nonexistent", "PostgreSQL", "16"),
            Some("2".to_string())
        );

        // No match
        assert_eq!(
            select_profile_id(&profiles, "nonexistent", "mysql", "5.7"),
            None
        );
    }
}
