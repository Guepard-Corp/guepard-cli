use crate::application::services::compute_service;
use crate::config::config::Config;
use crate::domain::errors::compute_error::ComputeError;
use crate::structure::GetComputeArgs;
use anyhow::Result;

pub async fn list(args: &GetComputeArgs, config: &Config) -> Result<()> {
    let compute = compute_service::list_compute(&args.deployment_id, &args.compute_id, config).await?;
    println!(
        "✅ Compute Details:\n\
         Deployment ID: {}\nRepository Name: {}\nCompute ID: {}\nSnapshot ID: {}\nCompute Name: {}\nFQDN: {}\n\
         Connection String: {}\nDatabase Provider: {}\nDatabase Version: {}\nRegion: {}\n\
         Instance Type: {}\nEphemeral: {}\nAttached Branch: {}",
        compute.id, compute.repository_name, compute.clone_id, compute.snapshot_id, compute.name,
        compute.fqdn, compute.connection_string, compute.database_provider, compute.database_version,
        compute.region, compute.instance_type, compute.is_ephemeral, compute.attached_branch
    );
    Ok(())
}

pub async fn start(args: &GetComputeArgs, config: &Config) -> Result<()> {
    compute_service::start_compute(&args.deployment_id, &args.compute_id, config).await?;
    println!("✅ Compute Started for Compute ID: {}", args.compute_id);
    Ok(())
}

pub async fn stop(args: &GetComputeArgs, config: &Config) -> Result<()> {
    compute_service::stop_compute(&args.deployment_id, &args.compute_id, config).await?;
    println!("✅ Compute Stopped for Compute ID: {}", args.compute_id);
    Ok(())
}

pub async fn logs(args: &GetComputeArgs, config: &Config) -> Result<()> {
    let logs = compute_service::get_logs(&args.deployment_id, &args.compute_id, config).await?;
    println!(
        "✅ Compute Logs for Compute ID: {}\n\
         Stdout Logs:\n{}\n\
         Stderr Logs:\n{}",
        args.compute_id, logs.stdout_logs, logs.stderr_logs
    );
    Ok(())
}

pub async fn status(args: &GetComputeArgs, config: &Config) -> Result<()> {
    match compute_service::get_status(&args.deployment_id, &args.compute_id, config).await {
        Ok(()) => {
            println!("✅ Compute is Healthy with Compute ID: {}", args.compute_id);
            Ok(())
        }
        Err(err) => match err {
            ComputeError::NotHealthy(msg) => {
                println!("⚠️ Compute Status: {}", msg);
                Ok(())
            }
            other => Err(other.into()),
        },
    }
}