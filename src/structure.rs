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

    /// Branch management commmands
    #[command(arg_required_else_help = true)]
    #[command(subcommand)]
    Branch(BranchCommand),

    /// Bookmark management commands
    #[command(arg_required_else_help = true)]
    #[command(subcommand)]
    Bookmark(BookmarkCommand),

    /// Deployment management commands
    #[command(arg_required_else_help = true)]
    #[command(subcommand)]
    Deployment(DeploymentCommand),

    /// Compute management commands
    #[command(arg_required_else_help = true)]
    #[command(subcommand)]
    Compute(ComputeCommand),
}

/// Volume commands
#[derive(Subcommand, Debug)]
pub enum VolumeCommand {
    ///List Volume command
    List,
}

/// Branch commands
#[derive(Subcommand, Debug)]
pub enum BranchCommand {
    /// List Branch command
    List {
        #[arg(short, long, name = "DEPLOYMENT ID")]
        deployment_id: String,
    },

    /// Create Branch command
    Create {
        #[arg(short, long, name = "DEPLOYMENT ID")]
        deployment_id: String,

        #[arg(short, long, name = "CLONE ID")]
        clone_id: String,

        #[arg(short, long, name = "SNAPSHOT ID")]
        snapshot_id: String,
    },

    /// Checkout Branch command
    Checkout {
        #[arg(short, long, name = "DEPLOYMENT ID")]
        deployment_id: String,

        #[arg(short, long, name = "CLONE ID")]
        clone_id: String,
    },
}

/// Bookmark commands
#[derive(Subcommand, Debug)]
pub enum BookmarkCommand {
    /// List Bookmark command
    List {
        #[arg(short, long, name = "DEPLOYMENT ID")]
        deployment_id: String,

        #[arg(short, long, name = "CLONE ID")]
        clone_id: String,
    },

    /// Create Bookmark command
    Create {
        #[arg(short, long, name = "BOOKMARK MESSAGE")]
        message: String,

        #[arg(short, long, name = "DEPLOYMENT ID")]
        deployment_id: String,

        #[arg(short, long, name = "CLONE ID")]
        clone_id: String,
    },

    /// Checkout Bookmark command
    Checkout {
        #[arg(short, long, name = "DEPLOYMENT ID")]
        deployment_id: String,

        #[arg(short, long, name = "CLONE ID")]
        clone_id: String,

        #[arg(short, long, name = "SNAPSHOT ID")]
        snapshot_id: String,
    },
}

/// Deployment commands
#[derive(Subcommand, Debug)]
pub enum DeploymentCommand {
    /// List Deployment command
    List,

    /// Get Deployment command
    Get {
        #[arg(short, long, name = "DEPLOYMENT ID")]
        deployment_id: String,
    },
}

// Compute commands
#[derive(Subcommand, Debug)]
pub enum ComputeCommand {
    /// Get Compute Info command
    Get {
        #[arg(short, long, name = "DEPLOYMENT ID")]
        deployment_id: String,

        #[arg(short, long, name = "CLONE ID")]
        clone_id: String,
    },

    /// Get Compute Status command
    Status {
        #[arg(short, long, name = "DEPLOYMENT ID")]
        deployment_id: String,

        #[arg(short, long, name = "CLONE ID")]
        clone_id: String,
    },

    /// Start Compute command
    Start {
        #[arg(short, long, name = "DEPLOYMENT ID")]
        deployment_id: String,

        #[arg(short, long, name = "CLONE ID")]
        clone_id: String,
    },

    /// Stop Compute command
    Stop {
        #[arg(short, long, name = "DEPLOYMENT ID")]
        deployment_id: String,

        #[arg(short, long, name = "CLONE ID")]
        clone_id: String,
    },
}
