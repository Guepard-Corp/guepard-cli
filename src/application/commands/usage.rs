use crate::application::services::usage_service;
use crate::config::config::Config;
use anyhow::Result;

pub async fn usage(config: &Config) -> Result<()> {
    let usage = usage_service::get_usage(config).await?;
    println!(
        "✅ Usage Details:\n\
         Deployment Quota: {}\nSnapshot Quota: {}\nClone Quota: {}\n\
         Deployments Used: {}\nSnapshots Used: {}\nClones Used: {}",
        usage.quota_deployments, usage.quota_snapshots, usage.quota_clones,
        usage.usage_deployments, usage.usage_snapshots, usage.usage_clones
    );
    Ok(())
}