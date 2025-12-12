use serde::{Deserialize, Serialize};

/// This module contains data transfer objects (DTOs) for deployment requests.
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDeploymentRequest {
    pub repository_name: String,
    pub database_provider: String,
    pub database_version: String,
    pub deployment_type: String, // REPOSITORY or F2
    pub region: String,
    pub datacenter: String,
    pub database_username: String,
    pub database_password: String,
    pub performance_profile_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateDeploymentRequest {
    pub repository_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeploymentResponse {
    pub id: String,
    pub name: String,
    pub repository_name: String,
    pub status: String,
    pub deployment_type: String,
    pub database_provider: String,
    pub database_version: String,
    pub fqdn: String,
    pub port: Option<i32>,
    pub connection_string: Option<String>,
    pub database_username: String,
    pub database_password: String,
    pub region: String,
    pub datacenter: String,
    pub created_date: String,
    pub created_by: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListDeploymentsResponse {
    pub id: String,
    pub name: String,
    pub repository_name: String,
    pub status: String,
    pub deployment_type: String,
    pub database_provider: String,
    pub database_version: String,
    pub fqdn: String,
    pub port: Option<i32>,
    pub connection_string: Option<String>,
    pub region: String,
    pub datacenter: String,
    pub created_date: String,
    pub created_by: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDeploymentResponse {
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
    pub port: Option<i32>,
    pub connection_string: Option<String>,
    pub database_provider: String,
    pub database_version: String,
    pub database_username: String,
    pub database_password: String,
    pub created_by: String,
    pub created_date: String,
    pub last_modified_by: Option<String>,
    pub last_modified_date: Option<String>,
    pub node_id: Option<String>,
    pub db_role_id: Option<String>,
    pub branch_id: Option<String>,
    pub region: String,
    pub datacenter: String,
}