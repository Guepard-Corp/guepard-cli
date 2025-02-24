use crate::application::dto::branch_dto::{BranchDTO, NewBranchDTO, CheckoutBranchDTO};
use crate::application::services::branch_service;
use crate::domain::errors::branch_error::BranchError;

pub async fn list(deployment_id: &str) -> anyhow::Result<Vec<BranchDTO>> {
    let branches = branch_service::list_branches(deployment_id).await?;
    Ok(branches)
}

pub async fn create(deployment_id: &str, clone_id: &str, snapshot_id: &str) -> anyhow::Result<NewBranchDTO> {
    let branch = branch_service::create_branch(deployment_id, clone_id, snapshot_id).await?;
    Ok(branch)
}

pub async fn checkout(deployment_id: &str, clone_id: &str) -> anyhow::Result<CheckoutBranchDTO> {
    let branch = branch_service::checkout_branch(deployment_id, clone_id).await?;
    Ok(branch)
}
