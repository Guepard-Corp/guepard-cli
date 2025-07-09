use crate::application::dto::bookmark_dto::{CreateBookmarkRequest}; 
use crate::config::config::Config;
use crate::structure::{CreateBookmarkArgs, CheckoutBookmarkArgs};
use anyhow::Result;
use crate::application::dto::branch_dto::BranchRequest;
use crate::application::services::bookmark_service;
use tabled::{Table, Tabled, settings::Style};
use colored::Colorize;
#[derive(Tabled)]
struct BookmarkRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Clone ID")]
    clone_id: String,
    #[tabled(rename = "Comment")]
    comment: String,
}

pub async fn list_all(deployment_id: &str, config: &Config) -> Result<()> {
    let bookmarks = bookmark_service::list_all_bookmarks(deployment_id, config).await?;
    if bookmarks.is_empty() {
        println!("{} No bookmarks found for deployment ID: {}", "ℹ️".blue(), deployment_id);
        return Ok(());
    }

    let rows: Vec<BookmarkRow> = bookmarks.into_iter().map(|b| BookmarkRow {
        id: b.id,
        name: b.name,
        status: b.status,
        clone_id: b.clone_id,
        comment: if b.snapshot_comment.len() > 30 { format!("{}...", &b.snapshot_comment[..27]) } else { b.snapshot_comment },
    }).collect();

    println!("{} Retrieved {} bookmarks:", "✅".green(), rows.len());
    println!("{}", Table::new(rows).with(Style::rounded()));
    Ok(())
}

pub async fn list(deployment_id: &str, clone_id: &str, config: &Config) -> Result<()> {
    let bookmarks = bookmark_service::list_bookmark(deployment_id, clone_id, config).await?;
    if bookmarks.is_empty() {
        println!("{} No bookmarks found for clone ID: {} in deployment ID: {}", "ℹ️".blue(), clone_id, deployment_id);        return Ok(());
    }
    let rows: Vec<BookmarkRow> = bookmarks.into_iter().map(|b| BookmarkRow {
        id: b.id,
        name: b.name,
        status: b.status,
        clone_id: b.clone_id,
        comment: if b.snapshot_comment.len() > 30 { format!("{}...", &b.snapshot_comment[..27]) } else { b.snapshot_comment },
    }).collect();

    println!("{} Retrieved {} bookmarks:", "✅".green(), rows.len());
    println!("{}", Table::new(rows).with(Style::rounded()));
    Ok(())
}

pub async fn create(args: &CreateBookmarkArgs, config: &Config) -> Result<()> {
    let request = CreateBookmarkRequest {
        snapshot_comment: args.snapshot_comment.clone(),
    };
    let bookmark = bookmark_service::create_bookmark(&args.deployment_id, &args.branch_id, request, config).await?;
    println!(
        "{} Created bookmark [{}] '{}' ({}, type: {}) for deployment [{}], branch [{}]: {}",
        "✅".green(),
        bookmark.id.cyan(),
        bookmark.name,
        bookmark.status,
        bookmark.snapshot_type,
        bookmark.deployment_id,
        bookmark.dataset_id,
        bookmark.snapshot_comment
    );
    Ok(())
}

pub async fn checkout(args: &CheckoutBookmarkArgs, config: &Config) -> Result<()> {
    let request = BranchRequest {
        discard_changes: Some(args.discard_changes.clone()),
        checkout: args.checkout,
        ephemeral: args.ephemeral,
    };
    let bookmark = bookmark_service::checkout_bookmark(
        &args.deployment_id, &args.clone_id, &args.snapshot_id, request, config
    ).await?;
    println!(
        "{} Checked out bookmark [{}] '{}' ({}) on snapshot [{}] with [{}], user: [{}]",
        "✅".green(),
        bookmark.id.cyan(),
        bookmark.name,
        bookmark.status,
        bookmark.snapshot_id,
        bookmark.database_provider,
        bookmark.database_username
    );
    Ok(())
}