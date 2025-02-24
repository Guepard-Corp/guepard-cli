use clap::Parser;
use dotenvy::dotenv;
use guepard_cli::structure::{SubCommand, CLI, VolumeCommand, BranchCommand, BookmarkCommand, DeploymentCommand, ComputeCommand};
use guepard_cli::application::commands::{bookmark, branch, compute, deployment, volume};

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    let args = CLI::parse();

    let sub_commands: &SubCommand = &args.sub_commands;

    let mut exit_code = 0;
    if let Err(err) = run(sub_commands).await {
        eprintln!("{}", err);
        exit_code = 1;
    }
    if exit_code != 0 {
        std::process::exit(exit_code);
   }
}

async fn run(sub_commands: &SubCommand) -> anyhow::Result<()> {
    match sub_commands {
        SubCommand::Volume(cmd) => match cmd {
            VolumeCommand::List => {
                let _ = volume::list().await;
            },
        },

        SubCommand::Branch(cmd) => match cmd {
            BranchCommand::List { deployment_id } => {
                let branches = branch::list(deployment_id).await;
                println!("{branches:#?}");
            },
            BranchCommand::Create { deployment_id, clone_id, snapshot_id } => {
                let branch = branch::create(deployment_id, clone_id, snapshot_id).await;
                println!("{branch:#?}");
            },
            BranchCommand::Checkout { deployment_id, clone_id } => {
                let branch = branch::checkout(deployment_id, clone_id).await;
                println!("{branch:#?}");
            },
        },

        SubCommand::Bookmark(cmd) => match cmd {
            BookmarkCommand::List { deployment_id, clone_id } => {
                let bookmarks = bookmark::list(deployment_id, clone_id).await;
                println!("{bookmarks:#?}");
            },
            BookmarkCommand::Create { message, deployment_id, clone_id } => {
                let bookmark = bookmark::create(message, deployment_id, clone_id).await;
                println!("{bookmark:#?}");
            },
            BookmarkCommand::Checkout { deployment_id, clone_id, snapshot_id } => {
                let bookmark = bookmark::checkout(deployment_id, clone_id, snapshot_id).await;
                println!("{bookmark:#?}");
            },
        },

        SubCommand::Deployment(cmd) => match cmd {
            DeploymentCommand::List => {
                let deployments = deployment::list().await?;
                println!("{deployments:#?}");
            },
            DeploymentCommand::Get { deployment_id } => {
                let deployment = deployment::get(deployment_id).await?;
                println!("{deployment:#?}");
            },
        },

        SubCommand::Compute(cmd) => match cmd {
            ComputeCommand::Get { deployment_id, clone_id } =>  {
                let compute = compute::get(deployment_id, clone_id).await;
                println!("{compute:#?}");
            },
            ComputeCommand::Status { deployment_id, clone_id } => {
                let status = compute::status(deployment_id, clone_id).await;
                println!("{status:#?}");
            },
            ComputeCommand::Start { deployment_id, clone_id } => {
                let compute = compute::start(deployment_id, clone_id).await;
                println!("{compute:#?}");
            },
            ComputeCommand::Stop { deployment_id, clone_id } => {
                let compute = compute::stop(deployment_id, clone_id).await;
                println!("{compute:#?}");
            },
        }
    }

    Ok(())
}
