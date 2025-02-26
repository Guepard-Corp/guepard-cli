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
use dotenvy::dotenv;
use guepard_cli::application::commands::deploy;
use guepard_cli::domain::errors::deploy_error::DeployError;
use guepard_cli::structure::{DeployCommand, SubCommand, CLI};

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load .env variables

    let args = CLI::parse();

    let sub_commands: &SubCommand = &args.sub_commands;

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
    }
}
