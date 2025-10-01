use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct UsageResponse {
    pub quota_deployments: i32,
    pub quota_snapshots: i32,
    pub quota_clones: i32,
    pub usage_deployments: i32,
    pub usage_snapshots: i32,
    pub usage_clones: i32,
}