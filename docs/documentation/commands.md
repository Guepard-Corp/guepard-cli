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
| `--interactive` | `-I` | Interactive mode | No |

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
| `--ephemeral` | `-e` | Create ephemeral branch | No |
| `--source-branch-id` | `-b` | Source branch ID | No |
| `--discard-changes` | `-d` | Discard changes | No |

#### Examples

**List branches:**
```bash
guepard branch --deployment-id 12345678-1234-1234-1234-123456789abc
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

**Git-like usage:**
```bash
# Shows helpful message
guepard branch feature-auth
guepard branch
```

### `guepard checkout` - Switch Branches

Switch between branches or checkout specific snapshots.

#### Syntax
```bash
guepard checkout [OPTIONS] <target>
```

#### Options
| Option | Short | Description | Required |
|--------|-------|-------------|----------|
| `--deployment-id` | `-x` | Deployment ID | Yes |
| `--branch-id` | `-c` | Branch ID to checkout | For branch checkout |
| `--snapshot-id` | `-s` | Snapshot ID to checkout | For snapshot checkout |
| `--checkout` | `-k` | Perform checkout | No |
| `--discard-changes` | `-d` | Discard changes | No |

#### Examples

**Checkout a branch:**
```bash
guepard checkout \
  --deployment-id 12345678-1234-1234-1234-123456789abc \
  --branch-id def67890-1234-5678-9012-345678901234
```

**List available branches:**
```bash
guepard checkout --deployment-id 12345678-1234-1234-1234-123456789abc
```

**Git-like usage:**
```bash
# Shows helpful message
guepard checkout develop
```

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

### `guepard usage` - Usage Information

View your account usage and quotas.

#### Syntax
```bash
guepard usage
```

#### Example
```bash
guepard usage
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
Many commands support Git-like syntax for familiarity:

```bash
# These show helpful messages
guepard branch develop
guepard checkout develop
guepard commit -m "message"
```

## Output Formats

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
