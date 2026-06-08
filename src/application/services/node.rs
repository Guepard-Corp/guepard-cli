use reqwest::Client;

use crate::application::auth;
use crate::application::dto::node::{AccessibleNode, NodeResourceAvailability};
use crate::config::config::Config;
use crate::domain::errors::deploy_error::DeployError;

#[cfg_attr(test, mockall::automock)]
pub trait AuthProvider {
    fn get_auth_token(&self) -> Result<String, crate::domain::errors::config_error::ConfigError>;
}

pub struct DefaultAuthProvider;

impl AuthProvider for DefaultAuthProvider {
    fn get_auth_token(&self) -> Result<String, crate::domain::errors::config_error::ConfigError> {
        auth::get_auth_token()
    }
}

async fn fetch_node_resources_with_deps<A: AuthProvider>(
    url: String,
    auth_provider: &A,
) -> Result<NodeResourceAvailability, DeployError> {
    let jwt_token = auth_provider
        .get_auth_token()
        .map_err(|e| DeployError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
        .map_err(DeployError::RequestFailed)?;

    if response.status().is_success() {
        response
            .json::<NodeResourceAvailability>()
            .await
            .map_err(|e| DeployError::ParseError(e.to_string()))
    } else {
        Err(DeployError::from_response(response).await)
    }
}

pub async fn get_account_node_resources_with_deps<A: AuthProvider>(
    config: &Config,
    auth_provider: &A,
) -> Result<NodeResourceAvailability, DeployError> {
    fetch_node_resources_with_deps(
        format!("{}/deploy/node/resources", config.api_url),
        auth_provider,
    )
    .await
}

pub async fn get_account_node_resources(
    config: &Config,
) -> Result<NodeResourceAvailability, DeployError> {
    let auth = DefaultAuthProvider;
    get_account_node_resources_with_deps(config, &auth).await
}

pub async fn get_node_resources_by_id_with_deps<A: AuthProvider>(
    node_id: &str,
    config: &Config,
    auth_provider: &A,
) -> Result<NodeResourceAvailability, DeployError> {
    fetch_node_resources_with_deps(
        format!("{}/deploy/node/{}/resources", config.api_url, node_id),
        auth_provider,
    )
    .await
}

pub async fn get_node_resources(
    config: &Config,
    node_id: Option<&str>,
) -> Result<NodeResourceAvailability, DeployError> {
    let auth = DefaultAuthProvider;
    match node_id {
        Some(id) => get_node_resources_by_id_with_deps(id, config, &auth).await,
        None => get_account_node_resources_with_deps(config, &auth).await,
    }
}

pub async fn list_accessible_nodes_with_deps<A: AuthProvider>(
    config: &Config,
    auth_provider: &A,
) -> Result<Vec<AccessibleNode>, DeployError> {
    let jwt_token = auth_provider
        .get_auth_token()
        .map_err(|e| DeployError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    let response = client
        .get(format!("{}/deploy/accessible-nodes", config.api_url))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
        .map_err(DeployError::RequestFailed)?;

    if response.status().is_success() {
        response
            .json::<Vec<AccessibleNode>>()
            .await
            .map_err(|e| DeployError::ParseError(e.to_string()))
    } else {
        Err(DeployError::from_response(response).await)
    }
}

pub async fn list_accessible_nodes(config: &Config) -> Result<Vec<AccessibleNode>, DeployError> {
    let auth = DefaultAuthProvider;
    list_accessible_nodes_with_deps(config, &auth).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn test_config(uri: &str) -> Config {
        Config {
            api_url: uri.to_string(),
            app_url: "https://app.guepard.run".to_string(),
        }
    }

    fn sample_resources_body() -> serde_json::Value {
        serde_json::json!({
            "node_id": "node-uuid",
            "label_name": "Guepard-Shared-Node",
            "node_type": "public",
            "node_pool": "us-west-aws",
            "datacenter": "us-west-aws",
            "cpu": {"total_mhz":5000,"allocated_mhz":0,"reserved_mhz":0,"available_mhz":5000},
            "memory": {"total_mb":1907,"allocated_mb":0,"reserved_mb":410,"available_mb":1497},
            "schedulable": true
        })
    }

    #[tokio::test]
    async fn test_get_account_node_resources_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/deploy/node/resources"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(sample_resources_body()))
            .mount(&server)
            .await;

        let mut auth = MockAuthProvider::new();
        auth.expect_get_auth_token()
            .times(1)
            .returning(|| Ok("test-token".to_string()));

        let resources = get_account_node_resources_with_deps(&test_config(&server.uri()), &auth)
            .await
            .unwrap();
        assert_eq!(resources.label_name, "Guepard-Shared-Node");
        assert_eq!(resources.node_type, "public");
    }

    #[tokio::test]
    async fn test_get_node_resources_by_id_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/deploy/node/node-uuid/resources"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(sample_resources_body()))
            .mount(&server)
            .await;

        let mut auth = MockAuthProvider::new();
        auth.expect_get_auth_token()
            .times(1)
            .returning(|| Ok("test-token".to_string()));

        let resources =
            get_node_resources_by_id_with_deps("node-uuid", &test_config(&server.uri()), &auth)
                .await
                .unwrap();
        assert!(resources.schedulable);
    }

    #[tokio::test]
    async fn test_get_node_resources_insufficient_conflict() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/deploy/node/resources"))
            .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
                "code": "INSUFFICIENT_NODE_RESOURCES",
                "message": "Not enough CPU on node"
            })))
            .mount(&server)
            .await;

        let mut auth = MockAuthProvider::new();
        auth.expect_get_auth_token()
            .times(1)
            .returning(|| Ok("test-token".to_string()));

        let err = get_account_node_resources_with_deps(&test_config(&server.uri()), &auth)
            .await
            .unwrap_err();
        assert!(matches!(
            err,
            DeployError::InsufficientNodeResources(_)
        ));
    }
}
