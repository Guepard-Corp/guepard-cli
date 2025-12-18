use anyhow::Result;
use crate::config::config::Config;
use crate::structure::ComputeArgs;
use crate::application::services::{compute, branch};
use crate::application::output::{OutputFormat, print_row_or_json, print_json};
use crate::domain::errors::compute_error::ComputeError;
use colored::Colorize;
use serde::Serialize;
use tabled::{Tabled};

#[derive(Tabled, Serialize)]
struct ComputeRow {
    #[tabled(rename = "Deployment ID")]
    #[serde(rename = "deployment_id")]
    id: String,
    #[tabled(rename = "Branch ID")]
    #[serde(rename = "branch_id")]
    branch_id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "FQDN")]
    fqdn: String,
    #[tabled(rename = "Port")]
    port: String,
    #[tabled(rename = "Attached Branch")]
    #[serde(rename = "attached_branch")]
    attached_branch: String,
    #[tabled(rename = "Current Snapshot")]
    #[serde(rename = "current_snapshot")]
    current_snapshot: String,
}

#[derive(Tabled, Serialize)]
struct StatusRow {
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Message")]
    message: String,
}

#[derive(Serialize)]
struct StatusWithConnection {
    status: String,
    message: String,
    connection_string: Option<String>,
}

pub async fn compute(args: &ComputeArgs, config: &Config, output_format: OutputFormat) -> Result<()> {
    match args.action.as_deref() {
        Some("status") => status(args, config, output_format).await,
        Some("start") => start(args, config, output_format).await,
        Some("stop") => stop(args, config, output_format).await,
        Some("restart") => restart(args, config, output_format).await,
        Some("logs") => logs(args, config, output_format).await,
        Some(action) => {
            if output_format == OutputFormat::Table {
                println!("{} Unknown action: {}", "❌".red(), action);
                println!("Available actions: start, stop, status, logs");
            } else {
                print_json(&serde_json::json!({
                    "error": format!("Unknown action: {}", action),
                    "available_actions": ["start", "stop", "status", "logs", "list"]
                }));
            }
            Ok(())
        }
        None => {
            // Default action: show compute info (like list)
            list(args, config, output_format).await
        }
    }
}

pub async fn status(args: &ComputeArgs, config: &Config, output_format: OutputFormat) -> Result<()> {
    let compute_info = compute::list_compute(&args.deployment_id, config).await.ok();
    let connection_string = compute_info.as_ref().map(|c| c.connection_string.clone());
    
    match compute::get_status(&args.deployment_id, config).await {
        Ok(result) => {
            let message = result.message.unwrap_or_else(|| "Compute instance is running".to_string());
            
            if output_format == OutputFormat::Json {
                let status_data = StatusWithConnection {
                    status: "Healthy".to_string(),
                    message,
                    connection_string,
                };
                print_json(&status_data);
            } else {
                let status_row = StatusRow {
                    status: "Healthy".to_string(),
                    message,
                };
                
                println!("{} Compute Status for deployment: {}", "📊".blue(), args.deployment_id);
                print_row_or_json(status_row, output_format);
                
                if let Some(conn_str) = connection_string {
                    println!();
                    println!("{} Connection Information", "🔗".blue());
                    println!("  {} {}", "Connection URI:".yellow(), conn_str.cyan().bold());
                }
            }
        }
        Err(ComputeError::NotHealthy(message)) => {
            if output_format == OutputFormat::Json {
                let status_data = StatusWithConnection {
                    status: "Not Healthy".to_string(),
                    message,
                    connection_string,
                };
                print_json(&status_data);
            } else {
                let status_row = StatusRow {
                    status: "Not Healthy".to_string(),
                    message,
                };
                
                println!("{} Compute Status for deployment: {}", "📊".blue(), args.deployment_id);
                print_row_or_json(status_row, output_format);
                
                if let Some(conn_str) = connection_string {
                    println!();
                    println!("{} Connection Information", "🔗".blue());
                    println!("  {} {}", "Connection URI:".yellow(), conn_str.cyan().bold());
                }
            }
        }
        Err(e) => return Err(e.into()),
    }
    Ok(())
}

pub async fn start(args: &ComputeArgs, config: &Config, output_format: OutputFormat) -> Result<()> {
    compute::start_compute(&args.deployment_id, config).await?;
    if output_format == OutputFormat::Table {
        println!("{} Compute instance started successfully!", "✅".green());
    } else {
        print_json(&serde_json::json!({"status": "started", "deployment_id": args.deployment_id}));
    }
    Ok(())
}

pub async fn stop(args: &ComputeArgs, config: &Config, output_format: OutputFormat) -> Result<()> {
    compute::stop_compute(&args.deployment_id, config).await?;
    if output_format == OutputFormat::Table {
        println!("{} Compute instance stopped successfully!", "✅".green());
    } else {
        print_json(&serde_json::json!({"status": "stopped", "deployment_id": args.deployment_id}));
    }
    Ok(())
}

pub async fn restart(args: &ComputeArgs, config: &Config, output_format: OutputFormat) -> Result<()> {
    compute::stop_compute(&args.deployment_id, config).await?;
    compute::start_compute(&args.deployment_id, config).await?;
    if output_format == OutputFormat::Table {
        println!("{} Compute instance restarted successfully!", "✅".green());
    } else {
        print_json(&serde_json::json!({"status": "restarted", "deployment_id": args.deployment_id}));
    }
    Ok(())
}

pub async fn list(args: &ComputeArgs, config: &Config, output_format: OutputFormat) -> Result<()> {
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
        branch_id: result.branch_id.unwrap_or(result.attached_branch.clone()),
        name: result.name,
        fqdn: result.fqdn,
        port: result.port.to_string(),
        attached_branch: result.attached_branch,
        current_snapshot,
    };
    
    if output_format == OutputFormat::Table {
        println!("{} Compute instance details for deployment: {}", "🖥️".blue(), args.deployment_id);
    }
    print_row_or_json(compute_row, output_format);
    Ok(())
}

pub async fn logs(args: &ComputeArgs, config: &Config, output_format: OutputFormat) -> Result<()> {
    let result = compute::get_logs(&args.deployment_id, config).await?;
    
    if output_format == OutputFormat::Json {
        print_json(&serde_json::json!({
            "deployment_id": args.deployment_id,
            "stdout": result.stdout_logs,
            "stderr": result.stderr_logs
        }));
    } else {
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
    }
    
    Ok(())
}