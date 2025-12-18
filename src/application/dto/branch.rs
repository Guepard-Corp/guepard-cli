use serde::{Deserialize, Serialize};

/// matches the POST and UPDATE body.
#[derive(Debug, Serialize, Deserialize)]
pub struct BranchRequest {
    pub branch_name: Option<String>,
    pub discard_changes: Option<String>,
    pub checkout: bool,
    pub ephemeral: bool,
}

/// Wrapper for checkout API response
#[derive(Debug, Deserialize)]
pub struct CheckoutResponse {
    #[serde(rename = "statusCode")]
    pub status_code: i32,
    pub headers: serde_json::Value,
    pub body: String, // Body is a JSON string, not a BranchResponse object
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutRequest {
    pub snapshot_id: Option<String>,
    pub discard_changes: Option<bool>,
}

/// matches the POST, Checkout responses.
#[derive(Debug, Serialize, Deserialize)]
pub struct BranchResponse {
    pub id: String,  // Branch ID (UUID)
    pub account_id: Option<String>,
    pub label_name: Option<String>,
    pub job_status: Option<String>,
    pub deployment_id: Option<String>,
    pub branch_id: Option<String>,
    pub snapshot_id: Option<String>,
    pub is_ephemeral: Option<bool>,
    pub is_masked: Option<bool>,
    pub is_purged: Option<bool>,
    pub updated_at: Option<String>,
    pub created_at: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

/// matches the GET /deploy/{deployment_id}/branch response.
#[derive(Debug, Serialize, Deserialize)]
pub struct ListBranchesResponse {
    pub id: String, // branch id 
    pub branch_name: Option<String>,
    pub label_name: Option<String>,
    pub job_status: Option<String>,
    pub snapshot_id: String,
    pub deployment_id: String,
    pub account_id: Option<String>,
    pub is_ephemeral: bool,
    pub is_masked: bool,
    pub is_purged: bool,
    pub created_by: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub updated_by: Option<String>,
}