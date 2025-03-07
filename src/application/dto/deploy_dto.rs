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
#[derive(Debug, Deserialize)]
pub struct ListDeploymentsResponse {
    pub id: String, // Deployment ID
    pub name: String, // Compute name
    pub status: String,
    pub repository_name: String,
    pub fqdn: String,
    pub database_provider: String,
    pub database_version: String,
    pub region: String,
    pub instance_type: String,
    pub created_by: String,
    pub created_date: String,
    pub customer_id: String,
}
#[derive(Debug, Deserialize)]
pub struct GetDeploymentResponse {
    pub id: String,
    pub name: String, // Compute name
    pub status: String,
    pub repository_name: String,
    pub clone_id: String, // represents the   Active compute_id
    pub snapshot_id: String,
    pub fqdn: String,
    pub database_provider: String,
    pub database_version: String,
    pub region: String,
    pub instance_type: String,
    pub created_by: String,
    pub created_date: String,
    pub customer_id: String,
    pub database_password: String,
}
// THE APIS WILL BE UPDATED , The Compute name is the same as the deployment name ( deplotment_name= repository_name)
// Note : in the response of GetDeploymentResponse, the clone_id is the same as the active compute_id , the attributes in the api will change and the dtos will be updated