use clap::Parser;
use dotenvy::dotenv;
use guepard_cli::structure::{SubCommand, CLI, VolumeCommand};
use guepard_cli::application::commands::{volume};
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
                println!("Listing volumes...");
                let _ = volume::list().await;
                Ok(())
            }
        }
    }
}