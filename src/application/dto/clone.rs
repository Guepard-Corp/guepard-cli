use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCloneRequest {
    pub repository_name: String,
    pub branch_name: String,
    pub performance_profile_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCloneResponse {
    pub id: String,
    pub name: String,
    pub status: String,
    pub customer_id: String,
    pub clone_id: Option<String>,
    pub snapshot_id: Option<String>,
    pub snapshot_parent: Option<String>,
    pub pipeline_id: Option<String>,
    pub current_clone: Option<String>,
    pub deployment_parent: Option<String>,
    pub deployment_type: String,
    pub repository_name: String,
    pub fqdn: String,
    pub database_provider: String,
    pub database_version: String,
    pub database_username: Option<String>,
    pub database_password: Option<String>,
    pub created_by: String,
    pub created_date: String,
    pub last_modified_by: Option<String>,
    pub last_modified_date: Option<String>,
    pub node_id: Option<String>,
    pub db_role_id: Option<String>,
    pub region: Option<String>,
    pub datacenter: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListClonesResponse {
    pub shadows: Vec<CloneInfo>,
    pub total: Option<i32>,
    pub deployment_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloneInfo {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub deployment_id: String,
    pub snapshot_id: String,
    pub branch_id: Option<String>,
    pub status: String,
    pub created_at: String,
    pub created_by: Option<String>,
}

