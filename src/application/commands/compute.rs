use anyhow::Result;
use crate::config::config::Config;
use crate::structure::ComputeArgs;
use crate::application::services::{compute, branch};
use crate::domain::errors::compute_error::ComputeError;
use colored::Colorize;
use tabled::{Table, Tabled, settings::Style};

#[derive(Tabled)]
struct ComputeRow {
    #[tabled(rename = "Deployment ID")]
    id: String,
    #[tabled(rename = "Branch ID")]
    branch_id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "FQDN")]
    fqdn: String,
    #[tabled(rename = "Port")]
    port: String,
    #[tabled(rename = "Attached Branch")]
    attached_branch: String,
    #[tabled(rename = "Current Snapshot")]
    current_snapshot: String,
}

#[derive(Tabled)]
struct StatusRow {
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Message")]
    message: String,
}

pub async fn compute(args: &ComputeArgs, config: &Config) -> Result<()> {
    match args.action.as_deref() {
        Some("status") => status(args, config).await,
        Some("start") => start(args, config).await,
        Some("stop") => stop(args, config).await,
        Some("restart") => restart(args, config).await,
        Some("logs") => logs(args, config).await,
        Some(action) => {
            println!("{} Unknown action: {}", "❌".red(), action);
            println!("Available actions: start, stop, status, logs");
            Ok(())
        }
        None => {
            // Default action: show compute info (like list)
            list(args, config).await
        }
    }
}

pub async fn status(args: &ComputeArgs, config: &Config) -> Result<()> {
    match compute::get_status(&args.deployment_id, config).await {
        Ok(result) => {
            let status_row = StatusRow {
                status: "Healthy".to_string(),
                message: result.message,
            };
            
            println!("{} Compute Status for deployment: {}", "📊".blue(), args.deployment_id);
            println!("{}", Table::new(vec![status_row]).with(Style::rounded()));
        }
        Err(ComputeError::NotHealthy(message)) => {
            let status_row = StatusRow {
                status: "Not Healthy".to_string(),
                message,
            };
            
            println!("{} Compute Status for deployment: {}", "📊".blue(), args.deployment_id);
            println!("{}", Table::new(vec![status_row]).with(Style::rounded()));
        }
        Err(e) => return Err(e.into()),
    }
    Ok(())
}

pub async fn start(args: &ComputeArgs, config: &Config) -> Result<()> {
    compute::start_compute(&args.deployment_id, config).await?;
    println!("{} Compute instance started successfully!", "✅".green());
    Ok(())
}

pub async fn stop(args: &ComputeArgs, config: &Config) -> Result<()> {
    compute::stop_compute(&args.deployment_id, config).await?;
    println!("{} Compute instance stopped successfully!", "✅".green());
    Ok(())
}

pub async fn restart(args: &ComputeArgs, config: &Config) -> Result<()> {
    stop(args, config).await?;
    start(args, config).await?;
    println!("{} Compute instance restarted successfully!", "✅".green());
    Ok(())
}

pub async fn list(args: &ComputeArgs, config: &Config) -> Result<()> {
    let result = compute::list_compute(&args.deployment_id, config).await?;
    
    // Get branch information to find the current snapshot
    let branches = branch::list_branches(&args.deployment_id, config).await?;
    let current_snapshot = branches
        .iter()
        .find(|b| b.id == result.attached_branch)
        .map(|b| b.snapshot_id.clone())
        .unwrap_or_else(|| "Unknown".to_string());
    
    let compute_row = ComputeRow {
        id: result.id,
        branch_id: result.branch_id,
        name: result.name,
        fqdn: result.fqdn,
        port: result.port.to_string(),
        attached_branch: result.attached_branch,
        current_snapshot,
    };
    
    println!("{} Compute instance details for deployment: {}", "🖥️".blue(), args.deployment_id);
    println!("{}", Table::new(vec![compute_row]).with(Style::rounded()));
    Ok(())
}

pub async fn logs(args: &ComputeArgs, config: &Config) -> Result<()> {
    let result = compute::get_logs(&args.deployment_id, config).await?;
    
    println!("{} Compute logs for deployment: {}", "📋".blue(), args.deployment_id);
    println!("{}", "=".repeat(80).cyan());
    
    if !result.stdout_logs.is_empty() {
        println!("{} STDOUT:", "📤".green());
        println!("{}", result.stdout_logs);
    }
    
    if !result.stderr_logs.is_empty() {
        println!("{} STDERR:", "📥".red());
        println!("{}", result.stderr_logs);
    }
    
    if result.stdout_logs.is_empty() && result.stderr_logs.is_empty() {
        println!("{} No logs available", "ℹ️".blue());
    }
    
    Ok(())
}