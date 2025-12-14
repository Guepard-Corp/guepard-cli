use clap::{Args, Parser, Subcommand};

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[derive(Parser, Debug)]
#[clap(
    version = get_version(), 
    about = "🐆 Guepard CLI - Git for databases",
    long_about = "Guepard CLI brings Git-like version control capabilities to your databases.

Create snapshots (commits), manage branches, and deploy database instances with the same 
intuitive workflow you're already familiar with from Git. Perfect for development teams 
who want safe, collaborative database management.

Key Features:
  • Database snapshots: Create point-in-time captures of your database state
  • Branching: Work on database changes in parallel without conflicts
  • Easy rollbacks: Instantly revert to any previous database state
  • Multi-database support: PostgreSQL, MySQL, and MongoDB
  • Beautiful output: Colorized tables and intuitive command responses

Quick Start:
  1. Authenticate: guepard login
  2. Deploy: guepard deploy --interactive
  3. Commit: guepard commit -m \"message\" -x <deployment_id> -b <branch_id>
  4. Branch: guepard branch -x <deployment_id> -s <snapshot_id> -n <name>

For detailed documentation, visit: https://docs.guepard.run",
    propagate_version = true
)]
pub struct CLI {
    #[clap(subcommand)]
    pub sub_commands: SubCommand,
}

/// Shared output format option for all commands
#[derive(Args, Debug)]
pub struct OutputArgs {
    /// Output results as JSON instead of tables
    ///
    /// Use this flag to get machine-readable JSON output instead of formatted tables.
    ///
    /// Examples:
    ///   guepard list deployments --json
    ///   guepard deploy -x <id> --json
    #[clap(long = "json")]
    pub json: bool,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// 🚀 Deploy, manage, and configure database instances
    ///
    /// Create new database deployments, view deployment details, update configurations,
    /// or delete deployments. Supports PostgreSQL, MySQL, and MongoDB.
    ///
    /// Examples:
    ///   # Interactive deployment (recommended for first-time users)
    ///   guepard deploy --interactive
    ///
    ///   # Create a new PostgreSQL deployment
    ///   guepard deploy -p PostgreSQL -v 16 -r us-west -i REPOSITORY -d aws -n myapp -w password
    ///
    ///   # View deployment details
    ///   guepard deploy -x <deployment_id>
    ///
    ///   # Update deployment repository name
    ///   guepard deploy -x <deployment_id> -n new-name
    ///
    ///   # Delete deployment (use with caution)
    ///   guepard deploy -x <deployment_id> --yes
    Deploy(DeployArgs),
    
    /// 📸 Create snapshots of your database state (like git commit)
    ///
    /// Create a point-in-time snapshot of your database. Snapshots capture schema changes,
    /// data modifications, and configuration. Use descriptive messages to track changes.
    ///
    /// Examples:
    ///   # Create a snapshot with a message
    ///   guepard commit -m "Add user authentication tables" -x <deployment_id> -b <branch_id>
    ///
    ///   # Create multiple snapshots during development
    ///   guepard commit -m "Initial schema" -x <deployment_id> -b <branch_id>
    ///   guepard commit -m "Add user profiles" -x <deployment_id> -b <branch_id>
    ///   guepard commit -m "Add payment tables" -x <deployment_id> -b <branch_id>
    Commit(CommitArgs),
    
    /// 🌿 List and manage branches for your deployments
    ///
    /// List all branches for a deployment or create new branches from snapshots.
    /// Branches allow parallel development without conflicts. Use ephemeral branches
    /// for experiments and feature branches for development.
    ///
    /// Examples:
    ///   # List all branches
    ///   guepard branch -x <deployment_id>
    ///
    ///   # Create a new feature branch
    ///   guepard branch -x <deployment_id> -s <snapshot_id> -n feature-auth --checkout --ephemeral
    ///
    ///   # Create a bugfix branch
    ///   guepard branch -x <deployment_id> -s <snapshot_id> -n bugfix/login-error -k -e
    Branch(BranchArgs),
    
    /// 📋 View deployment logs and commit history
    ///
    /// Monitor deployment activity, view logs in real-time, or filter logs by date.
    /// Useful for debugging, monitoring, and understanding deployment activity.
    ///
    /// Examples:
    ///   # View recent logs (default: 50 lines)
    ///   guepard log -x <deployment_id>
    ///
    ///   # Follow logs in real-time
    ///   guepard log -x <deployment_id> --follow
    ///
    ///   # View logs with timestamps
    ///   guepard log -x <deployment_id> --timestamps
    ///
    ///   # Filter logs by date range
    ///   guepard log -x <deployment_id> --since "2025-01-08" --until "2025-01-09"
    Log(LogArgs),
    
    /// 🔄 Switch branches or checkout specific snapshots
    ///
    /// Change your database state to match a different branch or restore to a previous
    /// snapshot. Similar to git checkout but affects your entire database state.
    ///
    /// Examples:
    ///   # Checkout a branch
    ///   guepard checkout -x <deployment_id> -c <branch_id>
    ///
    ///   # Restore to a specific snapshot
    ///   guepard checkout -x <deployment_id> -s <snapshot_id>
    ///
    ///   # List available branches (shows helpful message)
    ///   guepard checkout -x <deployment_id>
    Checkout(CheckoutArgs),
    
    /// 💻 Manage compute instances (start, stop, status, logs)
    ///
    /// Control the compute resources for your deployments. Start or stop compute instances,
    /// check status, and view compute-specific logs.
    ///
    /// Examples:
    ///   # Check compute status
    ///   guepard compute status -x <deployment_id>
    ///
    ///   # Start compute instance
    ///   guepard compute start -x <deployment_id>
    ///
    ///   # Stop compute instance
    ///   guepard compute stop -x <deployment_id>
    ///
    ///   # View compute logs
    ///   guepard compute logs -x <deployment_id>
    ///
    ///   # List compute details
    ///   guepard compute list -x <deployment_id>
    Compute(ComputeArgs),
    
    /// 📊 Show account usage and quota information
    ///
    /// Display your current usage statistics including deployments, snapshots, and clones.
    /// Helps you monitor resource consumption and plan capacity.
    ///
    /// Example:
    ///   guepard usage
    Usage,
    
    /// 📋 List deployments, branches, commits, and other resources
    ///
    /// Display resources in beautiful, colorized tables. Supports custom column selection
    /// and filtering. Default resource is 'deployments'.
    ///
    /// Examples:
    ///   # List all deployments
    ///   guepard list deployments
    ///
    ///   # List deployments with specific columns
    ///   guepard list deployments --columns id,name,status,fqdn
    ///
    ///   # List branches for a deployment
    ///   guepard list branches -x <deployment_id>
    ///
    ///   # List commits with git-style graph
    ///   guepard list commits -x <deployment_id> --graph
    ///
    ///   # List all commits including AUTO SNAPs
    ///   guepard list commits -x <deployment_id> --all
    List(ListArgs),
    
    /// 🔐 Authenticate with your Guepard account
    ///
    /// Login to Guepard using interactive browser-based authentication or provide
    /// an access token directly. Required before using most commands.
    ///
    /// Examples:
    ///   # Interactive login (opens browser)
    ///   guepard login
    ///
    ///   # Direct token login
    ///   guepard login --code your-access-token
    Login(LoginArgs),
    
    /// 🚪 Logout and clear stored credentials
    ///
    /// Sign out from Guepard and remove locally stored authentication credentials.
    ///
    /// Example:
    ///   guepard logout
    Logout,
    
    /// ⚙️ Configure API endpoint and other settings
    ///
    /// View or modify CLI configuration including API endpoint URL and other settings.
    ///
    /// Examples:
    ///   # Show current configuration
    ///   guepard config --show
    ///
    ///   # Set API endpoint
    ///   guepard config --api-url https://api.guepard.run
    ///
    ///   # Get specific configuration value
    ///   guepard config --get
    Config(ConfigArgs),
    
    /// 🎭 Clone deployments from snapshots
    ///
    /// Create shadow/clone deployments from snapshots or list existing clones.
    /// Clones are read-only copies used for testing, analysis, or backup purposes.
    ///
    /// Examples:
    ///   # Create a clone from a snapshot
    ///   guepard clone -x <deployment_id> -s <snapshot_id>
    ///
    ///   # List all clones for a deployment
    ///   guepard clone list -x <deployment_id>
    Clone(CloneArgs),
}

// Git-like command arguments
#[derive(Args, Debug)]
pub struct ListArgs {
    #[clap(flatten)]
    pub output: OutputArgs,
    
    /// Resource type to list: deployments, branches, or commits
    ///
    /// - deployments: List all database deployments (default)
    /// - branches: List branches for a deployment (requires --deployment-id)
    /// - commits: List snapshots/commits for a deployment (requires --deployment-id)
    #[clap(value_parser, default_value = "deployments")]
    pub resource: String,
    
    /// Columns to display (comma-separated list)
    ///
    /// Available columns for deployments:
    ///   id, name, repository, provider, version, status, fqdn, region, datacenter, created
    ///
    /// Example: --columns id,name,status,fqdn
    #[clap(short = 'c', long)]
    pub columns: Option<String>,
    
    /// Deployment ID (required for listing branches or commits)
    ///
    /// Use this flag when listing branches or commits to specify which deployment
    /// to query. Can be omitted when listing deployments.
    #[clap(short = 'x', long)]
    pub deployment_id: Option<String>,
    
    /// Show git-style graph visualization (for commits only)
    ///
    /// Displays commit history in a visual graph format similar to 'git log --graph'.
    /// Shows branch relationships and commit hierarchy.
    #[clap(short = 'g', long)]
    pub graph: bool,
    
    /// Show all commits including AUTO SNAPs (for commits only)
    ///
    /// By default, AUTO SNAPs (automatic snapshots) are hidden. Use this flag
    /// to include them in the output.
    #[clap(short = 'a', long)]
    pub all: bool,
    
    /// Limit number of results to display
    ///
    /// Restrict the number of items shown in the output. Useful for pagination
    /// or when dealing with large result sets. Default: show all results.
    #[clap(short = 'l', long)]
    pub limit: Option<usize>,
}

#[derive(Args, Debug)]
pub struct DeployArgs {
    #[clap(flatten)]
    pub output: OutputArgs,
    
    /// Database provider type
    ///
    /// Supported providers: PostgreSQL, MySQL, MongoDB
    /// Required when creating a new deployment.
    #[clap(short = 'p', long)]
    pub database_provider: Option<String>,
    
    /// Database version number
    ///
    /// Examples:
    ///   - PostgreSQL: 16, 15, 14
    ///   - MySQL: 8.0, 5.7
    ///   - MongoDB: 7.0, 6.0
    ///
    /// Required when creating a new deployment.
    #[clap(short = 'v', long)]
    pub database_version: Option<String>,
    
    /// Geographic region for deployment
    ///
    /// Available regions: us-west, us-east, eu-west, asia-pacific
    /// Choose the region closest to your users for best performance.
    /// Required when creating a new deployment.
    #[clap(short = 'r', long)]
    pub region: Option<String>,
    
    /// Deployment instance type
    ///
    /// - REPOSITORY: Full version control, optimized for development and branching
    /// - F2: High-performance compute, optimized for production workloads
    ///
    /// Required when creating a new deployment.
    #[clap(short = 'i', long)]
    pub instance_type: Option<String>,
    
    /// Cloud provider datacenter
    ///
    /// Supported providers: aws, gcp, azure
    /// Required when creating a new deployment.
    #[clap(short = 'd', long)]
    pub datacenter: Option<String>,
    
    /// Repository name for the deployment
    ///
    /// A unique identifier for your deployment. Must be alphanumeric with hyphens allowed.
    /// If not provided, a name will be auto-generated.
    /// Required when creating a new deployment.
    #[clap(short = 'n', long)]
    pub repository_name: Option<String>,
    
    /// Database password
    ///
    /// The password for the database user. Use a strong, unique password.
    /// Required when creating a new deployment.
    #[clap(short = 'w', long)]
    pub database_password: Option<String>,
    
    /// Deployment ID for get/update/delete operations
    ///
    /// Use this flag to:
    ///   - View deployment details: guepard deploy -x <deployment_id>
    ///   - Update deployment: guepard deploy -x <deployment_id> -n new-name
    ///   - Delete deployment: guepard deploy -x <deployment_id> --yes
    #[clap(short = 'x', long)]
    pub deployment_id: Option<String>,
    
    /// Database username
    ///
    /// The username for database connections. Defaults to 'guepard' if not specified.
    #[clap(short = 'u', long)]
    pub user: Option<String>,
    
    /// Skip confirmation prompts
    ///
    /// Automatically confirm destructive operations like deletion without prompting.
    /// Use with caution.
    #[clap(short = 'y', long)]
    pub yes: bool,
    
    /// Performance profile for the deployment
    ///
    /// Available profiles:
    ///   - gp.g1.xsmall: 1 vCPU, 2GB RAM (development, testing)
    ///   - gp.g1.small: 2 vCPU, 4GB RAM (small production, staging)
    ///   - gp.g1.medium: 4 vCPU, 8GB RAM (medium production)
    ///   - gp.g1.large: 8 vCPU, 16GB RAM (large production, high-traffic)
    ///
    /// If not specified, defaults to gp.g1.xsmall.
    #[clap(short = 'f', long)]
    pub performance_profile: Option<String>,
    
    /// Node ID for the deployment
    ///
    /// Optional node identifier for the deployment. Used for specific node targeting.
    #[clap(short = 's', long)]
    pub node_id: Option<String>,
    
    /// Interactive mode - guided setup wizard
    ///
    /// Launches an interactive wizard that guides you through deployment creation
    /// step by step. Recommended for first-time users or complex deployments.
    #[clap(short = 'I', long)]
    pub interactive: bool,
}

#[derive(Args, Debug)]
pub struct CommitArgs {
    #[clap(flatten)]
    pub output: OutputArgs,
    
    /// Commit message describing the snapshot
    ///
    /// Similar to git commit messages. Use descriptive messages that explain what
    /// changes were made. Good examples:
    ///   - "Add user authentication tables"
    ///   - "Fix foreign key constraint on orders table"
    ///   - "Add indexes for performance optimization"
    ///
    /// Bad examples: "fix", "update", "changes"
    #[clap(short = 'm', long, required = true)]
    pub message: String,
    
    /// Deployment ID where the snapshot will be created
    ///
    /// The unique identifier of the deployment. You can find this using:
    ///   guepard list deployments
    #[clap(short = 'x', long, required = true)]
    pub deployment_id: String,
    
    /// Branch ID where the snapshot will be created
    ///
    /// The unique identifier of the branch. List branches with:
    ///   guepard branch -x <deployment_id>
    ///
    /// The snapshot will be created on this branch, capturing the current
    /// database state at the time of commit.
    #[clap(short = 'b', long, required = true)]
    pub branch_id: String,
}

#[derive(Args, Debug)]
pub struct BranchArgs {
    #[clap(flatten)]
    pub output: OutputArgs,
    
    /// Branch name to create (optional - if omitted, lists all branches)
    ///
    /// Use descriptive names following conventions:
    ///   - feature/user-authentication
    ///   - bugfix/login-error
    ///   - hotfix/security-patch
    ///   - experiment/new-architecture
    ///
    /// If not provided, the command will list all branches for the deployment.
    #[clap(value_parser)]
    pub name: Option<String>,
    
    /// Deployment ID
    ///
    /// Required for both listing and creating branches. Find deployments with:
    ///   guepard list deployments
    #[clap(short = 'x', long)]
    pub deployment_id: Option<String>,
    
    /// Snapshot ID to branch from
    ///
    /// The snapshot that will serve as the starting point for the new branch.
    /// List available snapshots with:
    ///   guepard list commits -x <deployment_id>
    ///
    /// Required when creating a new branch.
    #[clap(short = 's', long)]
    pub snapshot_id: Option<String>,
    
    /// Whether to discard uncommitted changes when creating branch
    ///
    /// Set to "true" to discard any uncommitted changes before creating the branch.
    /// Use when you want a clean branch from the specified snapshot.
    #[clap(short = 'd', long)]
    pub discard_changes: Option<String>,
    
    /// Automatically checkout the branch after creation
    ///
    /// If set, switches to the newly created branch immediately after creation.
    /// Equivalent to running 'guepard checkout' after branch creation.
    #[clap(short = 'k', long)]
    pub checkout: bool,
    
    /// Mark branch as ephemeral (temporary)
    ///
    /// Ephemeral branches are intended for experiments and can be safely cleaned up
    /// after merging or discarding. Use for feature branches, bugfixes, and experiments.
    #[clap(short = 'e', long)]
    pub ephemeral: bool,
    
    /// Source branch ID to create from
    ///
    /// The branch to use as the source when creating a new branch. If not specified,
    /// the branch will be created from the snapshot specified by --snapshot-id.
    #[clap(short = 'b', long)]
    pub source_branch_id: Option<String>,
}

#[derive(Args, Debug)]
pub struct LogArgs {
    #[clap(flatten)]
    pub output: OutputArgs,
    
    /// Deployment ID to view logs for
    ///
    /// Required. Find deployment IDs with:
    ///   guepard list deployments
    #[clap(short = 'x', long, required = true)]
    pub deployment_id: String,
    
    /// Number of log lines to display
    ///
    /// Controls how many lines of log history to show. Default: 50.
    /// Use a larger number to see more history, or combine with --follow for real-time streaming.
    #[clap(short = 'n', long, default_value = "50")]
    pub lines: usize,
    
    /// Follow mode - stream logs in real-time
    ///
    /// Similar to 'tail -f', continuously displays new log entries as they are generated.
    /// Press Ctrl+C to stop following. Useful for monitoring active deployments.
    #[clap(short = 'f', long)]
    pub follow: bool,
    
    /// Show only stdout logs
    ///
    /// Filter to display only standard output logs, excluding error logs.
    /// Useful when you only want to see application output.
    #[clap(long)]
    pub stdout_only: bool,
    
    /// Show only stderr logs
    ///
    /// Filter to display only error logs, excluding standard output.
    /// Useful for debugging errors and warnings.
    #[clap(long)]
    pub stderr_only: bool,
    
    /// Display timestamps with each log line
    ///
    /// Adds timestamp information to each log entry, making it easier to correlate
    /// logs with specific events or time periods.
    #[clap(short = 't', long)]
    pub timestamps: bool,
    
    /// Filter logs from this date/time onwards
    ///
    /// Show only logs created on or after this date/time.
    /// Format: YYYY-MM-DD or YYYY-MM-DD HH:MM:SS
    ///
    /// Examples:
    ///   --since "2025-01-08"
    ///   --since "2025-01-08 14:30:00"
    #[clap(long)]
    pub since: Option<String>,
    
    /// Filter logs until this date/time
    ///
    /// Show only logs created on or before this date/time.
    /// Format: YYYY-MM-DD or YYYY-MM-DD HH:MM:SS
    ///
    /// Examples:
    ///   --until "2025-01-09"
    ///   --until "2025-01-09 18:00:00"
    ///
    /// Can be combined with --since to create a date range.
    #[clap(long)]
    pub until: Option<String>,
}

#[derive(Args, Debug)]
pub struct CheckoutArgs {
    #[clap(flatten)]
    pub output: OutputArgs,
    
    /// Deployment ID
    ///
    /// Required for checkout operations. Find deployments with:
    ///   guepard list deployments
    #[clap(short = 'x', long)]
    pub deployment_id: Option<String>,
    
    /// Branch ID to checkout
    ///
    /// The unique identifier of the branch to switch to. List branches with:
    ///   guepard branch -x <deployment_id>
    ///
    /// When you checkout a branch, your database state changes to match that branch.
    #[clap(short = 'c', long)]
    pub branch_id: Option<String>,
    
    /// Snapshot ID to checkout
    ///
    /// The unique identifier of the snapshot to restore to. List snapshots with:
    ///   guepard list commits -x <deployment_id>
    ///
    /// Restores your database to the exact state captured in this snapshot.
    /// Useful for rollbacks and testing previous states.
    #[clap(short = 's', long)]
    pub snapshot_id: Option<String>,
    
    /// Discard uncommitted changes before checkout
    ///
    /// Set to "true" to discard any uncommitted changes before switching branches
    /// or restoring to a snapshot. Use when you want a clean state.
    #[clap(short = 'd', long)]
    pub discard_changes: Option<String>,
    
    /// Perform the checkout operation
    ///
    /// Explicitly perform the checkout. This flag is typically used internally
    /// but can be specified for clarity.
    #[clap(short = 'k', long)]
    pub checkout: bool,
    
    /// Source branch ID for branch creation
    ///
    /// When creating a new branch during checkout, specify the source branch.
    /// This is used in advanced workflows where checkout creates a new branch.
    #[clap(short = 'b', long)]
    pub source_branch_id: Option<String>,
}


#[derive(Args, Debug)]
pub struct ComputeArgs {
    #[clap(flatten)]
    pub output: OutputArgs,
    
    /// Deployment ID to manage compute for
    ///
    /// Required. Find deployment IDs with:
    ///   guepard list deployments
    #[clap(short = 'x', long, required = true)]
    pub deployment_id: String,
    
    /// Action to perform on compute instance
    ///
    /// Available actions:
    ///   - status: Check current compute status (running, stopped, etc.)
    ///   - start: Start the compute instance
    ///   - stop: Stop the compute instance
    ///   - restart: Restart the compute instance
    ///   - logs: View compute-specific logs
    ///   - list: Show detailed compute information (default if not specified)
    ///
    /// Examples:
    ///   guepard compute status -x <deployment_id>
    ///   guepard compute start -x <deployment_id>
    ///   guepard compute stop -x <deployment_id>
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
    #[clap(flatten)]
    pub output: OutputArgs,
    
    /// Direct access token input (skip interactive login)
    ///
    /// Provide your Guepard access token directly instead of using interactive
    /// browser-based authentication. Useful for automation and CI/CD pipelines.
    ///
    /// Get your access token from: https://guepard.run/account/tokens
    ///
    /// Example:
    ///   guepard login --code your-access-token-here
    #[clap(short = 'c', long)]
    pub code: Option<String>,
}

#[derive(Args, Debug)]
pub struct ConfigArgs {
    #[clap(flatten)]
    pub output: OutputArgs,
    
    /// Get current configuration value
    ///
    /// Display the current value of a configuration setting.
    /// Use with other flags to get specific values.
    #[clap(long)]
    pub get: bool,
    
    /// Set API endpoint URL
    ///
    /// Configure the API endpoint that the CLI uses to communicate with Guepard.
    /// Default: https://api.guepard.run
    ///
    /// Example:
    ///   guepard config --api-url https://api.guepard.run
    #[clap(short = 'a', long)]
    pub api_url: Option<String>,
    
    /// Show all current configuration settings
    ///
    /// Display all configuration values including API endpoint and other settings.
    ///
    /// Example:
    ///   guepard config --show
    #[clap(long)]
    pub show: bool,
}

#[derive(Args, Debug)]
pub struct CloneArgs {
    #[clap(flatten)]
    pub output: OutputArgs,
    
    /// Deployment ID
    ///
    /// Required for creating a clone. Also used with 'list' subcommand.
    #[clap(short = 'x', long)]
    pub deployment_id: Option<String>,
    
    /// Snapshot ID to clone from
    ///
    /// Required when creating a clone. If provided with deployment_id, creates a new clone.
    #[clap(short = 's', long)]
    pub snapshot_id: Option<String>,
    
    /// Repository name for the clone
    ///
    /// A unique identifier for the clone deployment.
    #[clap(short = 'n', long)]
    pub repository_name: Option<String>,
    
    /// Branch name for the clone
    ///
    /// The branch name to use for the clone.
    #[clap(short = 'b', long)]
    pub branch_name: Option<String>,
    
    /// Performance profile for the clone
    ///
    /// Available profiles:
    ///   - gp.g1.xsmall: 1 vCPU, 2GB RAM (development, testing)
    ///   - gp.g1.small: 2 vCPU, 4GB RAM (small production, staging)
    ///   - gp.g1.medium: 4 vCPU, 8GB RAM (medium production)
    ///   - gp.g1.large: 8 vCPU, 16GB RAM (large production, high-traffic)
    ///
    /// If not specified, defaults to gp.g1.xsmall.
    #[clap(short = 'f', long)]
    pub performance_profile: Option<String>,
    
    #[clap(subcommand)]
    pub command: Option<CloneSubCommand>,
}

#[derive(Subcommand, Debug)]
pub enum CloneSubCommand {
    /// List all clones for a deployment
    ///
    /// Lists all shadow/clone deployments associated with a specific deployment.
    ///
    /// Example:
    ///   guepard clone list -x <deployment_id>
    List {
        /// Deployment ID
        #[clap(short = 'x', long, required = true)]
        deployment_id: String,
    },
}
