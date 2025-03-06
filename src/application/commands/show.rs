use crate::application::services::show_service;
use crate::config::config::Config;
use crate::structure::GetDeployArgs;
use anyhow::Result;

pub async fn show_branches(args: &GetDeployArgs, config: &Config) -> Result<()> {
    let (branches, active_branch_id) = show_service::list_branches_with_active(&args.deployment_id, config).await?;

    println!("Branches:");
    for branch in branches {
        let marker = if branch.clone_id == active_branch_id { //  Use clone_id to match attached_branch 
            "🐆" 
        } else {
            "  "
        };
        println!("{} {}", marker, branch.name); 
    }
    Ok(())
}

pub async fn show_bookmarks(args: &GetDeployArgs, config: &Config) -> Result<()> {
    let (bookmarks, active_snapshot_id) = show_service::list_bookmarks_with_active(&args.deployment_id, config).await?;

    println!("Bookmarks:");
    for bookmark in bookmarks {
        let marker = if bookmark.id == active_snapshot_id {
            "🐆"
        } else {
            "  "
        };
        println!("{} {}", marker, bookmark.name);
    }
    Ok(())
}