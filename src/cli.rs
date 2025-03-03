use clap::Parser;

use guepard_cli::application::commands::{bookmark,branch, deploy};
use guepard_cli::domain::errors::{bookmark_error::BookmarkError,branch_error::BranchError, deploy_error::DeployError};
use guepard_cli::structure::{BookmarkCommand,DeployCommand, SubCommand, CLI,BranchCommand};
use guepard_cli::config::config::load_config;
use guepard_cli::config::config::Config;

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
        match err.downcast_ref::<DeployError>() {
            Some(deploy_error) => {
                eprintln!("Deployment Error: {}", deploy_error);
                exit_code = 2;
            }
            None => match err.downcast_ref::<BranchError>() {
                Some(branch_error) => {
                    eprintln!("Branch Error: {}", branch_error);
                    exit_code = 3;
                }
                None => match err.downcast_ref::<BookmarkError>() { // UPDATE 4: Added
                    Some(bookmark_error) => {
                        eprintln!("Bookmark Error: {}", bookmark_error);
                        exit_code = 4;
                    }
                    None => {
                        eprintln!("{}", err);
                        exit_code = 1;
                    }
                },
            },
        }
    }
    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}

async fn run(sub_commands: &SubCommand, config: &Config) -> anyhow::Result<()> {
    match sub_commands {
        SubCommand::Deploy(cmd) => match cmd {
            DeployCommand::Create(args) => deploy::create(args, config).await, 
            DeployCommand::Update(args) => deploy::update(args, config).await, 
            DeployCommand::List => deploy::list(config).await,               
            DeployCommand::Get(args) => deploy::get(&args.deployment_id, config).await, 
        },
        SubCommand::Branch(cmd) => match cmd {
            BranchCommand::Create(args) => branch::create(args, config).await,
            BranchCommand::List(args) => branch::list(&args.deployment_id, config).await, 
            BranchCommand::Checkout(args) => branch::checkout(args, config).await, 
        },SubCommand::Bookmark(cmd) => match cmd { // UPDATE 5: Added
            BookmarkCommand::ListAll(args) => bookmark::list_all(&args.deployment_id, config).await,
            BookmarkCommand::List(args) => bookmark::list(&args.deployment_id, &args.clone_id, config).await,
            BookmarkCommand::Create(args) => bookmark::create(args, config).await,
            BookmarkCommand::Checkout(args) => bookmark::checkout(args, config).await,
        },
    }
}
