use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ListComputeResponse {
    pub id: String, // deployment_id
    pub repository_name: String, // deployment_name
    pub clone_id: String,
    pub snapshot_id: String, // Bookmark (snapshot) ID
    pub name: String, // Compute name
    pub fqdn: String,
    pub connection_string: String,
    pub database_provider: String,
    pub database_version: String,
    pub region: String,
    pub instance_type: String,
    pub is_ephemeral: bool,
    pub attached_branch: String, // Branch ID Compute is on
}

#[derive(Debug, Deserialize)]
pub struct LogsResponse {
    pub stdout_logs: String,
    pub stderr_logs: String,
}

#[derive(Debug, Deserialize)]
pub struct StatusErrorResponse {
    pub message: String,
}