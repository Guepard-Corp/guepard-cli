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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
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

/// Runtime state for a deployment database (active or pending).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeploymentRuntimeSummary {
    pub deployment_id: String,
    pub name: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deployment_runtime_summary_deserializes() {
        let json = r#"[
            {"deployment_id":"d1","name":"db","status":"enabled","port":5432},
            {"deployment_id":"d2","name":"db2","status":"provisioning"}
        ]"#;
        let items: Vec<DeploymentRuntimeSummary> = serde_json::from_str(json).unwrap();
        assert_eq!(items[0].port, Some(5432));
        assert_eq!(items[1].port, None);
    }
}
