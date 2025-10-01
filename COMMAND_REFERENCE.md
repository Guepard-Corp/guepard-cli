# Complete Guepard CLI Command Reference

## Overview
Your Guepard CLI now follows a Git-like structure while preserving all your original functionality and beautiful table outputs. Updated for API v0.4.4.

## Commands

### Core Commands
- `gfs init [path]` - Initialize a new Guepard environment
- `gfs deploy` - Deploy database instances with beautiful table output
- `gfs commit -m "message" -x <deployment_id> -c <clone_id>` - Create snapshots (bookmarks)
- `gfs branch` - List and manage branches with detailed tables
- `gfs checkout` - Switch branches or checkout snapshots
- `gfs log` - Show commit history
- `gfs rev-parse` - Find .gfs directory

### Management Commands
- `gfs compute status|start|stop|restart|list|logs -x <deployment_id>` - Compute management
- `gfs show branches|bookmarks -x <deployment_id>` - Show detailed information
- `gfs usage` - Show usage information

### Authentication
- `gfs link` - Start login process
- `gfs login -c <code>` - Complete login with verification code
- `gfs logout` - Log out and clear credentials

## Detailed Usage

### Deploy Command
```bash
# Create new deployment
gfs deploy -p PostgreSQL -v 16 -r us-west-aws -d us-west-aws -n myrepo -w password

# Update deployment
gfs deploy -x <deployment_id> -n <new_repository_name>

# Get deployment details
gfs deploy -x <deployment_id>
```

### Commit Command (Snapshots)
```bash
# Create a snapshot with message
gfs commit -m "Production snapshot" -x <deployment_id> -c <clone_id>
```

### Branch Command
```bash
# List branches for deployment
gfs branch -x <deployment_id>

# Create new branch
gfs branch -x <deployment_id> -s <snapshot_id> -n <branch_name> -k -e

# Git-like usage (simplified)
gfs branch <branch_name>  # Shows helpful message
gfs branch              # Shows helpful message
```

### Checkout Command
```bash
# Checkout branch
gfs checkout -x <deployment_id> -c <branch_id>

# Git-like usage (simplified)
gfs checkout <branch_name>  # Shows helpful message
```

### Compute Commands
```bash
# Get compute status
gfs compute status -x <deployment_id>

# Start compute
gfs compute start -x <deployment_id>

# Stop compute
gfs compute stop -x <deployment_id>

# Restart compute
gfs compute restart -x <deployment_id>

# List compute details
gfs compute list -x <deployment_id>

# View logs
gfs compute logs -x <deployment_id>
```

### Show Commands
```bash
# Show branches with active indicator
gfs show branches -x <deployment_id>

# Show bookmarks
gfs show bookmarks -x <deployment_id>
```

## Beautiful Output Features

### Table Formatting
All commands use beautiful tables with:
- **Colored headers** and status indicators
- **Rounded borders** for modern look
- **Emoji indicators** for active items (🐆 for active branches)
- **Status colors** (✅ green, ❌ red, ℹ️ blue, 💡 yellow)

### Enhanced Display
- **Deployment tables** show ID, name, repository, provider, version, status, FQDN
- **Branch tables** show ID, name, status, snapshot, environment, ephemeral status
- **Compute tables** show detailed instance information
- **Status tables** show current state with messages
- **Log output** with colored stdout/stderr separation

## API v0.4.4 Features

### New Capabilities
- **Deployment Types**: REPOSITORY and F2 deployments
- **Performance Profiles**: Configurable performance profiles
- **Enhanced Branching**: Improved branch management
- **Streamlined Compute**: Simplified compute operations
- **Better Error Handling**: More detailed error responses

### Updated Endpoints
- `/deploy/{deployment_id}/branch` - Branch management
- `/deploy/{deployment_id}/compute/*` - Compute operations
- `/deploy/{deployment_id}/checkout` - Checkout operations
- `/deploy/{deployment_id}/snapshot` - Snapshot management

## Workflow Examples

### Complete Workflow
```bash
# Initialize environment
gfs init .

# Deploy database
gfs deploy -p PostgreSQL -v 16 -r us-west-aws -d us-west-aws -n myrepo -w password

# Create snapshot
gfs commit -m "Initial setup" -x <deployment_id> -c <clone_id>

# Create branch
gfs branch -x <deployment_id> -s <snapshot_id> -n feature-branch -k

# Manage compute
gfs compute start -x <deployment_id>
gfs compute status -x <deployment_id>

# Show branches
gfs show branches -x <deployment_id>
```

### Quick Commands
```bash
# List deployments
gfs deploy -x <deployment_id>

# List branches
gfs branch -x <deployment_id>

# Check compute status
gfs compute status -x <deployment_id>
```

## Migration Notes

- **No duplicate commands**: Removed unnecessary duplication
- **Preserved functionality**: All original features maintained
- **Enhanced output**: Beautiful tables and formatting
- **Git-like structure**: Familiar command organization
- **API v0.4.4**: Updated to latest API specifications

This design provides a clean, Git-like interface while maintaining all your original functionality and beautiful output formatting.