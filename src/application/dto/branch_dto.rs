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
    pub snapshot_id: String, // Source bookmark
    pub deployment_id: String,
    pub environment_type: Option<String>,
    pub database_provider: String,
    pub database_username: String,
    pub database_password: String,
    pub is_ephemeral: bool,
    pub is_masked: bool,
    pub is_purged: bool,
    pub created_by: String,
    pub created_date: String,
}

#[derive(Debug, Deserialize)]
pub struct ListBranchesResponse {
    pub id: String, // branch ID 
    pub account_id: String,
    pub deployment_id: String,
    pub label_name: String, 
    pub job_status: String,
    pub snapshot_id: Option<String>, // source bookmark
    pub is_ephemeral: bool,
    pub is_masked: bool,
    pub is_purged: bool,
    pub created_by: String,
    pub created_at: Option<String>,
    pub updated_by: Option<String>,
    pub updated_at: Option<String>,
}