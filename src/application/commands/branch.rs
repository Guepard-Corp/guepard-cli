use anyhow::Result;
use crate::config::config::Config;
use crate::structure::{BranchArgs, CreateBranchArgs, CheckoutBranchArgs};
use crate::application::dto::branch::BranchRequest;
use crate::application::services::branch;
use colored::Colorize;
use tabled::{Table, Tabled, settings::Style};

#[derive(Tabled)]
struct BranchRow {
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
    #[tabled(rename = "Ephemeral")]
    is_ephemeral: String,
}

pub async fn branch(args: &BranchArgs, config: &Config) -> Result<()> {
    if let Some(deployment_id) = &args.deployment_id {
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
            create(&create_args, config).await?;
        } else {
            // List branches
            list(deployment_id, config).await?;
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

pub async fn create(args: &CreateBranchArgs, config: &Config) -> Result<()> {
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
    
    println!("{} Branch '{}' created successfully!", "✅".green(), branch_row.name);
    println!("{}", Table::new(vec![branch_row]).with(Style::rounded()));
    Ok(())
}

pub async fn list(deployment_id: &str, config: &Config) -> Result<()> {
    let branches = branch::list_branches(deployment_id, config).await?;
    
    if branches.is_empty() {
        println!("{} No branches found for deployment: {}", "ℹ️".blue(), deployment_id);
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
    
    println!("{} Found {} branches for deployment: {}", "✅".green(), rows.len(), deployment_id);
    println!("{}", Table::new(rows).with(Style::rounded()));
    Ok(())
}

pub async fn checkout(args: &CheckoutBranchArgs, config: &Config) -> Result<()> {
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
