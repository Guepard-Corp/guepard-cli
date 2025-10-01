use anyhow::Result;
use crate::config::config::Config;
use crate::structure::CommitArgs;
use crate::application::services::commit;
use crate::application::dto::commit::CreateCommitRequest;
use colored::Colorize;
use tabled::{Table, Tabled, settings::Style};

#[derive(Tabled)]
struct CommitRow {
    #[tabled(rename = "Commit ID")]
    commit_id: String,
    #[tabled(rename = "Message")]
    message: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Created")]
    created_date: String,
}

pub async fn commit(args: &CommitArgs, config: &Config) -> Result<()> {
    let request = CreateCommitRequest {
        snapshot_comment: args.message.clone(),
    };
    
    let commit = commit::create_commit(
        &args.deployment_id,
        &args.clone_id,
        request,
        config,
    ).await?;
    
    // Create a beautiful table showing the created commit
    let commit_row = CommitRow {
        commit_id: commit.id,
        message: args.message.clone(),
        status: commit.status,
        created_date: commit.created_date,
    };
    
    println!("{} Created commit successfully!", "✅".green());
    println!("{}", Table::new(vec![commit_row]).with(Style::rounded()));
    
    Ok(())
}