use crate::application::dto::deployment_dto::{DeploymentDTO, ListDeploymentDTO};
use crate::application::services::deployment_service;
use crate::domain::errors::deployment_error::DeploymentError;

pub async fn list() -> anyhow::Result<Vec<ListDeploymentDTO>> {
    let deployments = deployment_service::list_deployments().await?;
    Ok(deployments)
}

pub async fn get(deployment_id: &str) -> anyhow::Result<DeploymentDTO> {
    let deployment = deployment_service::get_deployment(deployment_id).await?;
    Ok(deployment)
}
