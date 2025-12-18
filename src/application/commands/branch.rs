use anyhow::{Result, bail};
use crate::config::config::Config;
use crate::structure::{BranchArgs, CreateBranchArgs, CheckoutBranchArgs};
use crate::application::dto::branch::BranchRequest;
use crate::application::services::{branch, deploy};
use colored::Colorize;
use tabled::{Table, Tabled, settings::Style};

#[derive(Tabled, Serialize)]
struct BranchRow {
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
    #[tabled(rename = "Environment")]
    #[serde(rename = "environment_type")]
    environment_type: String,
    #[tabled(rename = "Ephemeral")]
    #[serde(rename = "is_ephemeral")]
    is_ephemeral: String,
}

use crate::application::output::{OutputFormat, print_row_or_json, print_table_or_json, print_json};
use serde::Serialize;

pub async fn branch(args: &BranchArgs, config: &Config, output_format: OutputFormat) -> Result<()> {
    if let Some(deployment_id) = &args.deployment_id {
        // Check if deployment is an F2 type
        let deployment = deploy::get_deployment(deployment_id, config).await?;
        if deployment.deployment_type == "F2" {
            bail!("{} Branch operations are not supported for F2 deployments. Use a REPOSITORY deployment instead.", "❌".red());
        }
        
        if let Some(name) = &args.name {
            // Create branch
            let source_branch_id = args.source_branch_id.clone()
                .unwrap_or_else(|| "a7d373a3-4244-47b7-aacb-ad366f2520f6".to_string()); // Default to main branch
            
            let create_args = CreateBranchArgs {
                deployment_id: deployment_id.clone(),
                snapshot_id: args.snapshot_id.clone().unwrap(),
                branch_name: name.clone(),
                source_branch_id,
                discard_changes: args.discard_changes.clone().unwrap_or("false".to_string()),
                checkout: args.checkout,
                ephemeral: args.ephemeral,
            };
            create(&create_args, config, output_format).await?;
        } else {
            // List branches
            list(deployment_id, config, output_format).await?;
        }
    } else if let Some(name) = &args.name {
        // Git-like branch creation (simplified)
        println!("{} Creating branch '{}'", "ℹ️".blue(), name);
        println!("{} Use 'guepard branch -x <deployment_id> -s <snapshot_id> -n {}' for full functionality", "💡".yellow(), name);
    } else {
        // Git-like branch listing (simplified)
        println!("{} Use 'guepard branch -x <deployment_id>' to list branches", "💡".yellow());
    }
    Ok(())
}

pub async fn create(args: &CreateBranchArgs, config: &Config, output_format: OutputFormat) -> Result<()> {
    let request = BranchRequest {
        branch_name: Some(args.branch_name.clone()),
        discard_changes: Some(args.discard_changes.clone()),
        checkout: args.checkout,
        ephemeral: args.ephemeral,
    };
    
    let branch = branch::create_branch(
        &args.deployment_id,
        &args.source_branch_id,
        &args.snapshot_id,
        request,
        config,
    ).await?;
    
    let branch_row = BranchRow {
        id: branch.id.clone(),
        name: branch.label_name.unwrap_or_else(|| branch.id.clone()),
        status: branch.job_status.unwrap_or_default(),
        snapshot_id: branch.snapshot_id.unwrap_or_else(|| branch.branch_id.unwrap_or_else(|| branch.id.clone())),
        environment_type: "development".to_string(),
        is_ephemeral: if branch.is_ephemeral.unwrap_or(false) { "Yes".to_string() } else { "No".to_string() },
    };
    
    if output_format == OutputFormat::Table {
        println!("{} Branch created successfully!", "✅".green());
    }
    print_row_or_json(branch_row, output_format);
    Ok(())
}

pub async fn list(deployment_id: &str, config: &Config, output_format: OutputFormat) -> Result<()> {
    let branches = branch::list_branches(deployment_id, config).await?;
    
    if branches.is_empty() {
        if output_format == OutputFormat::Json {
            print_json(&serde_json::json!([]));
        } else {
            println!("{} No branches found for deployment: {}", "ℹ️".blue(), deployment_id);
        }
        return Ok(());
    }
    
    let rows: Vec<BranchRow> = branches.into_iter().map(|b| {
        let id = b.id.clone();
        BranchRow {
            id,
            name: b.branch_name.as_ref().map(|s| s.clone()).unwrap_or_else(|| b.id.clone()),
            status: b.job_status.as_ref().map(|s| s.clone()).unwrap_or_default(),
            snapshot_id: b.snapshot_id,
            environment_type: "development".to_string(),
            is_ephemeral: if b.is_ephemeral { "Yes".to_string() } else { "No".to_string() },
        }
    }).collect();
    
    if output_format == OutputFormat::Table {
        println!("{} Found {} branches for deployment: {}", "✅".green(), rows.len(), deployment_id);
    }
    print_table_or_json(rows, output_format);
    Ok(())
}

pub async fn checkout(args: &CheckoutBranchArgs, config: &Config) -> Result<()> {
    // Check if deployment is an F2 type
    let deployment = deploy::get_deployment(&args.deployment_id, config).await?;
    if deployment.deployment_type == "F2" {
        bail!("{} Branch checkout is not supported for F2 deployments. Use a REPOSITORY deployment instead.", "❌".red());
    }
    
    let branch = branch::checkout_branch(&args.deployment_id, &args.branch_id, config).await?;
    
    let branch_row = BranchRow {
        id: branch.id.clone(),
        name: branch.label_name.unwrap_or_else(|| branch.id.clone()),
        status: branch.job_status.unwrap_or_default(),
        snapshot_id: branch.snapshot_id.unwrap_or_else(|| branch.branch_id.unwrap_or_else(|| branch.id.clone())),
        environment_type: "development".to_string(),
        is_ephemeral: if branch.is_ephemeral.unwrap_or(false) { "Yes".to_string() } else { "No".to_string() },
    };
    
    println!("{} Checked out branch successfully!", "✅".green());
    println!("{}", Table::new(vec![branch_row]).with(Style::rounded()));
    Ok(())
}