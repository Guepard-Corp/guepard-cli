use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDeploymentRequest {
    pub repository_name: String,
    pub database_provider: String,
    pub database_version: String,
    pub deployment_type: String,
    pub region: String,
    pub datacenter: String,
    pub instance_type: String,
    pub database_username: String,
    pub database_password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_profile_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateDeploymentResponse {
    pub id: String,
    pub name: String,
    pub status: String,
    pub customer_id: String,
    pub clone_id: Option<String>,
    pub snapshot_id: String,
    pub snapshot_parent: Option<String>,
    pub pipeline_id: Option<String>,
    pub current_clone: Option<String>,
    pub deployment_parent: Option<String>,
    pub deployment_type: String,
    pub repository_name: String,
    pub fqdn: String,
    pub database_username: String,
    pub database_password: String,
    pub created_by: String,
    pub created_date: String,
    pub last_modified_by: Option<String>,
    pub last_modified_date: Option<String>,
    pub node_id: String,
    pub database_provider: String,
    pub database_version: String,
    pub db_role_id: String,
    pub branch_id: String,
    pub region: String,
    pub datacenter: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateDeploymentRequest {
    pub repository_name: String,
}

#[derive(Debug, Deserialize)]
pub struct ListDeploymentsResponse {
    pub id: String,
    pub name: String,
    pub status: String,
    pub repository_name: String,
    pub fqdn: String,
    pub database_provider: String,
    pub database_version: String,
    pub region: String,
    pub datacenter: String,
    pub deployment_type: String,
    pub database_username: Option<String>,
    // pub instance_type: String,
    pub created_by: String,
    pub created_date: String,
    pub customer_id: String,
    pub branch_id: Option<String>,
    pub gp_node: Value, // Flexible JSON for gp_node
}

#[derive(Debug, Deserialize)]
pub struct GetDeploymentResponse {
    pub id: String,
    pub name: String,
    pub status: String,
    pub repository_name: String,
    pub branch_id: Option<String>,
    pub snapshot_id: Option<String>,
    pub fqdn: String,
    pub database_provider: String,
    pub database_version: String,
    pub region: String,
    pub datacenter: String,
    pub deployment_type: String,
    pub database_username:  Option<String>,
    pub database_password:  Option<String>,
    // pub instance_type: String,
    pub created_by: String,
    pub created_date: String,
    pub customer_id: String,
    pub gp_node: Value, // Flexible JSON for gp_node
}
