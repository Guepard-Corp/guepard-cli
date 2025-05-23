use clap::Parser;
use guepard_cli::application::commands::{bookmark, branch, deploy, compute, usage, show, link, login, logout};
use guepard_cli::config::config::{load_config, Config};
use guepard_cli::domain::errors::{bookmark_error::BookmarkError, branch_error::BranchError, compute_error::ComputeError, deploy_error::DeployError, link_error::LinkError, usage_error::UsageError};
use guepard_cli::structure::{BookmarkCommand, DeployCommand, SubCommand, CLI, BranchCommand, ComputeCommand, ShowCommand};

use std::env;
use dotenvy;

// Consider initializing your logger (e.g., env_logger::init();) here if its configuration
// doesn't depend on environment variables loaded below. Otherwise, initialize after loading.

#[tokio::main]
async fn main() {
    // --- Load Environment Variables ---
    let guepard_env_path = "/etc/guepard/environment";
    match dotenvy::from_path(guepard_env_path) {
        Ok(_) => {
            // Replace with your logger: log::info!("Successfully loaded environment from {}", guepard_env_path);
            println!("INFO: Successfully loaded environment from {}", guepard_env_path);
        }
        Err(_) => {
            // Replace with your logger: log::warn!("Could not load environment from {}. Attempting to load from local .env file.", guepard_env_path);
            println!("WARN: Could not load environment from {}. Attempting to load from local .env file.", guepard_env_path);
            match dotenvy::dotenv() { // Try loading .env from current dir or parent dirs
                Ok(_) => {
                    // Replace with your logger: log::info!("Successfully loaded environment from .env file.");
                    println!("INFO: Successfully loaded environment from .env file.");
                }
                Err(_) => {
                    // Replace with your logger: log::info!("No .env file found or failed to load. Proceeding with preset environment variables if any.");
                    println!("INFO: No .env file found or failed to load. Proceeding with preset environment variables if any.");
                }
            }
        }
    }

    // --- Initialize Logger ---
    // If you're using env_logger or a similar system that reads RUST_LOG, initialize it here.
    // For example:
    // if env::var("RUST_LOG").is_err() {
    //    env::set_var("RUST_LOG", "info"); // Default log level if not set
    // }
    // env_logger::init();

    // --- Check for PUBLIC_API ---
    match env::var("PUBLIC_API") {
        Ok(api_url) => {
            // Replace with your logger: log::info!("Using PUBLIC_API: {}", api_url);
            println!("INFO: Using PUBLIC_API: {}", api_url);
        }
        Err(_) => {
            // Replace with your logger: log::error!("CRITICAL: PUBLIC_API environment variable is not set.");
            eprintln!("❌ CRITICAL: PUBLIC_API environment variable is not set. This variable must be defined in {} or a local .env file, or set in the environment.", guepard_env_path);
            std::process::exit(1); // Exit if PUBLIC_API is critical and not found
        }
    }
    
    let args = CLI::parse();
    let sub_commands: &SubCommand = &args.sub_commands;

    // The `load_config()` function might internally use `std::env::var()` to get `PUBLIC_API`
    // or other settings if needed.
    let config = match load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            // Replace with your logger: log::error!("Configuration Error: {}", e);
            eprintln!("❌ Configuration Error: {}", e);
            std::process::exit(1);
        }
    };

    let mut exit_code = 0;
    // The various command handlers (deploy::create, branch::list, etc.)
    // should use std::env::var("PUBLIC_API") when they need the API endpoint.
    if let Err(err) = run(sub_commands, &config).await {
        // It's good practice to log the full error with context if possible, e.g., log::error!("{:?}", err);
        if let Some(link_error) = err.downcast_ref::<LinkError>() {
            eprintln!("❌ Link Error: {}", link_error);
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
            eprintln!("❌ Usage Error: {}", usage_error);
            exit_code = 6;
        } else {
            eprintln!("❌ An unexpected error occurred: {}", err); // Ensure all errors are descriptive
            exit_code = 1;
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
        },
        SubCommand::Bookmark(cmd) => match cmd {
            BookmarkCommand::ListAll(args) => bookmark::list_all(&args.deployment_id, config).await,
            BookmarkCommand::List(args) => bookmark::list(&args.deployment_id, &args.clone_id, config).await,
            BookmarkCommand::Create(args) => bookmark::create(args, config).await,
            BookmarkCommand::Checkout(args) => bookmark::checkout(args, config).await,
        },
        SubCommand::Compute(cmd) => match cmd {
            ComputeCommand::List(args) => compute::list(args, config).await,
            ComputeCommand::Start(args) => compute::start(args, config).await,
            ComputeCommand::Stop(args) => compute::stop(args, config).await,
            ComputeCommand::Logs(args) => compute::logs(args, config).await,
            ComputeCommand::Status(args) => compute::status(args, config).await,
        },
        SubCommand::Usage => usage::usage(config).await,
        SubCommand::Show(cmd) => match cmd {
            ShowCommand::Branches(args) => show::show_branches(args, config).await,
            ShowCommand::Bookmarks(args) => show::show_bookmarks(args, config).await,
        },
        SubCommand::Link => link::execute(config).await.map_err(Into::into),    
        SubCommand::Login(args) => login::execute(config, &args.code).await,
        SubCommand::Logout => logout::logout(config).await,
    }
}