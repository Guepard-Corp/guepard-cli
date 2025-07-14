use crate::application::services::compute_service;
use crate::config::config::Config;
use crate::domain::errors::compute_error::ComputeError;
use crate::structure::GetComputeArgs;
use anyhow::Result;
use colored::Colorize;

pub async fn list(args: &GetComputeArgs, config: &Config) -> Result<()> {
    let compute = compute_service::list_compute(&args.deployment_id, config).await?;
    println!(
        "{} Compute Name: '{}', Attached Branch: [{}], Connection String: [{}]",
        "✅".green(),
        compute.name,
        compute.attached_branch.cyan(),
        compute.connection_string
    );
    Ok(())
}

pub async fn start(args: &GetComputeArgs, config: &Config) -> Result<()> {
    compute_service::start_compute(&args.deployment_id, config).await?;
    println!("{} Successfully initiated start request for compute in deployment [{}].", "✅".green(), args.deployment_id.cyan());
    Ok(())
}

pub async fn stop(args: &GetComputeArgs, config: &Config) -> Result<()> {
    compute_service::stop_compute(&args.deployment_id, config).await?;
    println!("{} Stopped compute for deployment [{}]", "✅".green(), args.deployment_id.cyan());
    Ok(())
}

// pub async fn logs(args: &GetComputeArgs, config: &Config) -> Result<()> {
//     let logs = compute_service::get_logs(&args.deployment_id, config).await?;
//     let stdout_truncated: Vec<&str> = logs.stdout_logs.lines().collect();
//     let stderr_truncated: Vec<&str> = logs.stderr_logs.lines().collect();
//     let stdout_display = if stdout_truncated.len() > 5 {
//         format!("{}\n...", stdout_truncated[..5].join("\n"))
//     } else {
//         logs.stdout_logs.clone()
//     };
//     let stderr_display = if stderr_truncated.len() > 5 {
//         format!("{}\n...", stderr_truncated[..5].join("\n"))
//     } else {
//         logs.stderr_logs.clone()
//     };

//     println!(
//         "{} Compute Logs for Compute ID: {}\n{} Stdout Logs:\n{}\n{} Stderr Logs:\n{}",
//         "✅".green(),
//         // args.compute_id.cyan(),
//         "📜".green(),
//         stdout_display,
//         "⚠️".yellow(),
//         stderr_display
//     );
//     Ok(())
// }

pub async fn status(args: &GetComputeArgs, config: &Config) -> Result<()> {
    match compute_service::get_status(&args.deployment_id ,config).await {
        Ok(()) => println!("{} Compute  is healthy", "✅".green()),
        Err(ComputeError::NotHealthy(msg)) => println!("{} Compute is unhealthy: {}", "⚠️".yellow(), msg),
        Err(e) => return Err(e.into()),
    }
    Ok(())
}