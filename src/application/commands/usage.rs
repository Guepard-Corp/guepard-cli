use crate::application::services::usage;
use crate::config::config::Config;
use anyhow::Result;
use tabled::{Table, Tabled, settings::Style};
use colored::Colorize;

#[derive(Tabled)]
struct UsageRow {
    #[tabled(rename = "Resource")]
    resource: String,
    #[tabled(rename = "Quota")]
    quota: i32,
    #[tabled(rename = "Used")]
    used: i32,
}
pub async fn usage(config: &Config) -> Result<()> {
    let usage = usage::get_usage(config).await?;
    let rows = vec![
        UsageRow { resource: "Deployments".to_string(), quota: usage.quota_deployments, used: usage.usage_deployments },
        UsageRow { resource: "Snapshots".to_string(), quota: usage.quota_snapshots, used: usage.usage_snapshots },
        UsageRow { resource: "Clones".to_string(), quota: usage.quota_clones, used: usage.usage_clones },
    ];
    println!("{} Usage Summary:", "✅".green());
    println!("{}", Table::new(rows).with(Style::rounded()));
    Ok(())
}