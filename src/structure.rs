use clap::{Args, Parser, Subcommand};

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[derive(Parser, Debug)]
#[clap(
    version = get_version(), 
    about = "🐆 Guepard CLI - Git for databases",
    long_about = "Guepard CLI provides Git-like capabilities for your databases. 
Create snapshots, manage branches, and deploy database instances with ease.",
    propagate_version = true
)]
pub struct CLI {
    #[clap(subcommand)]
    pub sub_commands: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// 🚀 Deploy database instances
    Deploy(DeployArgs),
    
    /// 📸 Create snapshots (like git commit)
    Commit(CommitArgs),
    
    /// 🌿 List and manage branches
    Branch(BranchArgs),
    
    /// 📋 Show commit history
    Log(LogArgs),
    
    /// 🔄 Switch branches or checkout snapshots
    Checkout(CheckoutArgs),
    
    /// 💻 Manage compute instances (start/stop/status)
    Compute(ComputeArgs),
    
    /// 📊 Show usage information
    Usage,
    
    /// 📋 List deployments, branches, commits, etc.
    List(ListArgs),
    
    /// 🔐 Login to Guepard (interactive)
    Login(LoginArgs),
    
    /// 🚪 Logout and clear credentials
    Logout,
    
    /// ⚙️ Configure API endpoint and other settings
    Config(ConfigArgs),
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
    
    /// Limit number of results to display (default: show all)
    #[clap(short = 'l', long)]
    pub limit: Option<usize>,
}

#[derive(Args, Debug)]
pub struct DeployArgs {
    /// Database type: PostgreSQL, MySQL, MongoDB
    #[clap(short = 'p', long)]
    pub database_provider: Option<String>,
    
    /// Database version (e.g., 16 for PostgreSQL)
    #[clap(short = 'v', long)]
    pub database_version: Option<String>,
    
    /// Region: us-west, us-east, eu-west, asia-pacific
    #[clap(short = 'r', long)]
    pub region: Option<String>,
    
    /// Type: REPOSITORY (for versioning) or F2 (for compute)
    #[clap(short = 'i', long)]
    pub instance_type: Option<String>,
    
    /// Cloud provider: aws, gcp, azure
    #[clap(short = 'd', long)]
    pub datacenter: Option<String>,
    
    /// Repository name (will be auto-generated if not provided)
    #[clap(short = 'n', long)]
    pub repository_name: Option<String>,
    
    /// Database password (required for creation)
    #[clap(short = 'w', long)]
    pub database_password: Option<String>,
    
    /// Deployment ID (for get/update/delete operations)
    #[clap(short = 'x', long)]
    pub deployment_id: Option<String>,
    
    /// Database username (defaults to 'guepard')
    #[clap(short = 'u', long)]
    pub user: Option<String>,
    
    /// Skip confirmation prompts
    #[clap(short = 'y', long)]
    pub yes: bool,
    
    /// Performance profile: gp.g1.xsmall, gp.g1.small, etc.
    #[clap(short = 'f', long)]
    pub performance_profile: Option<String>,
    
    /// Interactive mode - guided setup
    #[clap(short = 'I', long)]
    pub interactive: bool,
}

#[derive(Args, Debug)]
pub struct CommitArgs {
    /// Commit message (like git commit -m)
    #[clap(short = 'm', long, required = true)]
    pub message: String,
    
    /// Deployment ID
    #[clap(short = 'x', long, required = true)]
    pub deployment_id: String,
    
    /// Branch ID (will use current branch if not provided)
    #[clap(short = 'b', long, required = true)]
    pub branch_id: String,
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
pub struct LogArgs {
    /// The ID of the deployment
    #[clap(short = 'x', long, required = true)]
    pub deployment_id: String,
    
    /// Number of lines to show (default: 50)
    #[clap(short = 'n', long, default_value = "50")]
    pub lines: usize,
    
    /// Follow mode - stream logs in real-time
    #[clap(short = 'f', long)]
    pub follow: bool,
    
    /// Show only stdout logs
    #[clap(long)]
    pub stdout_only: bool,
    
    /// Show only stderr logs
    #[clap(long)]
    pub stderr_only: bool,
    
    /// Show timestamps
    #[clap(short = 't', long)]
    pub timestamps: bool,
    
    /// Filter logs from this date (format: YYYY-MM-DD or YYYY-MM-DD HH:MM:SS)
    #[clap(long)]
    pub since: Option<String>,
    
    /// Filter logs until this date (format: YYYY-MM-DD or YYYY-MM-DD HH:MM:SS)
    #[clap(long)]
    pub until: Option<String>,
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
    pub branch_id: Option<String>,
    
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
    /// Deployment ID
    #[clap(short = 'x', long, required = true)]
    pub deployment_id: String,
    
    /// Action: start, stop, status (defaults to info if not provided)
    #[clap(value_parser)]
    pub action: Option<String>,
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

#[derive(Args, Debug)]
pub struct LoginArgs {
    /// Direct access token input (skip interactive login)
    #[clap(short = 'c', long)]
    pub code: Option<String>,
}

#[derive(Args, Debug)]
pub struct ConfigArgs {
    /// Get current configuration
    #[clap(long)]
    pub get: bool,
    
    /// Set API endpoint URL
    #[clap(short = 'a', long)]
    pub api_url: Option<String>,
    
    /// Show current configuration
    #[clap(long)]
    pub show: bool,
}
