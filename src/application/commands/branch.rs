use crate::application::dto::branch_dto::BranchRequest;
use crate::application::services::branch_service;
use crate::structure::{CheckoutBranchArgs, CreateBranchArgs};
use anyhow::Result;
use crate::config::config::Config;
use tabled::{Table, Tabled, settings::Style};
use colored::Colorize;

#[derive(Tabled)]
struct BranchRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Snapshot ID")]
    snapshot_id: String,
    #[tabled(rename = "Clone ID")]
    clone_id: String,
}

pub async fn create(args: &CreateBranchArgs, config: &Config) -> Result<()> {
    let request = BranchRequest {
        discard_changes: Some(args.discard_changes.clone()),
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
    "{} Created branch [{}] '{}' ({}) from snapshot [{}] in deployment [{}]",
    "✅".green(),
    branch.id.cyan(),
    branch.name,
    branch.status,
    branch.snapshot_id,
    branch.deployment_id
);
Ok(())
}

pub async fn list(deployment_id: &str, config: &Config) -> Result<()> {
    let branches = branch_service::list_branches(deployment_id, config).await?;
    if branches.is_empty() {
        println!("{} No branches found for deployment ID: {}", "ℹ️".blue(), deployment_id);        return Ok(());
    }
    let rows: Vec<BranchRow> = branches.into_iter().map(|b| BranchRow {
        id: b.id,
        name: b.name,
        status: b.status,
        snapshot_id: b.snapshot_id.unwrap_or("None".to_string()),
        clone_id: b.clone_id,
    }).collect();

    println!("{} Retrieved {} branches:", "✅".green(), rows.len());
    println!("{}", Table::new(rows).with(Style::rounded()));
    Ok(())
}

pub async fn checkout(args: &CheckoutBranchArgs, config: &Config) -> Result<()> {
    let branch = branch_service::checkout_branch(&args.deployment_id, &args.clone_id, config).await?;
    println!(
        "{} Checked out branch [{}] '{}' ({}) from snapshot [{}] in deployment [{}]",
        "✅".green(),
        branch.id.cyan(),
        branch.name,
        branch.status,
        branch.snapshot_id,
        branch.deployment_id
    );
    Ok(())
}
