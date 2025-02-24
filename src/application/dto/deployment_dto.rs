use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeploymentDTO {
    pub id: String,
    pub name: String,
    pub status: String,
    pub repository_name: String,
    pub clone_id: String,
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

#[derive(Debug, Deserialize)]
pub struct ListDeploymentDTO {
    pub id: String,
    pub name: String,
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
