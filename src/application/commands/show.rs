use anyhow::Result;
use crate::config::config::Config;
use crate::structure::{ShowCommand, ShowDeployArgs};
use crate::application::services::{branch, commit};
use colored::Colorize;
use tabled::{Table, Tabled, settings::Style};

#[derive(Tabled)]
struct BranchShowRow {
    #[tabled(rename = "   Active   ")]
    marker: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Snapshot ID")]
    snapshot_id: String,
}

#[derive(Tabled)]
struct CommitShowRow {
    #[tabled(rename = "   Active   ")]
    marker: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Message")]
    message: String,
    #[tabled(rename = "Created")]
    created_date: String,
}

pub async fn show(cmd: &ShowCommand, config: &Config) -> Result<()> {
    match cmd {
        ShowCommand::Branches(args) => show_branches(args, config).await,
        ShowCommand::Commits(args) => show_commits(args, config).await,
    }
}

pub async fn show_branches(args: &ShowDeployArgs, config: &Config) -> Result<()> {
    let branches = branch::list_branches(&args.deployment_id, config).await?;
    
    if branches.is_empty() {
        println!("{} No branches found for deployment: {}", "ℹ️".blue(), args.deployment_id);
        return Ok(());
    }
    
    // For now, assume the first branch is active
    let active_branch_id = branches.first().map(|b| b.id.clone()).unwrap_or_default();
    
    let rows: Vec<BranchShowRow> = branches.into_iter().map(|b| BranchShowRow {
        marker: if b.id == active_branch_id { 
            format!("{: <10}", "🐆") 
        } else { 
            format!("{: <10}", " ") 
        },
        name: b.branch_name.as_ref().map(|s| s.clone()).unwrap_or_else(|| b.id.clone()),
        id: b.id,
        status: b.job_status.as_ref().map(|s| s.clone()).unwrap_or_default(),
        snapshot_id: b.snapshot_id,
    }).collect();
    
    println!("{} Branches for deployment: {}", "🌿".blue(), args.deployment_id);
    println!("{}", Table::new(rows).with(Style::rounded()));
    Ok(())
}

pub async fn show_commits(args: &ShowDeployArgs, config: &Config) -> Result<()> {
    let commits = commit::list_all_commits(&args.deployment_id, config).await?;
    
    if commits.is_empty() {
        println!("{} No commits found for deployment: {}", "ℹ️".blue(), args.deployment_id);
        return Ok(());
    }
    
    // For now, assume the first commit is active
    let active_snapshot_id = commits.first().map(|c| c.id.clone()).unwrap_or_default();
    
    let rows: Vec<CommitShowRow> = commits.into_iter().map(|c| CommitShowRow {
        marker: if c.id == active_snapshot_id { 
            format!("{: <10}", "📌") 
        } else { 
            format!("{: <10}", " ") 
        },
        name: c.name,
        id: c.id,
        message: c.snapshot_comment,
        created_date: c.created_date.split('T').next().unwrap_or(&c.created_date).to_string(),
    }).collect();
    
    println!("{} Commits for deployment: {}", "📝".blue(), args.deployment_id);
    println!("{}", Table::new(rows).with(Style::rounded()));
    Ok(())
}