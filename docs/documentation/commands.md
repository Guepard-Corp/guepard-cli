# Complete Command Reference

This comprehensive reference covers all Guepard CLI commands with detailed examples, options, and use cases.

## Command Overview

Guepard CLI follows a Git-like structure with these main command categories:

- **Core Commands**: `deploy`, `commit`, `branch`, `checkout`, `log`
- **Management Commands**: `compute`, `list`, `usage`
- **Authentication**: `login`, `logout`

## Core Commands

### `guepard deploy` - Database Deployment

Deploy, manage, and configure database instances.

#### Syntax
```bash
guepard deploy [OPTIONS]
```

#### Options
| Option | Short | Description | Required |
|--------|-------|-------------|----------|
| `--database-provider` | `-p` | Database type (PostgreSQL, MySQL, MongoDB) | For creation |
| `--database-version` | `-v` | Database version (e.g., 16 for PostgreSQL) | For creation |
| `--region` | `-r` | Region (us-west, us-east, eu-west, asia-pacific) | For creation |
| `--instance-type` | `-i` | Type (REPOSITORY, F2) | For creation |
| `--datacenter` | `-d` | Cloud provider (aws, gcp, azure) | For creation |
| `--repository-name` | `-n` | Repository name | For creation |
| `--database-password` | `-w` | Database password | For creation |
| `--deployment-id` | `-x` | Deployment ID | For get/update/delete |
| `--user` | `-u` | Database username (default: guepard) | No |
| `--yes` | `-y` | Skip confirmation prompts | No |
| `--performance-profile` | `-f` | Performance profile | No |
| `--node-id` | `-s` | Node ID for deployment | No |
| `--interactive` | `-I` | Interactive mode | No |
| `--json` | | Output results as JSON | No |

#### Examples

**Create a new deployment:**
```bash
guepard deploy \
  --database-provider PostgreSQL \
  --database-version 16 \
  --region us-west \
  --instance-type REPOSITORY \
  --datacenter aws \
  --repository-name myapp \
  --database-password secret123
```

**Create deployment with node ID:**
```bash
guepard deploy \
  --database-provider PostgreSQL \
  --database-version 17 \
  --region us-west-aws \
  --instance-type REPOSITORY \
  --datacenter us-west-aws \
  --repository-name db-new-api \
  --database-password guepard \
  --node-id <node_id>
```

**Or using short flag:**
```bash
guepard deploy -p PostgreSQL -v 17 -r us-west-aws -i REPOSITORY -d us-west-aws -n db-new-api -w guepard -s <node_id>
```

**Get deployment details as JSON:**
```bash
guepard deploy --deployment-id <id> --json
```

**Interactive deployment:**
```bash
guepard deploy --interactive
```

**Get deployment details:**
```bash
guepard deploy --deployment-id 12345678-1234-1234-1234-123456789abc
```

**Update deployment:**
```bash
guepard deploy \
  --deployment-id 12345678-1234-1234-1234-123456789abc \
  --repository-name new-name
```

**Delete deployment:**
```bash
guepard deploy \
  --deployment-id 12345678-1234-1234-1234-123456789abc \
  --yes
```

### `guepard commit` - Create Snapshots

Create snapshots (commits) of your database state.

#### Syntax
```bash
guepard commit --message <message> --deployment-id <id> --branch-id <id>
```

#### Options
| Option | Short | Description | Required |
|--------|-------|-------------|----------|
| `--message` | `-m` | Commit message | Yes |
| `--deployment-id` | `-x` | Deployment ID | Yes |
| `--branch-id` | `-b` | Branch ID | Yes |
| `--json` | | Output results as JSON | No |

#### Examples

**Create a snapshot:**
```bash
guepard commit \
  --message "Add user authentication tables" \
  --deployment-id 12345678-1234-1234-1234-123456789abc \
  --branch-id a7d373a3-4244-47b7-aacb-ad366f2520f6
```

**Create multiple snapshots:**
```bash
# Initial setup
guepard commit -m "Initial schema" -x <deployment_id> -b <branch_id>

# Add features
guepard commit -m "Add user profiles" -x <deployment_id> -b <branch_id>
guepard commit -m "Add payment tables" -x <deployment_id> -b <branch_id>
```

**Create snapshot and output as JSON:**
```bash
guepard commit -m "Initial schema" -x <deployment_id> -b <branch_id> --json
```

### `guepard branch` - Branch Management

List and create branches for your deployments.

#### Syntax
```bash
guepard branch [OPTIONS] [NAME]
```

#### Options
| Option | Short | Description | Required |
|--------|-------|-------------|----------|
| `--deployment-id` | `-x` | Deployment ID | For listing/creating |
| `--snapshot-id` | `-s` | Snapshot ID to branch from | For creation |
| `--name` | `-n` | Branch name | For creation |
| `--checkout` | `-k` | Checkout after creation | No |
| `--json` | | Output results as JSON | No |
| `--ephemeral` | `-e` | Create ephemeral branch | No |
| `--source-branch-id` | `-b` | Source branch ID | No |
| `--discard-changes` | `-d` | Discard changes | No |

#### Examples

**List branches:**
```bash
guepard branch --deployment-id 12345678-1234-1234-1234-123456789abc
```

**List branches as JSON:**
```bash
guepard branch --deployment-id 12345678-1234-1234-1234-123456789abc --json
```

**Create a new branch:**
```bash
guepard branch \
  --deployment-id 12345678-1234-1234-1234-123456789abc \
  --snapshot-id abc12345-6789-1234-5678-123456789abc \
  --name feature-auth \
  --checkout \
  --ephemeral
```

**Create branch and output as JSON:**
```bash
guepard branch -x <deployment_id> -s <snapshot_id> -n feature-auth --json
```

**Git-like usage:**
```bash
# Shows helpful message
guepard branch feature-auth
guepard branch
```

### `guepard checkout` - Switch Branches

Switch between branches or checkout specific snapshots.

**Note:** Only `--deployment-id` is required. Use it alone to list available branches, or combine it with `--branch-id` or `--snapshot-id` to perform checkout.

#### Syntax
```bash
guepard checkout [OPTIONS]
```

#### Options
| Option | Short | Description | Required |
|--------|-------|-------------|----------|
| `--deployment-id` | `-x` | Deployment ID (only required parameter) | Yes |
| `--branch-id` | `-c` | Branch ID to checkout | No |
| `--snapshot-id` | `-s` | Snapshot ID to checkout | No |
| `--json` | | Output results as JSON | No |

#### Examples

**List available branches (deployment_id only, required):**
```bash
guepard checkout --deployment-id 12345678-1234-1234-1234-123456789abc
```

**Checkout a branch:**
```bash
guepard checkout \
  --deployment-id 12345678-1234-1234-1234-123456789abc \
  --branch-id def67890-1234-5678-9012-345678901234
```

**Restore to a snapshot:**
```bash
guepard checkout \
  --deployment-id 12345678-1234-1234-1234-123456789abc \
  --snapshot-id abc12345-6789-1234-5678-123456789abc
```

**Checkout branch and output as JSON:**
```bash
guepard checkout -x <deployment_id> -c <branch_id> --json
```

**Restore snapshot and output as JSON:**
```bash
guepard checkout -x <deployment_id> -s <snapshot_id> --json
```

**Output format:**
The checkout command displays a table with the following columns:
- **Branch ID**: The unique identifier of the branch
- **Name**: The branch name or label
- **Status**: Current job status of the branch
- **Snapshot ID**: The snapshot associated with the branch
- **Comment**: The snapshot comment/message

### `guepard log` - View Logs

View and monitor deployment logs.

#### Syntax
```bash
guepard log --deployment-id <id> [OPTIONS]
```

#### Options
| Option | Short | Description | Required |
|--------|-------|-------------|----------|
| `--deployment-id` | `-x` | Deployment ID | Yes |
| `--lines` | `-n` | Number of lines to show (default: 50) | No |
| `--follow` | `-f` | Follow logs in real-time | No |
| `--stdout-only` | | Show only stdout logs | No |
| `--stderr-only` | | Show only stderr logs | No |
| `--timestamps` | `-t` | Show timestamps | No |
| `--since` | | Filter logs from date | No |
| `--until` | | Filter logs until date | No |
| `--json` | | Output results as JSON | No |

#### Examples

**View recent logs:**
```bash
guepard log --deployment-id 12345678-1234-1234-1234-123456789abc
```

**Follow logs in real-time:**
```bash
guepard log --deployment-id 12345678-1234-1234-1234-123456789abc --follow
```

**View logs with timestamps:**
```bash
guepard log --deployment-id 12345678-1234-1234-1234-123456789abc --timestamps
```

**Filter logs by date:**
```bash
guepard log \
  --deployment-id 12345678-1234-1234-1234-123456789abc \
  --since "2025-01-08" \
  --until "2025-01-09"
```

## Management Commands

### `guepard compute` - Instance Management

Manage compute instances for your deployments.

#### Syntax
```bash
guepard compute <action> --deployment-id <id>
```

#### Actions
- `status` - Get compute status
- `start` - Start compute instance
- `stop` - Stop compute instance
- `restart` - Restart compute instance
- `logs` - View compute logs
- `list` - List compute details (default)

#### Options
| Option | Short | Description | Required |
|--------|-------|-------------|----------|
| `--deployment-id` | `-x` | Deployment ID | Yes |
| `--json` | | Output results as JSON | No |

#### Examples

**Check compute status:**
```bash
guepard compute status --deployment-id 12345678-1234-1234-1234-123456789abc
```

**Start compute:**
```bash
guepard compute start --deployment-id 12345678-1234-1234-1234-123456789abc
```

**View compute logs:**
```bash
guepard compute logs --deployment-id 12345678-1234-1234-1234-123456789abc
```

**List compute details:**
```bash
guepard compute list --deployment-id 12345678-1234-1234-1234-123456789abc
```

**Get compute status as JSON:**
```bash
guepard compute status --deployment-id <id> --json
```

### `guepard list` - List Resources

List deployments, branches, commits, and other resources.

#### Syntax
```bash
guepard list <resource> [OPTIONS]
```

#### Resources
- `deployments` - List all deployments (default)
- `branches` - List branches for a deployment
- `commits` - List commits for a deployment

#### Options
| Option | Short | Description | Required |
|--------|-------|-------------|----------|
| `--deployment-id` | `-x` | Deployment ID | For branches/commits |
| `--columns` | `-c` | Columns to display | No |
| `--graph` | `-g` | Show git graph style | For commits |
| `--all` | `-a` | Show all commits including AUTO SNAPs | For commits |
| `--json` | | Output results as JSON | No |

#### Examples

**List all deployments:**
```bash
guepard list deployments
```

**List deployments with specific columns:**
```bash
guepard list deployments --columns id,name,status,fqdn
```

**List branches:**
```bash
guepard list branches --deployment-id 12345678-1234-1234-1234-123456789abc
```

**List commits with git graph:**
```bash
guepard list commits \
  --deployment-id 12345678-1234-1234-1234-123456789abc \
  --graph
```

**List deployments as JSON:**
```bash
guepard list deployments --json
```

### `guepard usage` - Usage Information

View your account usage and quotas.

#### Syntax
```bash
guepard usage [OPTIONS]
```

#### Options
| Option | Description | Required |
|--------|-------------|----------|
| `--json` | Output results as JSON | No |

#### Examples

**View usage:**
```bash
guepard usage
```

**View usage as JSON:**
```bash
guepard usage --json
```

**Output:**
```
✅ Usage Summary:
┌─────────────┬───────┬──────┐
│ Resource    │ Quota │ Used │
├─────────────┼───────┼──────┤
│ Deployments │ 10    │ 3    │
│ Snapshots   │ 100   │ 15   │
│ Clones      │ 50    │ 8    │
└─────────────┴───────┴──────┘
```

## Authentication Commands

### `guepard login` - Authentication

Authenticate with your Guepard account.

#### Syntax
```bash
guepard login [OPTIONS]
```

#### Options
| Option | Short | Description | Required |
|--------|-------|-------------|----------|
| `--code` | `-c` | Direct access token input | No |

#### Examples

**Interactive login:**
```bash
guepard login
```

**Direct token login:**
```bash
guepard login --code your-access-token
```

### `guepard logout` - Sign Out

Log out and clear stored credentials.

#### Syntax
```bash
guepard logout
```

#### Example
```bash
guepard logout
```

## Command Aliases and Shortcuts

### Common Aliases
- `guepard` command is the primary binary name
- Use `-x` instead of `--deployment-id`
- Use `-m` instead of `--message`
- Use `-n` instead of `--name`

### Git-like Shortcuts
Some commands support Git-like syntax for familiarity:

```bash
# These show helpful messages
guepard branch develop
guepard commit -m "message"

# Checkout requires explicit flags
guepard checkout -x <deployment_id> -c <branch_id>
```

## Output Formats

### JSON Output

All commands support `--json` flag for machine-readable output:

**Examples:**
```bash
# List deployments as JSON
guepard list deployments --json

# Get deployment details as JSON
guepard deploy --deployment-id <id> --json

# List branches as JSON
guepard branch -x <id> --json

# Get compute status as JSON
guepard compute status -x <id> --json

# View usage as JSON
guepard usage --json
```

JSON output is useful for:
- Scripting and automation
- Integration with other tools
- Parsing with `jq` or similar tools
- CI/CD pipelines

### Beautiful Tables
All commands use colorized, rounded tables for better readability:

```
┌─────────────────────────────────────────────────────────────────┐
│ Deployment ID │ Name    │ Repository │ Provider │ Version │ Status │
├─────────────────────────────────────────────────────────────────┤
│ 12345678...   │ myapp   │ myapp      │ PostgreSQL│ 16     │ active │
└─────────────────────────────────────────────────────────────────┘
```

### Status Indicators
- ✅ Green: Success, active, healthy
- ❌ Red: Error, failed, unhealthy
- ℹ️ Blue: Information, neutral
- 💡 Yellow: Tips, warnings
- 🐆 Cyan: Guepard branding

## Best Practices

### Command Organization
1. **Use interactive mode** for complex operations
2. **Store deployment IDs** in environment variables
3. **Use descriptive commit messages**
4. **Create branches for experiments**
5. **Monitor usage regularly**

### Error Handling
- All commands provide clear error messages
- Use `--help` for command-specific help
- Check authentication with `guepard login`
- Verify deployment IDs with `guepard list deployments`

---

*For more detailed examples, see [Real-World Examples](examples.md) and [Workflows](workflows.md).*
