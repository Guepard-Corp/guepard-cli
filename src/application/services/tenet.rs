use crate::application::auth;
use crate::application::dto::tenet::{
    TenetDeployRequest, TenetDeployResponse, TenetLifecycleResponse, TenetProxyYamlSetRequest,
};
use crate::config::config::Config;
use crate::domain::errors::tenet_error::TenetError;

use reqwest::{Client, StatusCode};

#[cfg_attr(test, mockall::automock)]
pub trait TenetAuthProvider {
    fn get_auth_token(&self) -> Result<String, crate::domain::errors::config_error::ConfigError>;
}

pub struct DefaultTenetAuthProvider;

impl TenetAuthProvider for DefaultTenetAuthProvider {
    fn get_auth_token(&self) -> Result<String, crate::domain::errors::config_error::ConfigError> {
        auth::get_auth_token()
    }
}

/// 202: the API registered the change but answered before the proxy was running again.
fn is_accepted(status: StatusCode) -> bool {
    status == StatusCode::OK || status == StatusCode::ACCEPTED
}

async fn read_error_body(response: reqwest::Response) -> String {
    response
        .text()
        .await
        .unwrap_or_else(|_| "No details".to_string())
}

pub async fn deploy_tenet_with_deps<A: TenetAuthProvider>(
    body: &TenetDeployRequest,
    config: &Config,
    auth_provider: &A,
) -> Result<TenetDeployResponse, TenetError> {
    let jwt_token = auth_provider
        .get_auth_token()
        .map_err(|e| TenetError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    let response = client
        .post(format!("{}/tenet/deploy", config.api_url))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .header("Content-Type", "application/json")
        .json(body)
        .send()
        .await
        .map_err(TenetError::RequestFailed)?;

    match response.status() {
        status if is_accepted(status) => response
            .json::<TenetDeployResponse>()
            .await
            .map_err(|e| TenetError::ParseError(e.to_string())),
        status => {
            let text = read_error_body(response).await;
            Err(TenetError::Unexpected(format!(
                "Status {}: {}",
                status, text
            )))
        }
    }
}

pub async fn deploy_tenet(
    body: &TenetDeployRequest,
    config: &Config,
) -> Result<TenetDeployResponse, TenetError> {
    let auth = DefaultTenetAuthProvider;
    deploy_tenet_with_deps(body, config, &auth).await
}

pub async fn start_tenet_with_deps<A: TenetAuthProvider>(
    job_id: &str,
    config: &Config,
    auth_provider: &A,
) -> Result<TenetLifecycleResponse, TenetError> {
    lifecycle_post(job_id, "start", config, auth_provider).await
}

pub async fn start_tenet(
    job_id: &str,
    config: &Config,
) -> Result<TenetLifecycleResponse, TenetError> {
    let auth = DefaultTenetAuthProvider;
    start_tenet_with_deps(job_id, config, &auth).await
}

pub async fn stop_tenet_with_deps<A: TenetAuthProvider>(
    job_id: &str,
    config: &Config,
    auth_provider: &A,
) -> Result<TenetLifecycleResponse, TenetError> {
    lifecycle_post(job_id, "stop", config, auth_provider).await
}

pub async fn stop_tenet(
    job_id: &str,
    config: &Config,
) -> Result<TenetLifecycleResponse, TenetError> {
    let auth = DefaultTenetAuthProvider;
    stop_tenet_with_deps(job_id, config, &auth).await
}

pub async fn purge_tenet_with_deps<A: TenetAuthProvider>(
    job_id: &str,
    config: &Config,
    auth_provider: &A,
) -> Result<TenetLifecycleResponse, TenetError> {
    lifecycle_post(job_id, "purge", config, auth_provider).await
}

pub async fn purge_tenet(
    job_id: &str,
    config: &Config,
) -> Result<TenetLifecycleResponse, TenetError> {
    let auth = DefaultTenetAuthProvider;
    purge_tenet_with_deps(job_id, config, &auth).await
}

async fn lifecycle_post<A: TenetAuthProvider>(
    job_id: &str,
    segment: &str,
    config: &Config,
    auth_provider: &A,
) -> Result<TenetLifecycleResponse, TenetError> {
    let jwt_token = auth_provider
        .get_auth_token()
        .map_err(|e| TenetError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    let url = format!("{}/tenet/{}/{}", config.api_url, job_id, segment);
    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
        .map_err(TenetError::RequestFailed)?;

    match response.status() {
        status if is_accepted(status) => {
            let text = response
                .text()
                .await
                .map_err(|e| TenetError::ParseError(e.to_string()))?;
            if text.trim().is_empty() {
                return Ok(TenetLifecycleResponse::default());
            }
            match serde_json::from_str::<TenetLifecycleResponse>(&text) {
                Ok(v) => Ok(v),
                Err(_) => Ok(TenetLifecycleResponse {
                    message: Some(text),
                    status: None,
                }),
            }
        }
        status => {
            let text = read_error_body(response).await;
            Err(TenetError::Unexpected(format!(
                "Status {}: {}",
                status, text
            )))
        }
    }
}

pub async fn get_proxy_yaml_with_deps<A: TenetAuthProvider>(
    job_id: &str,
    config: &Config,
    auth_provider: &A,
) -> Result<String, TenetError> {
    let jwt_token = auth_provider
        .get_auth_token()
        .map_err(|e| TenetError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    let response = client
        .get(format!("{}/tenet/{}/proxy.yaml", config.api_url, job_id))
        .header("Authorization", format!("Bearer {}", jwt_token))
        .send()
        .await
        .map_err(TenetError::RequestFailed)?;

    match response.status() {
        StatusCode::OK => response
            .text()
            .await
            .map_err(|e| TenetError::ParseError(e.to_string())),
        status => {
            let text = read_error_body(response).await;
            Err(TenetError::Unexpected(format!(
                "Status {}: {}",
                status, text
            )))
        }
    }
}

pub async fn get_proxy_yaml(job_id: &str, config: &Config) -> Result<String, TenetError> {
    let auth = DefaultTenetAuthProvider;
    get_proxy_yaml_with_deps(job_id, config, &auth).await
}

pub async fn set_proxy_yaml_with_deps<A: TenetAuthProvider>(
    job_id: &str,
    yaml: &str,
    apply: bool,
    upstream_password: Option<&str>,
    config: &Config,
    auth_provider: &A,
) -> Result<TenetLifecycleResponse, TenetError> {
    let jwt_token = auth_provider
        .get_auth_token()
        .map_err(|e| TenetError::SessionError(format!("{}", e)))?;
    let client = Client::new();
    let url = format!(
        "{}/tenet/{}/proxy.yaml?apply={}",
        config.api_url, job_id, apply
    );
    let body = TenetProxyYamlSetRequest {
        config_yaml: yaml.to_string(),
        upstream_password: upstream_password.map(str::to_string),
    };
    let response = client
        .put(url)
        .header("Authorization", format!("Bearer {}", jwt_token))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(TenetError::RequestFailed)?;

    match response.status() {
        status if is_accepted(status) => {
            let text = response
                .text()
                .await
                .map_err(|e| TenetError::ParseError(e.to_string()))?;
            if text.trim().is_empty() {
                return Ok(TenetLifecycleResponse::default());
            }
            match serde_json::from_str::<TenetLifecycleResponse>(&text) {
                Ok(v) => Ok(v),
                Err(_) => Ok(TenetLifecycleResponse {
                    message: Some(text),
                    status: None,
                }),
            }
        }
        status => {
            let text = read_error_body(response).await;
            Err(TenetError::Unexpected(format!(
                "Status {}: {}",
                status, text
            )))
        }
    }
}

pub async fn set_proxy_yaml(
    job_id: &str,
    yaml: &str,
    apply: bool,
    upstream_password: Option<&str>,
    config: &Config,
) -> Result<TenetLifecycleResponse, TenetError> {
    let auth = DefaultTenetAuthProvider;
    set_proxy_yaml_with_deps(job_id, yaml, apply, upstream_password, config, &auth).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_pending_202_counts_as_success() {
        assert!(is_accepted(StatusCode::OK));
        assert!(is_accepted(StatusCode::ACCEPTED));
        assert!(!is_accepted(StatusCode::BAD_GATEWAY));
        assert!(!is_accepted(StatusCode::NO_CONTENT));
    }

    #[tokio::test]
    async fn deploy_session_error() {
        let config = Config {
            api_url: "https://api.guepard.run".to_string(),
            app_url: "https://app.guepard.run".to_string(),
        };
        let mut auth = MockTenetAuthProvider::new();
        auth.expect_get_auth_token().times(1).returning(|| {
            Err(
                crate::domain::errors::config_error::ConfigError::SessionError(
                    "You need to log in first!".to_string(),
                ),
            )
        });

        let body = TenetDeployRequest {
            tenant_id: "t".to_string(),
            compute_job_id: None,
            upstream_host: "h".to_string(),
            upstream_port: 1,
            masking_salt: "s".to_string(),
            config_dir: None,
            config_yaml: "x: 1".to_string(),
            proxy_port: None,
            api_port: None,
            upstream_password: None,
        };
        let r = deploy_tenet_with_deps(&body, &config, &auth).await;
        assert!(matches!(r.unwrap_err(), TenetError::SessionError(_)));
    }

    #[tokio::test]
    async fn network_errors_for_tenet_calls() {
        let config = Config {
            api_url: "https://api.guepard.run".to_string(),
            app_url: "https://app.guepard.run".to_string(),
        };
        let mut auth = MockTenetAuthProvider::new();
        auth.expect_get_auth_token()
            .times(6)
            .returning(|| Ok("token".to_string()));

        let body = TenetDeployRequest {
            tenant_id: "t".to_string(),
            compute_job_id: None,
            upstream_host: "h".to_string(),
            upstream_port: 1,
            masking_salt: "s".to_string(),
            config_dir: None,
            config_yaml: "x".to_string(),
            proxy_port: None,
            api_port: None,
            upstream_password: None,
        };
        assert!(deploy_tenet_with_deps(&body, &config, &auth).await.is_err());
        assert!(start_tenet_with_deps("job", &config, &auth).await.is_err());
        assert!(stop_tenet_with_deps("job", &config, &auth).await.is_err());
        assert!(purge_tenet_with_deps("job", &config, &auth).await.is_err());
        assert!(get_proxy_yaml_with_deps("job", &config, &auth)
            .await
            .is_err());
        assert!(
            set_proxy_yaml_with_deps("job", "a: 1", true, None, &config, &auth)
                .await
                .is_err()
        );
    }
}
