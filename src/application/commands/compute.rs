use anyhow::Result;
use crate::config::config::Config;
use crate::structure::{ComputeCommand, GetComputeArgs};
use crate::application::services::compute;
use colored::Colorize;
use tabled::{Table, Tabled, settings::Style};

#[derive(Tabled)]
struct ComputeRow {
    #[tabled(rename = "Deployment ID")]
    id: String,
    #[tabled(rename = "Repository")]
    repository_name: String,
    #[tabled(rename = "Snapshot ID")]
    snapshot_id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "FQDN")]
    fqdn: String,
    #[tabled(rename = "Provider")]
    database_provider: String,
    #[tabled(rename = "Version")]
    database_version: String,
    #[tabled(rename = "Ephemeral")]
    is_ephemeral: String,
}

#[derive(Tabled)]
struct StatusRow {
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Message")]
    message: String,
}

pub async fn compute(cmd: &ComputeCommand, config: &Config) -> Result<()> {
    match cmd {
        ComputeCommand::Status(args) => status(args, config).await,
        ComputeCommand::Start(args) => start(args, config).await,
        ComputeCommand::Stop(args) => stop(args, config).await,
        ComputeCommand::Restart(args) => restart(args, config).await,
        ComputeCommand::List(args) => list(args, config).await,
        ComputeCommand::Logs(args) => logs(args, config).await,
    }
}

pub async fn status(args: &GetComputeArgs, config: &Config) -> Result<()> {
    let result = compute::get_status(&args.deployment_id, config).await?;
    
    let status_row = StatusRow {
        status: result.status,
        message: result.message.unwrap_or("No additional information".to_string()),
    };
    
    println!("{} Compute Status for deployment: {}", "📊".blue(), args.deployment_id);
    println!("{}", Table::new(vec![status_row]).with(Style::rounded()));
    Ok(())
}

pub async fn start(args: &GetComputeArgs, config: &Config) -> Result<()> {
    compute::start_compute(&args.deployment_id, config).await?;
    println!("{} Compute instance started successfully!", "✅".green());
    Ok(())
}

pub async fn stop(args: &GetComputeArgs, config: &Config) -> Result<()> {
    compute::stop_compute(&args.deployment_id, config).await?;
    println!("{} Compute instance stopped successfully!", "✅".green());
    Ok(())
}

pub async fn restart(args: &GetComputeArgs, config: &Config) -> Result<()> {
    stop(args, config).await?;
    start(args, config).await?;
    println!("{} Compute instance restarted successfully!", "✅".green());
    Ok(())
}

pub async fn list(args: &GetComputeArgs, config: &Config) -> Result<()> {
    let result = compute::list_compute(&args.deployment_id, config).await?;
    
    let compute_row = ComputeRow {
        id: result.id,
        repository_name: result.repository_name,
        snapshot_id: result.snapshot_id,
        name: result.name,
        fqdn: result.fqdn,
        database_provider: result.database_provider,
        database_version: result.database_version,
        is_ephemeral: if result.is_ephemeral { "Yes".to_string() } else { "No".to_string() },
    };
    
    println!("{} Compute instance details for deployment: {}", "🖥️".blue(), args.deployment_id);
    println!("{}", Table::new(vec![compute_row]).with(Style::rounded()));
    Ok(())
}

pub async fn logs(args: &GetComputeArgs, config: &Config) -> Result<()> {
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