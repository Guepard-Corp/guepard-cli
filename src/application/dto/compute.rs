use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ListComputeResponse {
    pub id: String, // deployment_id
    pub branch_id: String, // Branch ID Compute is on
    pub name: String, // Compute name
    pub fqdn: String,
    pub connection_string: String,
    pub attached_branch: String, // Branch ID Compute is on
    pub performance_profile_id: String,
    pub port: i32,
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

#[derive(Debug, Deserialize)]
pub struct ComputeStatusResponse {
    pub status: String,
    pub message: Option<String>,
}