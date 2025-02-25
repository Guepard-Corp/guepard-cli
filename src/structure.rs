use clap::{Args, Parser, Subcommand};

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[derive(Parser, Debug)]
#[clap(version = get_version(), about, long_about = None)]
#[clap(propagate_version = true)]
pub struct CLI {
    #[clap(subcommand)]
    pub sub_commands: SubCommand,
}

/// sub commands
#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// Deployment-related commands
    #[clap(subcommand)]
    Deploy(DeployCommand),

    /// Volume-related commands
    #[clap(subcommand)]
    Volume(VolumeCommand),
}

/// Volume commands
#[derive(Subcommand, Debug)]
pub enum VolumeCommand {
    ///List Volume command
    List,
}
/// Deploy commands

#[derive(Subcommand, Debug)]
pub enum DeployCommand {
    /// Create a new deployment
    Create(CreateDeployArgs),

    /// Update an existing deployment
    Update(UpdateDeployArgs),
}
#[derive(Args, Debug)]
pub struct CreateDeployArgs {
    #[clap(short = 'p', long, required = true)]
    pub database_provider: String,

    #[clap(short = 'v', long, required = true)]
    pub database_version: String,

    #[clap(short = 'r', long, required = true)]
    pub region: String,

    #[clap(short = 'i', long, required = true)]
    pub instance_type: String,

    #[clap(short = 'd', long, required = true)]
    pub datacenter: String,

    #[clap(short = 'n', long, required = true)]
    pub repository_name: String,

    #[clap(short = 'w', long, required = true)]
    pub database_password: String,
}
#[derive(Args, Debug)]
pub struct UpdateDeployArgs {
    #[clap(short = 'x', long, required = true)]
    pub deployment_id: String,

    #[clap(short = 'n', long, required = true)]
    pub repository_name: String,
}