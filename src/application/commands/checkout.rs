use anyhow::Result;
use crate::config::config::Config;
use crate::structure::{CheckoutArgs, CheckoutBranchArgs};
use crate::application::services::branch;
use colored::Colorize;
use tabled::{Table, Tabled, settings::Style};

#[derive(Tabled)]
struct CheckoutRow {
    #[tabled(rename = "Branch ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Snapshot ID")]
    snapshot_id: String,
    #[tabled(rename = "Environment")]
    environment_type: String,
}

pub async fn checkout(args: &CheckoutArgs, config: &Config) -> Result<()> {
    if let Some(deployment_id) = &args.deployment_id {
        if let Some(branch_id) = &args.clone_id {
            // Checkout specific branch
            let checkout_args = CheckoutBranchArgs {
                deployment_id: deployment_id.clone(),
                branch_id: branch_id.clone(),
            };
            checkout_branch(&checkout_args, config).await?;
        } else {
            // List available branches for checkout
            println!("{} Available branches for deployment: {}", "🌿".blue(), deployment_id);
            list_branches_for_checkout(deployment_id, config).await?;
        }
    } else {
        // Show help for checkout command
        println!("{} Checkout command requires deployment ID", "💡".yellow());
        println!("{} Usage: gfs checkout -x <deployment_id> -c <branch_id>", "ℹ️".blue());
        println!("{} Or: gfs checkout -x <deployment_id> (to list available branches)", "ℹ️".blue());
    }
    Ok(())
}

pub async fn checkout_branch(args: &CheckoutBranchArgs, config: &Config) -> Result<()> {
    let branch = branch::checkout_branch(&args.deployment_id, &args.branch_id, config).await?;
    
    let checkout_row = CheckoutRow {
        id: branch.id,
        name: branch.name,
        status: branch.status,
        snapshot_id: branch.snapshot_id,
        environment_type: branch.environment_type.unwrap_or("development".to_string()),
    };
    
    println!("{} Checked out branch successfully!", "✅".green());
    println!("{}", Table::new(vec![checkout_row]).with(Style::rounded()));
    Ok(())
}

async fn list_branches_for_checkout(deployment_id: &str, config: &Config) -> Result<()> {
    let branches = branch::list_branches(deployment_id, config).await?;
    
    if branches.is_empty() {
        println!("{} No branches available for checkout", "ℹ️".blue());
        return Ok(());
    }
    
    let rows: Vec<CheckoutRow> = branches.into_iter().map(|b| CheckoutRow {
        id: b.id,
        name: b.name,
        status: b.status,
        snapshot_id: b.snapshot_id,
        environment_type: b.environment_type.unwrap_or("development".to_string()),
    }).collect();
    
    println!("{} Use 'gfs checkout -x {} -c <branch_id>' to checkout a branch", "💡".yellow(), deployment_id);
    println!("{}", Table::new(rows).with(Style::rounded()));
    Ok(())
}