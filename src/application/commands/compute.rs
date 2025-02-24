use crate::application::dto::compute_dto::{ComputeDTO, ComputeStatus};
use crate::application::services::compute_service;
use crate::domain::errors::compute_error::ComputeError;

pub async fn get(deployment_id: &str, clone_id: &str) -> anyhow::Result<ComputeDTO> {
    let compute = compute_service::get_compute(deployment_id, clone_id).await?;
    Ok(compute)
}

pub async fn status(deployment_id: &str, clone_id: &str) -> anyhow::Result<ComputeStatus> {
    let compute_status = compute_service::get_compute_status(deployment_id, clone_id).await?;
    Ok(compute_status)
}

pub async fn start(deployment_id: &str, clone_id: &str) -> anyhow::Result<ComputeDTO> {
    let compute = compute_service::start_compute(deployment_id, clone_id).await?;
    Ok(compute)
}

pub async fn stop(deployment_id: &str, clone_id: &str) -> anyhow::Result<ComputeDTO> {
    let compute = compute_service::stop_compute(deployment_id, clone_id).await?;
    Ok(compute)
}
