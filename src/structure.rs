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

    /// Branch-related commands
    #[clap(subcommand)]
    Branch(BranchCommand),

    /// Bookmark-related commands
    #[clap(subcommand)]
    Bookmark(BookmarkCommand),

    /// Compute-related commands
    #[clap(subcommand)]
    Compute(ComputeCommand),

    /// Show usage information
    Usage,

    // /// Show details about branches, bookmarks, etc.
    // #[clap(subcommand)]
    // Show(ShowCommand),

    /// Start login and get authentication URL
    Link, 

    /// Complete login with verification code
    Login(LoginArgs),
    /// Log out and clear all credentials
    Logout,
}


#[derive(Args, Debug)]
pub struct LoginArgs {
    /// The verification code from the login URL
    #[clap(value_parser, required = true)]
    pub code: String,
}



#[derive(Args, Debug)]
pub struct GetComputeArgs {
    /// The ID of the deployment
    #[clap(short = 'x', long, required = true)]
    pub deployment_id: String,
}

#[derive(Subcommand, Debug)]
pub enum ComputeCommand {
    /// List all compute instances for a deployment
    List(GetComputeArgs),

    /// Start a compute instance
    Start(GetComputeArgs),

    /// Stop a compute instance
    Stop(GetComputeArgs),


    /// Check the status of a compute instance
    Status(GetComputeArgs),
}

#[derive(Subcommand, Debug)]
pub enum BookmarkCommand {
    /// List all bookmarks for a deployment
    ListAll(GetDeployArgs),

    /// List bookmarks for a specific branch
    List(GetBookmarkArgs),

    /// Checkout a bookmark
    Checkout(CheckoutBookmarkArgs),

    /// Create a new bookmark
    Create(CreateBookmarkArgs),
}

#[derive(Subcommand, Debug)]
pub enum BranchCommand {
    /// Create a new branch
    Create(CreateBranchArgs),

    /// List all branches for a deployment
    List(GetDeployArgs),

    /// Checkout a branch
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

    /// The datacenter for the deployment
    #[clap(short = 'd', long, required = true)]
    pub datacenter: String,

    /// The instance type for the deployment
    #[clap(short = 'i', long, required = true)]
    pub instance_type: String,

    /// The type of deployment (e.g., REPOSITORY,SHADOW,F2)
    #[clap(short = 't', long, required = true)]
    pub deployment_type: String,

    /// The name of the repository
    #[clap(short = 'n', long, required = true)]
    pub repository_name: String,

    /// The username for the database
    #[clap(short = 'u', long, required = true)]
    pub database_username: String,

    /// The password for the database
    #[clap(short = 'w', long, required = true)]
    pub database_password: String,

    /// The performance profile ID (optional)
    #[clap(long)]
    pub performance_profile_id: Option<String>,

    /// The node ID ,(optional, A shared Node will be assigned to the user )
    #[clap(long)]
    pub node_id: Option<String>,
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
    /// The ID of the deployment
    #[clap(short = 'x', long, required = true)]
    pub deployment_id: String,

    /// The ID of the source branch
    #[clap(short = 'b', long, required = true)]
    pub branch_id: String,

    /// The ID of the snapshot
    #[clap(short = 's', long, required = true)]
    pub snapshot_id: String,

    /// Discard changes in the source branch
    #[clap(long, default_value = "false")]
    pub discard_changes: bool,

    /// Checkout the branch after creation
    #[clap(long, default_value = "false")]
    pub checkout: bool,

    /// Create an ephemeral branch
    #[clap(long, default_value = "false")]
    pub ephemeral: bool,
}

#[derive(Args, Debug)]
pub struct CheckoutBranchArgs {
    /// The ID of the deployment
    #[clap(short = 'x', long, required = true)]
    pub deployment_id: String,

    /// The ID of the branch to checkout
    #[clap(short = 'b', long, required = true)]
    pub branch_id: String,
}

#[derive(Args, Debug)]
pub struct GetBookmarkArgs {
    /// The ID of the deployment
    #[clap(short = 'x', long, required = true)]
    pub deployment_id: String,

    /// The ID of the branch
    #[clap(short = 'b', long, required = true)]
    pub branch_id: String,
}

#[derive(Args, Debug)]
pub struct CreateBookmarkArgs {
 /// The ID of the deployment
    #[clap(short = 'x', long, required = true)]
    pub deployment_id: String,

    /// The ID of the branch
    #[clap(short = 'b', long, required = true)]
    pub branch_id: String,

    /// A comment for the snapshot
    #[clap(short = 'm', long, required = true)]
    pub snapshot_comment: String,
}

#[derive(Args, Debug)]
pub struct CheckoutBookmarkArgs {
    /// The ID of the deployment
    #[clap(short = 'x', long, required = true)]
    pub deployment_id: String,

    /// The ID of the branch
    #[clap(short = 'b', long, required = true)]
    pub branch_id: String,

    /// The ID of the snapshot
    #[clap(short = 's', long, required = true)]
    pub snapshot_id: String,

    /// Discard changes in the source branch
    #[clap(long, default_value = "false")]
    pub discard_changes: bool,

    /// Checkout the bookmark after creation
    #[clap(long, default_value = "true")]
    pub checkout: bool,

    /// Create an ephemeral branch
    #[clap(long, default_value = "false")]
    pub ephemeral: bool,

    /// Performance profile name
    #[clap(long)]
    pub performance_profile_name: Option<String>,
}