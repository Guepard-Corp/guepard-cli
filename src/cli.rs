/// Main entry point for the CLI application.
///
/// This function initializes the environment by loading `.env` variables,
/// parses the command line arguments, and executes the appropriate subcommand.
/// If an error occurs during execution, it handles the error and exits with
/// the appropriate exit code.
///
/// # Errors
///
/// If an error occurs during the execution of a subcommand, it will be printed
/// to the standard error output and the application will exit with a non-zero
/// exit code.
use clap::Parser;

use guepard_cli::application::commands::{branch, deploy};
use guepard_cli::domain::errors::deploy_error::DeployError;
use guepard_cli::structure::{DeployCommand, SubCommand, CLI,BranchCommand};
use guepard_cli::config::config::load_config;

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
    if let Err(err) = run(sub_commands).await {
        match err.downcast_ref::<DeployError>() {
            Some(deploy_error) => {
                eprintln!("Deployment Error: {}", deploy_error);
                exit_code = 2;
            }
            None => {
                eprintln!("{}", err);
                exit_code = 1;
            }
        }
    }
    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}
/// Executes the appropriate subcommand based on user input.
///
/// This function matches the parsed subcommands and calls the corresponding
/// handler function to perform the requested operation.
///
/// # Arguments
///
/// * `sub_commands` - A reference to the parsed subcommands.
///
/// # Returns
///
/// * `anyhow::Result<()>` - Returns an `Ok(())` if the subcommand executes successfully,
///   otherwise returns an error.
async fn run(sub_commands: &SubCommand) -> anyhow::Result<()> {
    match sub_commands {
        SubCommand::Deploy(cmd) => match cmd {
            DeployCommand::Create(args) => deploy::create(args).await,
            DeployCommand::Update(args) => deploy::update(args).await,
            DeployCommand::List => deploy::list().await,
            DeployCommand::Get(args) => deploy::get(&args.deployment_id).await,
        },
        SubCommand::Branch(cmd) => match cmd {
            BranchCommand::Create(args) => branch::create(args).await,
            BranchCommand::List(args) => branch::list(&args.deployment_id).await,
            BranchCommand::Checkout(args) => branch::checkout(args).await,
            BranchCommand::Update(args) => branch::update(args).await,
        },
    }
}
