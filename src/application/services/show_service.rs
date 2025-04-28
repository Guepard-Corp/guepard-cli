use crate::application::dto::branch_dto::ListBranchesResponse;
use crate::application::dto::bookmark_dto::GetBookmarkResponse;
use crate::application::services::branch_service;
use crate::application::services::bookmark_service;
use crate::application::services::compute_service;
use crate::application::services::deploy_service;
use crate::config::config::{Config};
use anyhow::Result;

pub async fn get_active_branch_and_bookmark(deployment_id: &str, config: &Config) -> Result<(String, String)> {

    let deployment = deploy_service::get_deployment(deployment_id, config).await?;
    let compute_id = deployment.clone_id;
    let compute = compute_service::list_compute(deployment_id, &compute_id, config).await?;
    Ok((compute.attached_branch, compute.snapshot_id))
}

pub async fn list_branches_with_active(deployment_id: &str, config: &Config) -> Result<(Vec<ListBranchesResponse>, String)> {
    let (active_branch_id, _) = get_active_branch_and_bookmark(deployment_id, config).await?;
    let branches = branch_service::list_branches(deployment_id, config).await?;
    Ok((branches, active_branch_id))
}

pub async fn list_bookmarks_with_active(deployment_id: &str, config: &Config) -> Result<(Vec<GetBookmarkResponse>, String)> {
    let (_, active_snapshot_id) = get_active_branch_and_bookmark(deployment_id, config).await?;
    let bookmarks = bookmark_service::list_all_bookmarks(deployment_id, config).await?;
    Ok((bookmarks, active_snapshot_id))
}