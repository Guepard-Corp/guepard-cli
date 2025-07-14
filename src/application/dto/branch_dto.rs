use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BranchRequest {
    pub branch_name: String,
    pub discard_changes: bool,
    pub checkout: bool,
    pub ephemeral: bool,
}

#[derive(Debug, Deserialize)]
pub struct BranchResponse {
    pub id: String, // branch ID
    pub account_id: String,
    pub deployment_id: String,
    pub branch_name: String,
    pub job_status: String, 
    pub snapshot_id: String, // source bookmark
    pub is_ephemeral: bool,
    pub is_masked: bool,
    pub is_purged: bool,
    pub created_by: String,
    pub created_at: Option<String>,
    pub updated_by: Option<String>,
    pub updated_at: Option<String>,
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