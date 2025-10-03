use clap::Parser;
use guepard_cli::application::commands::{init, deploy, commit, branch, log, rev_parse, checkout, compute, show, usage, login, logout, list};
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
        SubCommand::Init(args) => init::init(args, config).await,
        SubCommand::Deploy(args) => deploy::deploy(args, config).await,
        SubCommand::Commit(args) => commit::commit(args, config).await,
        SubCommand::Branch(args) => branch::branch(args, config).await,
        SubCommand::Log => log::log(config).await,
        SubCommand::RevParse => rev_parse::rev_parse(config).await,
        SubCommand::Checkout(args) => checkout::checkout(args, config).await,
        SubCommand::Compute(cmd) => compute::compute(cmd, config).await,
        SubCommand::Show(cmd) => show::show(cmd, config).await,
        SubCommand::Usage => usage::usage(config).await,
        SubCommand::List(args) => list::list(args, config).await,
        SubCommand::Login => login::execute(config).await,
        SubCommand::Logout => logout::logout(config).await,
    }
}