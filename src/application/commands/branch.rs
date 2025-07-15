use crate::application::dto::branch_dto::BranchRequest;
use crate::application::services::branch_service;
use crate::structure::{CheckoutBranchArgs, CreateBranchArgs};
use anyhow::Result;
use crate::config::config::Config;
use tabled::{Table, Tabled, settings::Style};
use colored::Colorize;

#[derive(Tabled)]
struct BranchRow {
    #[tabled(rename = "Branch ID")]
    id: String,
    #[tabled(rename = "Name")]
    label_name: String,
    #[tabled(rename = "Status")]
    job_status: String,
    #[tabled(rename = "Snapshot ID")]
    snapshot_id: String,
}

pub async fn create(args: &CreateBranchArgs, config: &Config) -> Result<()> {
    let request = BranchRequest {
        branch_name: args.branch_id.clone(), // Use branch_id as branch_name for simplicity
        discard_changes: args.discard_changes,
        checkout: args.checkout,
        ephemeral: args.ephemeral,
    };
    let branch = branch_service::create_branch(
        &args.deployment_id,
        &args.branch_id,
        &args.snapshot_id,
        request,
        config,
    )
    .await?;
    println!(
        "{} Created branch [{}] '{}' ({}) from snapshot [{}] in deployment [{}]",
        "✅".green(),
        branch.id.cyan(),
        branch.branch_name,
        branch.job_status,
        branch.snapshot_id,
        branch.deployment_id
    );
    Ok(())
}

pub async fn list(deployment_id: &str, config: &Config) -> Result<()> {
    let branches = branch_service::list_branches(deployment_id, config).await?;
    if branches.is_empty() {
        println!("{} No branches found for deployment ID: {}", "ℹ️".blue(), deployment_id);
        return Ok(());
    }

    let filtered_branches: Vec<_> = branches.into_iter()
        .filter(|b| !b.is_ephemeral) // Filter out ephemeral branches
        .collect();

    if filtered_branches.is_empty() {
        println!("{} No non-ephemeral branches found for deployment ID: {}", "ℹ️".blue(), deployment_id);
        return Ok(());
    }

    let rows: Vec<BranchRow> = filtered_branches.into_iter().map(|b| BranchRow {
        id: b.id,
        label_name: b.label_name,
        job_status: b.job_status,
        snapshot_id: b.snapshot_id.unwrap_or("None".to_string()),
    }).collect();

    println!("{} Retrieved {} non-ephemeral branches:", "✅".green(), rows.len());
    println!("{}", Table::new(rows).with(Style::rounded()));
    Ok(())
}

pub async fn checkout(args: &CheckoutBranchArgs, config: &Config) -> Result<()> {
    branch_service::checkout_branch(&args.deployment_id, &args.branch_id, config).await?;
    println!(
        "{} Checked out branch [{}] in deployment [{}]",
        "✅".green(),
        args.branch_id.cyan(),
        args.deployment_id
    );
    Ok(())
}
