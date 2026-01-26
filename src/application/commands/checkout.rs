use anyhow::Result;
use crate::config::config::Config;
use crate::structure::{CheckoutArgs, CheckoutBranchArgs};
use crate::application::services::{branch, commit};
use crate::application::output::{OutputFormat, print_row_or_json, print_table_or_json, print_json};
use colored::Colorize;
use serde::Serialize;
use tabled::{Tabled};

#[derive(Tabled, Serialize)]
struct CheckoutRow {
    #[tabled(rename = "Branch ID")]
    #[serde(rename = "branch_id")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Snapshot ID")]
    #[serde(rename = "snapshot_id")]
    snapshot_id: String,
    #[tabled(rename = "Comment")]
    #[serde(rename = "comment")]
    comment: String,
}

pub async fn checkout(args: &CheckoutArgs, config: &Config, output_format: OutputFormat) -> Result<()> {
    if let Some(deployment_id) = &args.deployment_id {
        if let Some(branch_id) = &args.branch_id {
            // Checkout specific branch
            let checkout_args = CheckoutBranchArgs {
                deployment_id: deployment_id.clone(),
                branch_id: branch_id.clone(),
            };
            checkout_branch(&checkout_args, config, output_format).await?;
        } else if let Some(snapshot_id) = &args.snapshot_id {
            // Restore specific snapshot
            restore_snapshot(deployment_id, snapshot_id, config, output_format).await?;
        } else {
            // List available branches for checkout
            if output_format == OutputFormat::Table {
                println!("{} Available branches for deployment: {}", "🌿".blue(), deployment_id);
            }
            list_branches_for_checkout(deployment_id, config, output_format).await?;
        }
    } else {
        // Show help for checkout command
        if output_format == OutputFormat::Table {
            println!("{} Checkout command requires deployment ID", "💡".yellow());
            println!("{} Usage: guepard checkout -x <deployment_id> -c <branch_id>", "ℹ️".blue());
            println!("{} Or: guepard checkout -x <deployment_id> -s <snapshot_id>", "ℹ️".blue());
            println!("{} Or: guepard checkout -x <deployment_id> (to list available branches)", "ℹ️".blue());
        }
    }
    Ok(())
}

async fn restore_snapshot(deployment_id: &str, snapshot_id: &str, config: &Config, output_format: OutputFormat) -> Result<()> {
    // Get branch ID from compute or deployment
    let branch_id = match crate::application::services::compute::list_compute(deployment_id, config).await {
        Ok(compute) => compute.branch_id.or(Some(compute.attached_branch)).unwrap(),
        Err(_) => {
            let deployment = crate::application::services::deploy::get_deployment(deployment_id, config).await?;
            deployment.branch_id.ok_or_else(|| anyhow::anyhow!("No active branch found for deployment"))?
        }
    };

    let branch = branch::checkout_snapshot(deployment_id, &branch_id, snapshot_id, config).await?;
    
    let snapshot_comment = if let Ok(snapshots) = commit::list_all_commits(deployment_id, config).await {
        snapshots.iter()
            .find(|s| s.id == snapshot_id)
            .map(|s| s.snapshot_comment.clone())
            .unwrap_or_default()
    } else {
        String::new()
    };
    
    let checkout_row = CheckoutRow {
        id: branch.id.clone(),
        name: branch.label_name.unwrap_or_else(|| branch.id.clone()),
        status: branch.job_status.unwrap_or_default(),
        snapshot_id: branch.snapshot_id.unwrap_or_else(|| branch.branch_id.unwrap_or_else(|| branch.id.clone())),
        comment: snapshot_comment,
    };
    
    if output_format == OutputFormat::Table {
        println!("{} Snapshot restored successfully!", "✅".green());
    }
    print_row_or_json(checkout_row, output_format);
    Ok(())
}

pub async fn checkout_branch(args: &CheckoutBranchArgs, config: &Config, output_format: OutputFormat) -> Result<()> {
    let branch = branch::checkout_branch(&args.deployment_id, &args.branch_id, None, config).await?;
    
    let snapshot_id = branch.snapshot_id.clone()
        .or(branch.branch_id.clone())
        .unwrap_or_else(|| branch.id.clone());
    
    let snapshot_comment = if let Ok(snapshots) = commit::list_all_commits(&args.deployment_id, config).await {
        snapshots.iter()
            .find(|s| s.id == snapshot_id)
            .map(|s| s.snapshot_comment.clone())
            .unwrap_or_default()
    } else {
        String::new()
    };
    
    let checkout_row = CheckoutRow {
        id: branch.id.clone(),
        name: branch.label_name.unwrap_or_else(|| branch.id.clone()),
        status: branch.job_status.unwrap_or_default(),
        snapshot_id,
        comment: snapshot_comment,
    };
    
    if output_format == OutputFormat::Table {
        println!("{} Checked out branch successfully!", "✅".green());
    }
    print_row_or_json(checkout_row, output_format);
    Ok(())
}

async fn list_branches_for_checkout(deployment_id: &str, config: &Config, output_format: OutputFormat) -> Result<()> {
    let branches = branch::list_branches(deployment_id, config).await?;
    
    if branches.is_empty() {
        if output_format == OutputFormat::Json {
            print_json(&serde_json::json!([]));
        } else {
            println!("{} No branches available for checkout", "ℹ️".blue());
        }
        return Ok(());
    }
    
    let snapshots = commit::list_all_commits(deployment_id, config).await.unwrap_or_default();
    
    let rows: Vec<CheckoutRow> = branches.into_iter().map(|b| {
        let id = b.id.clone();
        let snapshot_id = b.snapshot_id.clone();
        let snapshot_comment = snapshots.iter()
            .find(|s| s.id == snapshot_id)
            .map(|s| s.snapshot_comment.clone())
            .unwrap_or_default();
        
        CheckoutRow {
            id,
            name: b.branch_name.as_ref().map(|s| s.clone()).unwrap_or_else(|| b.id.clone()),
            status: b.job_status.as_ref().map(|s| s.clone()).unwrap_or_default(),
            snapshot_id,
            comment: snapshot_comment,
        }
    }).collect();
    
    if output_format == OutputFormat::Table {
        println!("{} Use 'guepard checkout -x {} -c <branch_id>' to checkout a branch", "💡".yellow(), deployment_id);
    }
    print_table_or_json(rows, output_format);
    Ok(())
}