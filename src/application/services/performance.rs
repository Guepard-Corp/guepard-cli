use crate::application::auth;
use crate::config::config::Config;
use crate::domain::errors::deploy_error::DeployError;
use anyhow::Context;
use reqwest::Client;
use serde::{Deserialize, Serialize};

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

pub async fn list_performance_profiles(config: &Config) -> Result<Vec<PerformanceProfile>, DeployError> {
    let token = auth::get_auth_token()
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

pub async fn get_performance_profile_by_label(
    label_name: &str,
    database_provider: &str,
    database_version: &str,
    config: &Config,
) -> Result<String, DeployError> {
    let profiles = list_performance_profiles(config).await?;
    
    // First try to find exact match with provider and version
    if let Some(profile) = profiles.iter().find(|p| {
        p.label_name == label_name && 
        p.database_provider == database_provider && 
        p.database_version == database_version
    }) {
        return Ok(profile.id.clone());
    }
    
    // If no exact match, try to find by label name only
    if let Some(profile) = profiles.iter().find(|p| p.label_name == label_name) {
        return Ok(profile.id.clone());
    }
    
    // If still no match, try to find default profile for the provider/version
    if let Some(profile) = profiles.iter().find(|p| {
        p.is_default && 
        p.database_provider == database_provider && 
        p.database_version == database_version
    }) {
        return Ok(profile.id.clone());
    }
    
    Err(DeployError::ApiError(format!(
        "No performance profile found for label '{}' with provider '{}' version '{}'",
        label_name, database_provider, database_version
    )))
}
