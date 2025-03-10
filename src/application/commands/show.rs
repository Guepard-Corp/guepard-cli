use crate::application::services::show_service;
use crate::config::config::Config;
use crate::structure::GetDeployArgs;
use anyhow::Result;
use tabled::{Table, Tabled, settings::Style};
use colored::Colorize;

#[derive(Tabled)]
struct ShowRow {
    #[tabled(rename = " ")]
    marker: String,
    #[tabled(rename = "Name")]
    name: String,
}

pub async fn show_branches(args: &GetDeployArgs, config: &Config) -> Result<()> {
    let (branches, active_branch_id) = show_service::list_branches_with_active(&args.deployment_id, config).await?;

    let rows: Vec<ShowRow> = branches.into_iter().map(|b| ShowRow {
        marker: if b.clone_id == active_branch_id { "🐆".green().to_string() } else { " ".to_string() },
        name: b.name,
    }).collect();

    println!("Branches:");
    println!("{}", Table::new(rows).with(Style::rounded()));
    Ok(())
}

pub async fn show_bookmarks(args: &GetDeployArgs, config: &Config) -> Result<()> {
    let (bookmarks, active_snapshot_id) = show_service::list_bookmarks_with_active(&args.deployment_id, config).await?;

    let rows: Vec<ShowRow> = bookmarks.into_iter().map(|b| ShowRow {
        marker: if b.id == active_snapshot_id { "🐆".green().to_string() } else { " ".to_string() },
        name: b.name,
    }).collect();

    println!("Bookmarks:");
    println!("{}", Table::new(rows).with(Style::rounded()));
    Ok(())
}