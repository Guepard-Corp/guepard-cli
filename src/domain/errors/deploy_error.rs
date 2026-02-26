use anyhow::Result;
use reqwest::StatusCode;
use thiserror::Error;
use serde_json;

#[derive(Error, Debug)]
pub enum DeployError {
    #[error("{0}")]
    ApiError(String),

    #[error("Request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Failed to parse response: {0}")]
    ParseError(String), 

    #[error("400 Bad Request: {0}")]
    BadRequest(String),

    #[error("403 Forbidden: Invalid API token or permissions")]
    Forbidden,

    #[error("404 Not Found: The requested resource was not found")]
    NotFound,

    #[error("500 Internal Server Error: Something went wrong on the server")]
    InternalServerError,

    #[error("503 Service Unavailable: The API is currently down")]
    ServiceUnavailable,

    #[error("Unexpected response: {0}")]
    Unexpected(String),

    #[error("{0}")]
    SessionError(String),
}

impl DeployError {
    pub async fn from_response(response: reqwest::Response) -> Self {
        let status = response.status();
        let text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());

        // Try to parse JSON error response
        let error_message = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(msg) = json.get("message") {
                if msg.is_string() {
                    msg.as_str().unwrap_or("").to_string()
                } else if msg.is_object() {
                    serde_json::to_string_pretty(msg).unwrap_or_else(|_| format!("{:?}", msg))
                } else {
                    format!("{}", msg)
                }
            } else if let Some(errors) = json.get("errors") {
                serde_json::to_string_pretty(errors).unwrap_or_else(|_| format!("{:?}", errors))
            } else {
                serde_json::to_string_pretty(&json).unwrap_or(text)
            }
        } else {
            text
        };

        match status {
            StatusCode::BAD_REQUEST => DeployError::BadRequest(error_message),
            StatusCode::FORBIDDEN => DeployError::Forbidden,
            StatusCode::NOT_FOUND => DeployError::NotFound,
            StatusCode::INTERNAL_SERVER_ERROR => DeployError::InternalServerError,
            StatusCode::SERVICE_UNAVAILABLE => DeployError::ServiceUnavailable,
            _ => DeployError::Unexpected(format!("{}: {}", status, error_message)),
        }
    }
}

/// Handles DELETE /deploy/:id response: returns parsed body on 200, Err with message and purge_result on non-200.
pub async fn handle_delete_response(response: reqwest::Response) -> Result<serde_json::Value> {
    let status = response.status();
    let text = response.text().await.unwrap_or_default();
    if status == reqwest::StatusCode::OK {
        serde_json::from_str(&text).map_err(|e| anyhow::anyhow!("Failed to parse response: {}", e))
    } else {
        let err_msg = build_error_message(status, &text);
        Err(anyhow::anyhow!("{}", err_msg))
    }
}

fn build_error_message(status: reqwest::StatusCode, text: &str) -> String {
    let (mut msg, purge_stderr, purge_stdout, steps) = if let Ok(json) = serde_json::from_str::<serde_json::Value>(text) {
        let m = json.get("message").and_then(|x| x.as_str()).unwrap_or("").to_string();
        let m = if m.is_empty() && status == reqwest::StatusCode::INTERNAL_SERVER_ERROR {
            "Server error".to_string()
        } else {
            m
        };
        let stderr = json.get("purge_result").and_then(|p| p.get("stderr")).and_then(|s| s.as_str()).map(String::from);
        let stdout = json.get("purge_result").and_then(|p| p.get("stdout")).and_then(|s| s.as_str()).map(String::from);
        let steps: Option<Vec<String>> = json.get("steps")
            .and_then(|s| s.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect());
        (m, stderr, stdout, steps)
    } else {
        (text.to_string(), None, None, None)
    };
    if let Some(s) = &steps {
        if !s.is_empty() {
            msg.push_str("\n\n--- purge details ---\n");
            for step in s {
                msg.push_str("  ");
                msg.push_str(step);
                msg.push('\n');
            }
        }
    }
    if let Some(s) = purge_stderr {
        msg.push_str("\n--- Purge stderr ---\n");
        msg.push_str(&s);
    }
    if let Some(s) = purge_stdout {
        msg.push_str("\n--- Purge stdout ---\n");
        msg.push_str(&s);
    }
    if std::env::var("GUEPARD_DEBUG").is_ok() && !text.is_empty() {
        msg.push_str("\n\n--- Raw API response ---\n");
        msg.push_str(text);
    }
    format!("❌ {}: {}", status, msg)
}

/// handles API responses
pub async fn handle_api_response(response: reqwest::Response) -> Result<()> {
    match response.status() {
        reqwest::StatusCode::OK => {
            Ok(())
        }
        reqwest::StatusCode::BAD_REQUEST => {
            let text = response
                .text()
                .await
                .unwrap_or("Invalid request data".to_string());
            
            // Try to parse JSON error response
            let err_msg = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                if let Some(msg) = json.get("message") {
                    if msg.is_string() {
                        msg.as_str().unwrap_or("").to_string()
                    } else if msg.is_object() {
                        serde_json::to_string_pretty(msg).unwrap_or_else(|_| format!("{:?}", msg))
                    } else {
                        format!("{}", msg)
                    }
                } else if let Some(errors) = json.get("errors") {
                    serde_json::to_string_pretty(errors).unwrap_or_else(|_| format!("{:?}", errors))
                } else {
                    serde_json::to_string_pretty(&json).unwrap_or(text)
                }
            } else {
                text
            };
            
            Err(anyhow::anyhow!("❌ 400 Bad Request: {}", err_msg))
        }
        reqwest::StatusCode::UNAUTHORIZED | reqwest::StatusCode::FORBIDDEN => {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            let detail = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                json.get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("Invalid API token or permissions")
                    .to_string()
            } else if text.is_empty() {
                "Invalid API token or permissions".to_string()
            } else {
                text
            };
            let status_str = if status == reqwest::StatusCode::UNAUTHORIZED {
                "401 Unauthorized"
            } else {
                "403 Forbidden"
            };
            Err(anyhow::anyhow!("❌ {}: {}", status_str, detail))
        }
        reqwest::StatusCode::SERVICE_UNAVAILABLE => Err(anyhow::anyhow!(
            "❌ 503 Service Unavailable: The API is currently down"
        )),
        reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
            let status = response.status();
            let text = response.text().await.unwrap_or("Unknown error".to_string());
            let err_msg = build_error_message(status, &text);
            Err(anyhow::anyhow!("{}", err_msg))
        }
        _ => {
            let status = response.status();
            let text = response.text().await.unwrap_or("Unknown error".to_string());
            let err_msg = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                if let Some(msg) = json.get("message") {
                    if msg.is_string() {
                        msg.as_str().unwrap_or("").to_string()
                    } else {
                        serde_json::to_string_pretty(&json).unwrap_or(text)
                    }
                } else {
                    serde_json::to_string_pretty(&json).unwrap_or(text)
                }
            } else {
                text
            };
            Err(anyhow::anyhow!("❌ {}: {}", status, err_msg))
        }
    }
}