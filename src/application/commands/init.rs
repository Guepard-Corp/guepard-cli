use anyhow::Result;
use crate::config::config::Config;
use crate::structure::InitArgs;
use crate::application::services::deploy;
use colored::Colorize;
use tabled::{Table, Tabled, settings::Style};

#[derive(Tabled)]
struct InitRow {
    #[tabled(rename = "Deployment ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Repository")]
    repository_name: String,
    #[tabled(rename = "Provider")]
    database_provider: String,
    #[tabled(rename = "Version")]
    database_version: String,
    #[tabled(rename = "Status")]
    status: String,
}

pub async fn init(_args: &InitArgs, config: &Config) -> Result<()> {
    println!("{} Initializing Guepard environment...", "🚀".blue());
    
    // List existing deployments to show what's available
    let deployments = deploy::list_deployments(config).await?;
    
    if deployments.is_empty() {
        println!("{} No deployments found. Create one with:", "ℹ️".blue());
        println!("{} guepard deploy -p PostgreSQL -v 16 -r us-west-aws -d us-west-aws -n myrepo -w password", "💡".yellow());
        return Ok(());
    }
    
    let rows: Vec<InitRow> = deployments.into_iter().map(|d| InitRow {
        id: d.id,
        name: d.name,
        repository_name: d.repository_name,
        database_provider: d.database_provider,
        database_version: d.database_version,
        status: d.status,
    }).collect();
    
    println!("{} Found {} deployments:", "✅".green(), rows.len());
    println!("{}", Table::new(rows).with(Style::rounded()));
    
    println!("{} Use 'guepard branch -x <deployment_id>' to work with branches", "💡".yellow());
    println!("{} Use 'guepard checkout -x <deployment_id> -c <branch_id>' to checkout branches", "💡".yellow());
    
    Ok(())
}