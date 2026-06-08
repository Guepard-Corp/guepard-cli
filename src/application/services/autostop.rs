use reqwest::Client;

use crate::application::auth;
use crate::application::dto::autostop::{
    AutostopConfigureRequest, AutostopConfigureResponse, AutostopResponse, AutostopStatusResponse,
};
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

async fn autostop_request_with_deps<A: AuthProvider>(
    deployment_id: &str,
    method: reqwest::Method,
    path_suffix: &str,
    body: Option<&AutostopConfigureRequest>,
    config: &Config,
    auth_provider: &A,
) -> Result<reqwest::Response, DeployError> {
    let jwt_token = auth_provider
        .get_auth_token()
        .map_err(|e| DeployError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    let url = format!(
        "{}/deploy/{}/compute/autostop/{}",
        config.api_url, deployment_id, path_suffix
    );
    let mut request = client
        .request(method, url)
        .header("Authorization", format!("Bearer {}", jwt_token));
    if let Some(payload) = body {
        request = request.json(payload);
    }
    request.send().await.map_err(DeployError::RequestFailed)
}

pub async fn autostop_action_with_deps<A: AuthProvider>(
    deployment_id: &str,
    action: &str,
    config: &Config,
    auth_provider: &A,
) -> Result<AutostopResponse, DeployError> {
    let response =
        autostop_request_with_deps(deployment_id, reqwest::Method::GET, action, None, config, auth_provider)
            .await?;

    if response.status().is_success() {
        response
            .json::<AutostopResponse>()
            .await
            .map_err(|e| DeployError::ParseError(e.to_string()))
    } else {
        Err(DeployError::from_response(response).await)
    }
}

pub async fn autostop_enable(
    deployment_id: &str,
    config: &Config,
) -> Result<AutostopResponse, DeployError> {
    let auth = DefaultAuthProvider;
    autostop_action_with_deps(deployment_id, "enable", config, &auth).await
}

pub async fn autostop_disable(
    deployment_id: &str,
    config: &Config,
) -> Result<AutostopResponse, DeployError> {
    let auth = DefaultAuthProvider;
    autostop_action_with_deps(deployment_id, "disable", config, &auth).await
}

pub async fn autostop_status_with_deps<A: AuthProvider>(
    deployment_id: &str,
    config: &Config,
    auth_provider: &A,
) -> Result<AutostopStatusResponse, DeployError> {
    let response = autostop_request_with_deps(
        deployment_id,
        reqwest::Method::GET,
        "status",
        None,
        config,
        auth_provider,
    )
    .await?;

    if response.status().is_success() {
        response
            .json::<AutostopStatusResponse>()
            .await
            .map_err(|e| DeployError::ParseError(e.to_string()))
    } else {
        Err(DeployError::from_response(response).await)
    }
}

pub async fn autostop_status(
    deployment_id: &str,
    config: &Config,
) -> Result<AutostopStatusResponse, DeployError> {
    let auth = DefaultAuthProvider;
    autostop_status_with_deps(deployment_id, config, &auth).await
}

pub async fn configure_autostop_with_deps<A: AuthProvider>(
    deployment_id: &str,
    idle_duration: &str,
    config: &Config,
    auth_provider: &A,
) -> Result<AutostopConfigureResponse, DeployError> {
    let body = AutostopConfigureRequest {
        idle_duration: idle_duration.to_string(),
    };
    let response = autostop_request_with_deps(
        deployment_id,
        reqwest::Method::PUT,
        "configure",
        Some(&body),
        config,
        auth_provider,
    )
    .await?;

    if response.status().is_success() {
        response
            .json::<AutostopConfigureResponse>()
            .await
            .map_err(|e| DeployError::ParseError(e.to_string()))
    } else {
        Err(DeployError::from_response(response).await)
    }
}

pub async fn configure_autostop(
    deployment_id: &str,
    idle_duration: &str,
    config: &Config,
) -> Result<AutostopConfigureResponse, DeployError> {
    let auth = DefaultAuthProvider;
    configure_autostop_with_deps(deployment_id, idle_duration, config, &auth).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn test_config(uri: &str) -> Config {
        Config {
            api_url: uri.to_string(),
            app_url: "https://app.guepard.run".to_string(),
        }
    }

    #[tokio::test]
    async fn test_autostop_enable_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/deploy/dep-1/compute/autostop/enable"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "action": "enable",
                "message": "Autostop enabled"
            })))
            .mount(&server)
            .await;

        let mut auth = MockAuthProvider::new();
        auth.expect_get_auth_token()
            .times(1)
            .returning(|| Ok("test-token".to_string()));

        let result =
            autostop_action_with_deps("dep-1", "enable", &test_config(&server.uri()), &auth)
                .await
                .unwrap();
        assert_eq!(result.action, "enable");
        assert_eq!(result.message, "Autostop enabled");
    }

    #[tokio::test]
    async fn test_autostop_disable_skipped() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/deploy/dep-1/compute/autostop/disable"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "action": "disable",
                "message": "Already disabled",
                "skipped": true
            })))
            .mount(&server)
            .await;

        let mut auth = MockAuthProvider::new();
        auth.expect_get_auth_token()
            .times(1)
            .returning(|| Ok("test-token".to_string()));

        let result =
            autostop_action_with_deps("dep-1", "disable", &test_config(&server.uri()), &auth)
                .await
                .unwrap();
        assert_eq!(result.skipped, Some(true));
    }

    #[tokio::test]
    async fn test_autostop_status_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/deploy/dep-1/compute/autostop/status"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "deployment_id": "dep-1",
                "autostop": true,
                "idle_duration": "30m"
            })))
            .mount(&server)
            .await;

        let mut auth = MockAuthProvider::new();
        auth.expect_get_auth_token()
            .times(1)
            .returning(|| Ok("test-token".to_string()));

        let result = autostop_status_with_deps("dep-1", &test_config(&server.uri()), &auth)
            .await
            .unwrap();
        assert!(result.autostop);
        assert_eq!(result.idle_duration, "30m");
    }

    #[tokio::test]
    async fn test_configure_autostop_success() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/deploy/dep-1/compute/autostop/configure"))
            .and(header("authorization", "Bearer test-token"))
            .and(body_json(serde_json::json!({ "idle_duration": "45m" })))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "job_id": "job-1",
                "idle_duration": "45m",
                "message": "Idle duration updated"
            })))
            .mount(&server)
            .await;

        let mut auth = MockAuthProvider::new();
        auth.expect_get_auth_token()
            .times(1)
            .returning(|| Ok("test-token".to_string()));

        let result =
            configure_autostop_with_deps("dep-1", "45m", &test_config(&server.uri()), &auth)
                .await
                .unwrap();
        assert_eq!(result.idle_duration, "45m");
        assert_eq!(result.job_id, "job-1");
    }

    #[tokio::test]
    async fn test_autostop_bad_request_not_running() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/deploy/dep-1/compute/autostop/enable"))
            .respond_with(
                ResponseTemplate::new(400)
                    .set_body_json(serde_json::json!({ "message": "Compute is not running" })),
            )
            .mount(&server)
            .await;

        let mut auth = MockAuthProvider::new();
        auth.expect_get_auth_token()
            .times(1)
            .returning(|| Ok("test-token".to_string()));

        let err =
            autostop_action_with_deps("dep-1", "enable", &test_config(&server.uri()), &auth)
                .await
                .unwrap_err();
        match err {
            DeployError::BadRequest(msg) => assert!(msg.contains("not running")),
            _ => panic!("expected BadRequest, got {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_autostop_unauthorized() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/deploy/dep-1/compute/autostop/status"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(serde_json::json!({ "message": "Unauthorized" })),
            )
            .mount(&server)
            .await;

        let mut auth = MockAuthProvider::new();
        auth.expect_get_auth_token()
            .times(1)
            .returning(|| Ok("bad-token".to_string()));

        let err = autostop_status_with_deps("dep-1", &test_config(&server.uri()), &auth)
            .await
            .unwrap_err();
        assert!(matches!(
            err,
            DeployError::Unexpected(_) | DeployError::SessionError(_)
        ));
    }

    #[tokio::test]
    async fn test_configure_autostop_bad_request_invalid_duration() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/deploy/dep-1/compute/autostop/configure"))
            .respond_with(
                ResponseTemplate::new(400).set_body_json(serde_json::json!({
                    "message": "idle_duration must match ^\\d+[smhd]$"
                })),
            )
            .mount(&server)
            .await;

        let mut auth = MockAuthProvider::new();
        auth.expect_get_auth_token()
            .times(1)
            .returning(|| Ok("test-token".to_string()));

        let err =
            configure_autostop_with_deps("dep-1", "bad", &test_config(&server.uri()), &auth)
                .await
                .unwrap_err();
        assert!(matches!(err, DeployError::BadRequest(_)));
    }

    #[tokio::test]
    async fn test_autostop_session_error() {
        let config = test_config("https://api.guepard.run");
        let mut auth = MockAuthProvider::new();
        auth.expect_get_auth_token().times(1).returning(|| {
            Err(
                crate::domain::errors::config_error::ConfigError::SessionError(
                    "You need to log in first!".to_string(),
                ),
            )
        });

        let err =
            autostop_action_with_deps("dep-1", "enable", &config, &auth).await.unwrap_err();
        assert!(matches!(err, DeployError::SessionError(_)));
    }
}
