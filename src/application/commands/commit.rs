use anyhow::Result;
use crate::config::config::Config;
use crate::structure::CommitArgs;
use crate::application::services::commit;
use crate::application::dto::commit::CreateCommitRequest;
use crate::application::output::{OutputFormat, print_row_or_json};
use colored::Colorize;
use serde::Serialize;
use tabled::{Tabled};

#[derive(Tabled, Serialize)]
struct CommitRow {
    #[tabled(rename = "Commit ID")]
    #[serde(rename = "commit_id")]
    commit_id: String,
    #[tabled(rename = "Message")]
    message: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Created")]
    #[serde(rename = "created_date")]
    created_date: String,
}

pub async fn commit(args: &CommitArgs, config: &Config, output_format: OutputFormat) -> Result<()> {
    let request = CreateCommitRequest {
        snapshot_comment: args.message.clone(),
    };
    
    let commit = commit::create_commit(
        &args.deployment_id,
        &args.branch_id,
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
    
    if output_format == OutputFormat::Table {
        println!("{} Created commit successfully!", "✅".green());
    }
    print_row_or_json(commit_row, output_format);
    
    Ok(())
}