//use anyhow::Result;
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BranchError {
    #[error("API configuration error: {0}")]
    ApiError(String),

    #[error("Request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Failed to parse response: {0}")]
    ParseError(String),

    #[error("400 Bad Request: {0}")]
    BadRequest(String),

    #[error("403 Forbidden: {0}")]
    Forbidden(String), // Changed to include server message

    #[error("404 Not Found: {0}")]
    NotFound(String), // Changed to include server message

    #[error("500 Internal Server Error: {0}")]
    InternalServerError(String), // Changed to include server message

    #[error("503 Service Unavailable: {0}")]
    ServiceUnavailable(String), // Changed to include server message

    #[error("Unexpected error: {0}")]
    Unexpected(String), // For other status codes with server message
}

impl BranchError {
    pub async fn from_response(response: reqwest::Response) -> Self {
        let status = response.status();
        let text = response
            .text()
            .await
            .unwrap_or_else(|_| "No additional details provided by server".to_string());

        match status {
            StatusCode::BAD_REQUEST => BranchError::BadRequest(text),
            StatusCode::FORBIDDEN => BranchError::Forbidden(text), // Use server message
            StatusCode::NOT_FOUND => BranchError::NotFound(text), // Use server message
            StatusCode::INTERNAL_SERVER_ERROR => BranchError::InternalServerError(text),
            StatusCode::SERVICE_UNAVAILABLE => BranchError::ServiceUnavailable(text),
            _ => BranchError::Unexpected(format!("Status {}: {}", status, text)),
        }
    }
}

// Handles API responses for success cases
// pub async fn handle_api_response(response: reqwest::Response) -> Result<()> {
//     match response.status() {
//         reqwest::StatusCode::OK => {
//             let text = response
//                 .text()
//                 .await
//                 .unwrap_or_else(|_| "Success".to_string());
//             println!("✅ Success: {}", text);
//             Ok(())
//         }
//         reqwest::StatusCode::BAD_REQUEST => {
//             let err_msg = response
//                 .text()
//                 .await
//                 .unwrap_or("Invalid request data".to_string());
//             Err(anyhow::anyhow!("❌ 400 Bad Request: {}", err_msg))
//         }
//         reqwest::StatusCode::FORBIDDEN => {
//             let err_msg = response
//                 .text()
//                 .await
//                 .unwrap_or("Invalid API token or permissions".to_string());
//             Err(anyhow::anyhow!("❌ 403 Forbidden: {}", err_msg))
//         }
//         reqwest::StatusCode::SERVICE_UNAVAILABLE => {
//             let err_msg = response
//                 .text()
//                 .await
//                 .unwrap_or("The API is currently down".to_string());
//             Err(anyhow::anyhow!("❌ 503 Service Unavailable: {}", err_msg))
//         }
//         _ => {
//             let err_msg = response
//                 .text()
//                 .await
//                 .unwrap_or("Unknown error".to_string());
//             Err(anyhow::anyhow!("❌ Unexpected Error: {}", err_msg))
//         }
//     }
// }