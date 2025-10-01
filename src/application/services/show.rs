use crate::application::dto::branch::ListBranchesResponse;
use crate::application::dto::commit::GetCommitResponse;
use crate::application::services::branch;
use crate::application::services::commit;
use crate::application::services::compute;
use crate::application::services::deploy;
use crate::config::config::{Config};
use anyhow::Result;

pub async fn get_active_branch_and_bookmark(deployment_id: &str, config: &Config) -> Result<(String, String)> {

    let _deployment = deploy::get_deployment(deployment_id, config).await?;
    let compute = compute::list_compute(deployment_id, config).await?;
    Ok((compute.attached_branch, compute.snapshot_id))
}

pub async fn list_branches_with_active(deployment_id: &str, config: &Config) -> Result<(Vec<ListBranchesResponse>, String)> {
    let (active_branch_id, _) = get_active_branch_and_bookmark(deployment_id, config).await?;
    let branches = branch::list_branches(deployment_id, config).await?;
    Ok((branches, active_branch_id))
}

pub async fn list_commits_with_active(deployment_id: &str, config: &Config) -> Result<(Vec<GetCommitResponse>, String)> {
    let (_, active_snapshot_id) = get_active_branch_and_bookmark(deployment_id, config).await?;
    let commits = commit::list_all_commits(deployment_id, config).await?;
    Ok((commits, active_snapshot_id))
}