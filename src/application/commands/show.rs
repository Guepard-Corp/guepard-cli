use anyhow::Result;
use crate::config::config::Config;
use crate::structure::{ShowCommand, GetDeployArgs};
use crate::application::services::show;
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

pub async fn show_branches(args: &GetDeployArgs, config: &Config) -> Result<()> {
    let (branches, active_branch_id) = show::list_branches_with_active(&args.deployment_id, config).await?;
    
    if branches.is_empty() {
        println!("{} No branches found for deployment: {}", "ℹ️".blue(), args.deployment_id);
        return Ok(());
    }
    
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

pub async fn show_commits(args: &GetDeployArgs, config: &Config) -> Result<()> {
    let (commits, active_snapshot_id) = show::list_commits_with_active(&args.deployment_id, config).await?;
    
    if commits.is_empty() {
        println!("{} No commits found for deployment: {}", "ℹ️".blue(), args.deployment_id);
        return Ok(());
    }
    
    let rows: Vec<CommitShowRow> = commits.into_iter().map(|c| CommitShowRow {
        marker: if c.id == active_snapshot_id { 
            format!("{: <10}", "📌") 
        } else { 
            format!("{: <10}", " ") 
        },
        name: c.name,
        id: c.id,
        message: c.snapshot_comment,
        created_date: c.created_date,
    }).collect();
    
    println!("{} Commits for deployment: {}", "📝".blue(), args.deployment_id);
    println!("{}", Table::new(rows).with(Style::rounded()));
    Ok(())
}