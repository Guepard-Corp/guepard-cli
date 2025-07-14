use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ListComputeResponse {
    pub id: String, // deployment_id
    pub clone_id: String, // branch_id ( clone_id)
    pub name: String, // compute name
    pub fqdn: String,
    pub connection_string: String,
    pub attached_branch: String, // branch_id compute is on
    pub performance_profile_id: String,
    pub port: i32,
}

// #[derive(Debug, Deserialize)]
// pub struct LogsResponse {
//     pub stdout_logs: String,
//     pub stderr_logs: String,
// }

#[derive(Debug, Deserialize)]
pub struct StatusErrorResponse {
    pub message: String,
}