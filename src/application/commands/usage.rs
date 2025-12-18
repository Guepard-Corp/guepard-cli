use crate::application::services::usage;
use crate::application::output::{OutputFormat, print_table_or_json};
use crate::config::config::Config;
use crate::structure::UsageArgs;
use anyhow::Result;
use serde::Serialize;
use tabled::{Tabled};
use colored::Colorize;

#[derive(Tabled, Serialize)]
struct UsageRow {
    #[tabled(rename = "Resource")]
    resource: String,
    #[tabled(rename = "Quota")]
    quota: i32,
    #[tabled(rename = "Used")]
    used: i32,
}
pub async fn usage(_args: &UsageArgs, config: &Config, output_format: OutputFormat) -> Result<()> {
    let usage = usage::get_usage(config).await?;
    let rows = vec![
        UsageRow { resource: "Deployments".to_string(), quota: usage.quota_deployments, used: usage.usage_deployments },
        UsageRow { resource: "Snapshots".to_string(), quota: usage.quota_snapshots, used: usage.usage_snapshots },
        UsageRow { resource: "Clones".to_string(), quota: usage.quota_clones, used: usage.usage_clones },
    ];
    if output_format == OutputFormat::Table {
        println!("{} Usage Summary:", "✅".green());
    }
    print_table_or_json(rows, output_format);
    Ok(())
}