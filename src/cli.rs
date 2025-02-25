use clap::Parser;
use dotenvy::dotenv;
use guepard_cli::application::commands::deploy;
use guepard_cli::application::services::deploy_service;
use guepard_cli::domain::errors::deploy_error::DeployError;
use guepard_cli::structure::{CLI, DeployCommand, SubCommand};


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

async fn run(sub_commands: &SubCommand) -> anyhow::Result<()> {
    match sub_commands {
        SubCommand::Deploy(cmd) => match cmd {
            DeployCommand::Create(args) => deploy::create(args).await,
            DeployCommand::Update(args) => deploy::update(args).await,
        },
        SubCommand::Volume(_) => {
            // Handle the Volume command here
            unimplemented!();
        },
    }
}