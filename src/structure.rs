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


#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// Deployment-related commands
    #[clap(subcommand)]
    Deploy(DeployCommand),
    #[clap(subcommand)]
    Branch(BranchCommand),
    #[clap(subcommand)] 
    Bookmark(BookmarkCommand),
    #[clap(subcommand)]
    Compute(ComputeCommand),
    Usage,
    #[clap(subcommand)] 
    Show(ShowCommand),
}
#[derive(Subcommand, Debug)]
pub enum ShowCommand {
    Branches(GetDeployArgs),
    Bookmarks(GetDeployArgs),
}

#[derive(Args, Debug)]
pub struct GetComputeArgs {
    #[clap(short = 'x', long, required = true)]
    pub deployment_id: String,
    #[clap(short = 'c', long, required = true)]
    pub compute_id: String,
}
#[derive(Subcommand, Debug)]
pub enum ComputeCommand {
    List(GetComputeArgs),
    Start(GetComputeArgs),
    Stop(GetComputeArgs),
    Logs(GetComputeArgs),
    Status(GetComputeArgs),
}

#[derive(Subcommand, Debug)] 
pub enum BookmarkCommand {
    ListAll(GetDeployArgs),
    List(GetBookmarkArgs),
    Checkout(CheckoutBookmarkArgs),
    Create(CreateBookmarkArgs),
}
#[derive(Subcommand, Debug)]
pub enum BranchCommand {
    Create(CreateBranchArgs),
    List(GetDeployArgs), 
    Checkout(CheckoutBranchArgs),
}

#[derive(Subcommand, Debug)]
pub enum DeployCommand {
    /// Create a new deployment
    Create(CreateDeployArgs),
    /// Update an existing deployment
    Update(UpdateDeployArgs),
    /// List all deployments
    List,
    /// Fetch details of a specific deployment
    Get(GetDeployArgs),
}
/// Arguments for fetching a specific deployment
#[derive(Args, Debug)]
pub struct GetDeployArgs {
    /// The ID of the deployment to fetch
    #[clap(short = 'x', long, required = true)]
    pub deployment_id: String,
}

#[derive(Args, Debug)]
pub struct CreateDeployArgs {
    /// The database provider (e.g., PostgreSQL, MySQL)
    #[clap(short = 'p', long, required = true)]
    pub database_provider: String,

    /// The version of the database
    #[clap(short = 'v', long, required = true)]
    pub database_version: String,

    /// The region where the deployment will be created
    #[clap(short = 'r', long, required = true)]
    pub region: String,

    /// The instance type for the deployment
    #[clap(short = 'i', long, required = true)]
    pub instance_type: String,

    /// The datacenter for the deployment
    #[clap(short = 'd', long, required = true)]
    pub datacenter: String,

    /// The name of the repository
    #[clap(short = 'n', long, required = true)]
    pub repository_name: String,

    /// The password for the database
    #[clap(short = 'w', long, required = true)]
    pub database_password: String,
}

/// Arguments for updating an existing deployment
#[derive(Args, Debug)]
pub struct UpdateDeployArgs {
    /// The ID of the deployment to update
    #[clap(short = 'x', long, required = true)]
    pub deployment_id: String,

    /// The name of the repository
    #[clap(short = 'n', long, required = true)]
    pub repository_name: String,
}

    #[derive(Args, Debug)]
pub struct CreateBranchArgs {
    #[clap(short = 'x', long, required = true)]
    pub deployment_id: String,
    #[clap(short = 'c', long, required = true)]
    pub clone_id: String,
    #[clap(short = 's', long, required = true)]
    pub snapshot_id: String,
    #[clap(short = 'd', long, required = true)]
    pub discard_changes: String,
    #[clap(short = 'k', long)]
    pub checkout: bool,
    #[clap(short = 'e', long)]
    pub ephemeral: bool,
}

#[derive(Args, Debug)]
pub struct CheckoutBranchArgs {
    #[clap(short = 'x', long, required = true)]
    pub deployment_id: String,
    #[clap(short = 'c', long, required = true)]
    pub clone_id: String,
}


#[derive(Args, Debug)]
pub struct GetBookmarkArgs {
    #[clap(short = 'x', long, required = true)]
    pub deployment_id: String,
    #[clap(short = 'c', long, required = true)]
    pub clone_id: String,
}

#[derive(Args, Debug)]
pub struct CreateBookmarkArgs {
    #[clap(short = 'x', long, required = true)]
    pub deployment_id: String,
    #[clap(short = 'c', long, required = true)]
    pub clone_id: String,
    #[clap(short = 'm', long, required = true)]
    pub snapshot_comment: String,
}



#[derive(Args, Debug)]
pub struct CheckoutBookmarkArgs {
    #[clap(short = 'x', long, required = true)]
    pub deployment_id: String,
    #[clap(short = 'c', long, required = true)]
    pub clone_id: String,
    #[clap(short = 's', long, required = true)]
    pub snapshot_id: String,
    #[clap(short = 'd', long, required = true)]
    pub discard_changes: String,
    #[clap(short = 'k', long)]
    pub checkout: bool,
    #[clap(short = 'e', long)]
    pub ephemeral: bool,
}