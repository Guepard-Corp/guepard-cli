use clap::Parser;
use guepard_cli::application::commands::{bookmark, branch, deploy, compute, usage, show, link,login,logout};
use guepard_cli::config::config::{load_config, Config};
use guepard_cli::domain::errors::{bookmark_error::BookmarkError, branch_error::BranchError, compute_error::ComputeError, deploy_error::DeployError, link_error::LinkError, usage_error::UsageError};
use guepard_cli::structure::{BookmarkCommand, DeployCommand, SubCommand, CLI, BranchCommand, ComputeCommand, ShowCommand};

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
        }else {
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
            BookmarkCommand::List(args) => bookmark::list(&args.deployment_id, &args.branch_id, config).await,
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