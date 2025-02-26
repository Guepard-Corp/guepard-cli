use serde::{Deserialize, Serialize};
/// This module contains data transfer objects (DTOs) for deployment requests.
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDeploymentRequest {
    pub database_provider: String,
    pub database_version: String,
    pub region: String,
    pub instance_type: String,
    pub datacenter: String,
    pub repository_name: String,
    pub database_password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateDeploymentRequest {
    pub repository_name: String,
}
