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
    /// Initialize a new Guepard environment
    Init(InitArgs),
    
    /// Deploy database instances
    Deploy(DeployArgs),
    
    /// Create snapshots (bookmarks)
    Commit(CommitArgs),
    
    /// List and manage branches
    Branch(BranchArgs),
    
    /// Show commit history
    Log,
    
    /// Find .gfs directory
    RevParse,
    
    /// Switch branches or checkout snapshots
    Checkout(CheckoutArgs),
    
            /// Compute instance management
            Compute(ComputeArgs),
    
    /// Show details about deployments, branches, etc.
    #[clap(subcommand)]
    Show(ShowCommand),
    
    /// Show usage information
    Usage,
    
    /// List deployments, branches, commits, etc.
    List(ListArgs),
    
    /// Interactive login process
    Login,
    
    /// Log out and clear all credentials
    Logout,
}

// Git-like command arguments
#[derive(Args, Debug)]
pub struct ListArgs {
    /// What to list (deployments, branches, commits)
    #[clap(value_parser, default_value = "deployments")]
    pub resource: String,
    
    /// Columns to display (comma-separated)
    /// Available columns: id, name, repository, provider, version, status, fqdn, region, datacenter, created
    #[clap(short = 'c', long)]
    pub columns: Option<String>,
    
    /// Deployment ID (required for branches/commits)
    #[clap(short = 'x', long)]
    pub deployment_id: Option<String>,
    
    /// Show git graph style (for commits only)
    #[clap(short = 'g', long)]
    pub graph: bool,
    
    /// Show all commits including AUTO SNAPs (for commits only)
    #[clap(short = 'a', long)]
    pub all: bool,
}

#[derive(Args, Debug)]
pub struct InitArgs {
    /// The path where to initialize the Guepard environment
    #[clap(value_parser, default_value = ".")]
    pub path: String,
}

#[derive(Args, Debug)]
pub struct DeployArgs {
    /// The database provider (e.g., PostgreSQL, MySQL)
    #[clap(short = 'p', long)]
    pub database_provider: Option<String>,
    
    /// The version of the database
    #[clap(short = 'v', long)]
    pub database_version: Option<String>,
    
    /// The region where the deployment will be created
    #[clap(short = 'r', long)]
    pub region: Option<String>,
    
    /// The deployment type (REPOSITORY or F2)
    #[clap(short = 'i', long)]
    pub instance_type: Option<String>,
    
    /// The datacenter for the deployment
    #[clap(short = 'd', long)]
    pub datacenter: Option<String>,
    
    /// The name of the repository
    #[clap(short = 'n', long)]
    pub repository_name: Option<String>,
    
    /// The password for the database
    #[clap(short = 'w', long)]
    pub database_password: Option<String>,
    
    /// The ID of the deployment (for get/update/delete operations)
    #[clap(short = 'x', long)]
    pub deployment_id: Option<String>,
    
    /// The username for the database
    #[clap(short = 'u', long)]
    pub user: Option<String>,
    
    /// Confirm deletion without prompting
    #[clap(short = 'y', long)]
    pub yes: bool,
    
    /// Performance profile label name (e.g., gp.g1.xsmall)
    #[clap(short = 'f', long)]
    pub performance_profile: Option<String>,
    
    /// Interactive mode - guide through deployment creation
    #[clap(short = 'I', long)]
    pub interactive: bool,
}

#[derive(Args, Debug)]
pub struct CommitArgs {
    /// The commit message
    #[clap(short = 'm', long, required = true)]
    pub message: String,
    
    /// The deployment ID
    #[clap(short = 'x', long, required = true)]
    pub deployment_id: String,
    
    /// The clone ID
    #[clap(short = 'c', long, required = true)]
    pub clone_id: String,
}

#[derive(Args, Debug)]
pub struct BranchArgs {
    /// The name of the branch to create (optional - if not provided, lists branches)
    #[clap(value_parser)]
    pub name: Option<String>,
    
    /// The ID of the deployment
    #[clap(short = 'x', long)]
    pub deployment_id: Option<String>,
    
    /// The ID of the snapshot
    #[clap(short = 's', long)]
    pub snapshot_id: Option<String>,
    
    /// Whether to discard changes
    #[clap(short = 'd', long)]
    pub discard_changes: Option<String>,
    
    /// Whether to checkout the branch after creation
    #[clap(short = 'k', long)]
    pub checkout: bool,
    
    /// Whether the branch is ephemeral
    #[clap(short = 'e', long)]
    pub ephemeral: bool,
    
    /// The source branch ID to create from
    #[clap(short = 'b', long)]
    pub source_branch_id: Option<String>,
}

#[derive(Args, Debug)]
pub struct CheckoutArgs {
    /// The branch name or commit hash to checkout
    #[clap(value_parser, required = true)]
    pub target: String,
    
    /// The ID of the deployment
    #[clap(short = 'x', long)]
    pub deployment_id: Option<String>,
    
    /// The ID of the branch
    #[clap(short = 'c', long)]
    pub clone_id: Option<String>,
    
    /// The ID of the snapshot
    #[clap(short = 's', long)]
    pub snapshot_id: Option<String>,
    
    /// Whether to discard changes
    #[clap(short = 'd', long)]
    pub discard_changes: Option<String>,
    
    /// Whether to checkout
    #[clap(short = 'k', long)]
    pub checkout: bool,
    
    /// The source branch ID to create from
    #[clap(short = 'b', long)]
    pub source_branch_id: Option<String>,
}


#[derive(Args, Debug)]
pub struct ComputeArgs {
    /// The ID of the deployment
    #[clap(short = 'x', long, required = true)]
    pub deployment_id: String,
    
    /// Action to perform: status, start, stop, restart, list, logs
    #[clap(short = 'a', long, default_value = "list")]
    pub action: String,
}

#[derive(Subcommand, Debug)]
pub enum ShowCommand {
    /// Show branch details
    Branches(ShowDeployArgs),
    /// Show commit details (snapshots)
    Commits(ShowDeployArgs),
}


#[derive(Args, Debug)]
pub struct ShowDeployArgs {
    /// The ID of the deployment
    #[clap(short = 'x', long, required = true)]
    pub deployment_id: String,
}

// Additional structs for original API compatibility
#[derive(Args, Debug)]
pub struct CreateBranchArgs {
    /// The ID of the deployment
    #[clap(short = 'x', long, required = true)]
    pub deployment_id: String,

    /// The ID of the snapshot
    #[clap(short = 's', long, required = true)]
    pub snapshot_id: String,

    /// The name of the branch
    #[clap(short = 'n', long, required = true)]
    pub branch_name: String,

    /// The source branch ID to create from
    #[clap(short = 'b', long, required = true)]
    pub source_branch_id: String,

    /// Whether to discard changes
    #[clap(short = 'd', long, required = true)]
    pub discard_changes: String,

    /// Whether to checkout the branch after creation
    #[clap(short = 'k', long)]
    pub checkout: bool,

    /// Whether the branch is ephemeral
    #[clap(short = 'e', long)]
    pub ephemeral: bool,
}

#[derive(Args, Debug)]
pub struct CheckoutBranchArgs {
    /// The ID of the deployment
    #[clap(short = 'x', long, required = true)]
    pub deployment_id: String,

    /// The ID of the branch
    #[clap(short = 'c', long, required = true)]
    pub branch_id: String,
}
