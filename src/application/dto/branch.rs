use serde::{Deserialize, Serialize};

/// matches the POST and UPDATE body.
#[derive(Debug, Serialize, Deserialize)]
pub struct BranchRequest {
    pub discard_changes: Option<String>,
    pub checkout: bool,
    pub ephemeral: bool,
}

/// matches the POST, Checkout responses.
#[derive(Debug, Deserialize)]
pub struct BranchResponse {
    pub id: String,  // Branch ID (UUID)
    pub name: String, // Branch name
    pub status: String,
    pub snapshot_id: String, // Source snapshot
    pub deployment_id: String,
    pub environment_type: Option<String>,
    pub database_provider: String,
    pub is_ephemeral: bool,
    pub is_masked: bool,
    pub is_purged: bool,
    pub created_by: String,
    pub created_date: String,
}

/// matches the GET /deploy/{deployment_id}/branch response.
#[derive(Debug, Deserialize)]
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