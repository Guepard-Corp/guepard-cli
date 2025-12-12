# Complete Guepard CLI Command Reference

## Overview
Your Guepard CLI now follows a Git-like structure while preserving all your original functionality and beautiful table outputs. Updated for API v0.4.4.

## Commands

### Core Commands
- `guepard deploy` - Deploy database instances with beautiful table output
- `guepard commit -m "message" -x <deployment_id> -c <clone_id>` - Create snapshots (bookmarks)
- `guepard branch` - List and manage branches with detailed tables
- `guepard checkout` - Switch branches or checkout snapshots
- `guepard log` - Show commit history
- `guepard rev-parse` - Find .gfs directory

### Management Commands
- `guepard compute status|start|stop|restart|list|logs -x <deployment_id>` - Compute management
- `guepard show branches|commits -x <deployment_id>` - Show detailed information
- `guepard usage` - Show usage information

### Authentication
- `guepard link` - Start login process
- `guepard login -c <code>` - Complete login with verification code
- `guepard logout` - Log out and clear credentials

## Detailed Usage

### Deploy Command
```bash
# Create new deployment
guepard deploy -p PostgreSQL -v 16 -r us-west-aws -d us-west-aws -n myrepo -w password

# Create deployment with node ID
guepard deploy -p PostgreSQL -v 17 -r us-west-aws -d us-west-aws -n myrepo -w password -s <node_id>

# Update deployment
guepard deploy -x <deployment_id> -n <new_repository_name>

# Get deployment details
guepard deploy -x <deployment_id>

# Get deployment details as JSON
guepard deploy -x <deployment_id> --json
```

### Commit Command (Snapshots)
```bash
# Create a snapshot with message
guepard commit -m "Production snapshot" -x <deployment_id> -c <clone_id>
```

### Branch Command
```bash
# List branches for deployment
guepard branch -x <deployment_id>

# Create new branch
guepard branch -x <deployment_id> -s <snapshot_id> -n <branch_name> -k -e

# Git-like usage (simplified)
guepard branch <branch_name>  # Shows helpful message
guepard branch              # Shows helpful message
```

### Checkout Command
```bash
# Checkout branch
guepard checkout -x <deployment_id> -c <branch_id>

# Git-like usage (simplified)
guepard checkout <branch_name>  # Shows helpful message
```

### Compute Commands
```bash
# Get compute status
guepard compute status -x <deployment_id>

# Start compute
guepard compute start -x <deployment_id>

# Stop compute
guepard compute stop -x <deployment_id>

# Restart compute
guepard compute restart -x <deployment_id>

# List compute details
guepard compute list -x <deployment_id>

# View logs
guepard compute logs -x <deployment_id>
```

### Show Commands
```bash
# Show branches with active indicator
guepard show branches -x <deployment_id>

# Show commits
guepard show commits -x <deployment_id>
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
guepard init .

# Deploy database
guepard deploy -p PostgreSQL -v 16 -r us-west-aws -d us-west-aws -n myrepo -w password

# Create snapshot
guepard commit -m "Initial setup" -x <deployment_id> -c <clone_id>

# Create branch
guepard branch -x <deployment_id> -s <snapshot_id> -n feature-branch -k

# Manage compute
guepard compute start -x <deployment_id>
guepard compute status -x <deployment_id>

# Show branches
guepard show branches -x <deployment_id>
```

### Quick Commands
```bash
# List deployments
guepard deploy -x <deployment_id>

# List branches
guepard branch -x <deployment_id>

# Check compute status
guepard compute status -x <deployment_id>
```

## Migration Notes

- **No duplicate commands**: Removed unnecessary duplication
- **Preserved functionality**: All original features maintained
- **Enhanced output**: Beautiful tables and formatting
- **Git-like structure**: Familiar command organization
- **API v0.4.4**: Updated to latest API specifications

This design provides a clean, Git-like interface while maintaining all your original functionality and beautiful output formatting.