use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ListComputeResponse {
    pub id: String,
    pub repository_name: String,
    pub clone_id: String,
    pub snapshot_id: String,
    pub name: String,
    pub fqdn: String,
    pub connection_string: String,
    pub database_provider: String,
    pub database_version: String,
    pub region: String,
    pub instance_type: String,
    pub is_ephemeral: bool,
    pub attached_branch: String,
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