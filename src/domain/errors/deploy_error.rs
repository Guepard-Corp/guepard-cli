use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeployError {
    #[error("API error: {0}")]
    ApiError(String),
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
}

//use reqwest::Client;

/// handles API responses
pub async fn handle_api_response(response: reqwest::Response) -> Result<()> {
    match response.status() {
        reqwest::StatusCode::OK => {
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "Success".to_string());
            println!("✅ Success: {}", text);
            Ok(())
        }
        reqwest::StatusCode::BAD_REQUEST => {
            let err_msg = response
                .text()
                .await
                .unwrap_or("Invalid request data".to_string());
            Err(anyhow::anyhow!("❌ 400 Bad Request: {}", err_msg))
        }
        reqwest::StatusCode::FORBIDDEN => Err(anyhow::anyhow!(
            "❌ 403 Forbidden: Invalid API token or permissions"
        )),
        reqwest::StatusCode::SERVICE_UNAVAILABLE => Err(anyhow::anyhow!(
            "❌ 503 Service Unavailable: The API is currently down"
        )),
        _ => {
            let err_msg = response.text().await.unwrap_or("Unknown error".to_string());
            Err(anyhow::anyhow!("❌ Unexpected Error: {}", err_msg))
        }
    }
}
