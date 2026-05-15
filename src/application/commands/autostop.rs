use crate::application::dto::autostop::{
    AutostopConfigureResponse, AutostopResponse, AutostopStatusResponse,
};
use crate::application::output::{print_json, print_table_or_json, OutputFormat};
use crate::application::services::autostop;
use crate::config::config::Config;
use crate::domain::errors::deploy_error::DeployError;
use crate::structure::AutostopCommand;
use anyhow::{bail, Result};
use colored::Colorize;
use serde::Serialize;
use tabled::Tabled;

pub fn autostop_output_format(command: &AutostopCommand) -> bool {
    match command {
        AutostopCommand::Enable { output, .. }
        | AutostopCommand::Disable { output, .. }
        | AutostopCommand::Status { output, .. }
        | AutostopCommand::Configure { output, .. } => output.json,
    }
}

pub fn is_valid_idle_duration(value: &str) -> bool {
    let Some(unit) = value.chars().last() else {
        return false;
    };
    if !matches!(unit, 's' | 'm' | 'h' | 'd') {
        return false;
    }
    let digits = &value[..value.len() - 1];
    !digits.is_empty() && digits.chars().all(|c| c.is_ascii_digit())
}

fn map_autostop_error(err: DeployError) -> DeployError {
    match &err {
        DeployError::BadRequest(msg) if is_compute_not_running_message(msg) => DeployError::BadRequest(
            format!(
                "{msg}\n\nStart compute first: guepard compute start -x <deployment_id>"
            ),
        ),
        _ => err,
    }
}

fn is_compute_not_running_message(msg: &str) -> bool {
    let lower = msg.to_lowercase();
    lower.contains("not running") || lower.contains("compute stopped") || lower.contains("stopped")
}

fn print_action_result(response: &AutostopResponse, output_format: OutputFormat) {
    if output_format == OutputFormat::Json {
        print_json(response);
        return;
    }

    if response.skipped == Some(true) {
        let state = match response.action.as_str() {
            "enable" => "already enabled",
            "disable" => "already disabled",
            _ => "unchanged",
        };
        println!("{} Autostop {}", "ℹ️".blue(), state);
    } else {
        println!("{} {}", "✅".green(), response.message);
    }
}

fn print_configure_result(response: &AutostopConfigureResponse, output_format: OutputFormat) {
    if output_format == OutputFormat::Json {
        print_json(response);
        return;
    }

    if response.skipped == Some(true) {
        println!(
            "{} Idle duration already set to {}",
            "ℹ️".blue(),
            response.idle_duration
        );
    } else {
        println!("{} {}", "✅".green(), response.message);
        println!("Idle duration: {}", response.idle_duration);
    }
}

#[derive(Tabled, Serialize)]
struct AutostopStatusRow {
    deployment_id: String,
    autostop: String,
    idle_duration: String,
}

pub async fn run(command: &AutostopCommand, config: &Config, output_format: OutputFormat) -> Result<()> {
    match command {
        AutostopCommand::Enable {
            deployment_id, ..
        } => {
            let response = autostop::autostop_enable(deployment_id, config)
                .await
                .map_err(map_autostop_error)?;
            print_action_result(&response, output_format);
        }
        AutostopCommand::Disable {
            deployment_id, ..
        } => {
            let response = autostop::autostop_disable(deployment_id, config)
                .await
                .map_err(map_autostop_error)?;
            print_action_result(&response, output_format);
        }
        AutostopCommand::Status {
            deployment_id, ..
        } => {
            let status = autostop::autostop_status(deployment_id, config)
                .await
                .map_err(map_autostop_error)?;
            print_status(&status, output_format);
        }
        AutostopCommand::Configure {
            deployment_id,
            idle_duration,
            duration,
            ..
        } => {
            let idle = idle_duration
                .clone()
                .or_else(|| duration.clone())
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "idle duration required (e.g. --idle-duration 45m or configure <deployment_id> 45m)"
                    )
                })?;
            if !is_valid_idle_duration(&idle) {
                bail!(
                    "invalid idle_duration {:?}: use format like 30s, 45m, 2h, 1d",
                    idle
                );
            }
            let response = autostop::configure_autostop(deployment_id, &idle, config)
                .await
                .map_err(map_autostop_error)?;
            print_configure_result(&response, output_format);
        }
    }
    Ok(())
}

fn print_status(status: &AutostopStatusResponse, output_format: OutputFormat) {
    if output_format == OutputFormat::Json {
        print_json(status);
        return;
    }

    let row = AutostopStatusRow {
        deployment_id: status.deployment_id.clone(),
        autostop: if status.autostop {
            "on".to_string()
        } else {
            "off".to_string()
        },
        idle_duration: status.idle_duration.clone(),
    };
    print_table_or_json(vec![row], output_format);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_idle_durations() {
        assert!(is_valid_idle_duration("30s"));
        assert!(is_valid_idle_duration("45m"));
        assert!(is_valid_idle_duration("2h"));
        assert!(is_valid_idle_duration("1d"));
        assert!(!is_valid_idle_duration("45"));
        assert!(!is_valid_idle_duration("m45"));
        assert!(!is_valid_idle_duration(""));
    }

    #[test]
    fn compute_not_running_detection() {
        assert!(is_compute_not_running_message("Compute is not running"));
        assert!(!is_compute_not_running_message("invalid action"));
    }
}
