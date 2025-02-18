use clap::{ Parser, Subcommand};

// Function to get the version
pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// guepard-gfs is a tool to manage your target environment, just pass `-h`
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
    /// Volume management commands
    #[clap(subcommand)]
    Volume(VolumeCommand),
}

/// Volume commands
#[derive(Subcommand, Debug)]
pub enum VolumeCommand {
    ///List Volume command
    List,
}