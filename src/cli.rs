use clap::Parser;
use guepard_cli::application::commands::{deploy, commit, branch, log, checkout, compute, usage, login, logout, list, config};
use guepard_cli::application::output::OutputFormat;
use guepard_cli::config::config::{load_config, Config};
use guepard_cli::domain::errors::{bookmark_error::BookmarkError, branch_error::BranchError, compute_error::ComputeError, deploy_error::DeployError, login_error::LoginError, usage_error::UsageError};
use guepard_cli::structure::{SubCommand, CLI};

#[tokio::main]
async fn main() {
    let args = CLI::parse();
    let sub_commands: &SubCommand = &args.sub_commands;

    let config = match load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("❌ Configuration Error: {}", e);
            std::process::exit(1);
        }
    };

    let mut exit_code = 0;
    if let Err(err) = run(sub_commands, &config).await {
        if let Some(login_error) = err.downcast_ref::<LoginError>() {
            eprintln!("❌ {}", login_error);
            exit_code = 7;
        } else if let Some(deploy_error) = err.downcast_ref::<DeployError>() {
            eprintln!("❌ Deployment Error: {}", deploy_error);
            exit_code = 2;
        } else if let Some(branch_error) = err.downcast_ref::<BranchError>() {
            eprintln!("❌ Branch Error: {}", branch_error);
            exit_code = 3;
        } else if let Some(bookmark_error) = err.downcast_ref::<BookmarkError>() {
            eprintln!("❌ Bookmark Error: {}", bookmark_error);
            exit_code = 4;
        } else if let Some(compute_error) = err.downcast_ref::<ComputeError>() {
            eprintln!("❌ Compute Error: {}", compute_error);
            exit_code = 5;
        } else if let Some(usage_error) = err.downcast_ref::<UsageError>() {
            eprintln!("❌ {}", usage_error);
            exit_code = 6;
        } else {
            eprintln!("{}", err);
            exit_code = 1;
        }
    }
    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}

async fn run(sub_commands: &SubCommand, config: &Config) -> anyhow::Result<()> {
        match sub_commands {
            SubCommand::Deploy(args) => {
                let output_format = if args.output.json { OutputFormat::Json } else { OutputFormat::Table };
                deploy::deploy(args, config, output_format).await
            }
        SubCommand::Commit(args) => {
            let output_format = if args.output.json { OutputFormat::Json } else { OutputFormat::Table };
            commit::commit(args, config, output_format).await
        }
        SubCommand::Branch(args) => {
            let output_format = if args.output.json { OutputFormat::Json } else { OutputFormat::Table };
            branch::branch(args, config, output_format).await
        }
        SubCommand::Log(args) => {
            let output_format = if args.output.json { OutputFormat::Json } else { OutputFormat::Table };
            log::log(args, config, output_format).await
        }
        SubCommand::Checkout(args) => {
            let output_format = if args.output.json { OutputFormat::Json } else { OutputFormat::Table };
            checkout::checkout(args, config, output_format).await
        }
                SubCommand::Compute(args) => {
            let output_format = if args.output.json { OutputFormat::Json } else { OutputFormat::Table };
            compute::compute(args, config, output_format).await
        }
        SubCommand::Usage => {
            // Usage doesn't have args, use default table format
            usage::usage(config, OutputFormat::Table).await
        }
        SubCommand::List(args) => {
            let output_format = if args.output.json { OutputFormat::Json } else { OutputFormat::Table };
            list::list(args, config, output_format).await
        }
        SubCommand::Login(args) => {
            let output_format = if args.output.json { OutputFormat::Json } else { OutputFormat::Table };
            login::execute(args, config, output_format).await
        }
        SubCommand::Logout => {
            // Logout doesn't have args, use default table format
            logout::logout(config, OutputFormat::Table).await
        }
        SubCommand::Config(args) => {
            let output_format = if args.output.json { OutputFormat::Json } else { OutputFormat::Table };
            config::config(args, output_format).await.map_err(|e| anyhow::anyhow!("{}", e))
        }
    }
}