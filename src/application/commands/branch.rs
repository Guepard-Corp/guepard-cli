use crate::application::dto::branch_dto::BranchRequest;
use crate::application::services::branch_service;
use crate::structure::{CheckoutBranchArgs, CreateBranchArgs, UpdateBranchArgs};
use anyhow::Result;
use crate::config::config::Config;

pub async fn create(args: &CreateBranchArgs, config: &Config) -> Result<()> {
    let request = BranchRequest {
        discard_changes: args.discard_changes.clone(),
        checkout: args.checkout,
        ephemeral: args.ephemeral,
    };
    let branch = branch_service::create_branch(
        &args.deployment_id,
        &args.clone_id,
        &args.snapshot_id,
        request,
        config, 
    )
    .await?;
    println!(
        "✅ Branch Created:\n\
         ID: {}\nName: {}\nStatus: {}\nSnapshot ID: {}\nDeployment ID: {}\n\
         Database: {}\nUsername: {}\nPassword: {}\nEphemeral: {}\nMasked: {}\nPurged: {}\n\
         Created By: {}\nCreated Date: {}",
        branch.id,
        branch.name,
        branch.status,
        branch.snapshot_id,
        branch.deployment_id,
        branch.database_provider,
        branch.database_username,
        branch.database_password,
        branch.is_ephemeral,
        branch.is_masked,
        branch.is_purged,
        branch.created_by,
        branch.created_date
    );
    Ok(())
}

pub async fn list(deployment_id: &str, config: &Config) -> Result<()> {
    let branches = branch_service::list_branches(deployment_id, config).await?;
    if branches.is_empty() {
        println!("ℹ️ No branches found for deployment ID: {}", deployment_id);
        return Ok(());
    }
    println!("✅ Retrieved {} branches:", branches.len());
    for (i, branch) in branches.iter().enumerate() {
        println!(
            "Branch #{}:\n\
             ID: {}\nName: {}\nStatus: {}\nSnapshot ID: {}\nDeployment ID: {}\n\
             Database: {}\nEphemeral: {}\nMasked: {}\nPurged: {}\n\
             Created By: {}\nCreated Date: {}\nClone ID: {}",
            i + 1,
            branch.id,
            branch.name,
            branch.status,
            branch.snapshot_id,
            branch.deployment_id,
            branch.database_provider,
            branch.is_ephemeral,
            branch.is_masked,
            branch.is_purged,
            branch.created_by,
            branch.created_date,
            branch.clone_id
        );
    }
    Ok(())
}

pub async fn checkout(args: &CheckoutBranchArgs, config: &Config) -> Result<()> {
    let branch = branch_service::checkout_branch(&args.deployment_id, &args.clone_id, config).await?;
    println!(
        "✅ Branch Checked Out:\n\
         ID: {}\nName: {}\nStatus: {}\nSnapshot ID: {}\nDeployment ID: {}\n\
         Database: {}\nUsername: {}\nPassword: {}\nEphemeral: {}\nMasked: {}\nPurged: {}\n\
         Created By: {}\nCreated Date: {}",
        branch.id,
        branch.name,
        branch.status,
        branch.snapshot_id,
        branch.deployment_id,
        branch.database_provider,
        branch.database_username,
        branch.database_password,
        branch.is_ephemeral,
        branch.is_masked,
        branch.is_purged,
        branch.created_by,
        branch.created_date
    );
    Ok(())
}

pub async fn update(args: &UpdateBranchArgs, config: &Config) -> Result<()> {
    let request = BranchRequest {
        discard_changes: args.discard_changes.clone(),
        checkout: args.checkout,
        ephemeral: args.ephemeral,
    };
    let branch = branch_service::update_branch(
        &args.deployment_id,
        &args.clone_id,
        &args.snapshot_id,
        request,
        config,
    )
    .await?;
    println!(
        "✅ Branch Updated:\n\
         ID: {}\nName: {}\nStatus: {}\nSnapshot ID: {}\nDeployment ID: {}\n\
         Database: {}\nUsername: {}\nPassword: {}\nEphemeral: {}\nMasked: {}\nPurged: {}\n\
         Created By: {}\nCreated Date: {}",
        branch.id,
        branch.name,
        branch.status,
        branch.snapshot_id,
        branch.deployment_id,
        branch.database_provider,
        branch.database_username,
        branch.database_password,
        branch.is_ephemeral,
        branch.is_masked,
        branch.is_purged,
        branch.created_by,
        branch.created_date
    );
    Ok(())
}
