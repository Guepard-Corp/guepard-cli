# Understanding Guepard

Learn the core concepts behind Guepard CLI and how it brings Git-like version control to your databases.

## What is Guepard?

Guepard is a revolutionary platform that brings Git-like version control capabilities to databases. Just as Git revolutionized how we manage code, Guepard transforms how we manage database schemas and data.

### The Problem Guepard Solves

Traditional database management has several challenges:

- **No version control**: Database changes are hard to track and revert
- **Difficult collaboration**: Multiple developers can't work on database changes simultaneously
- **Risky deployments**: Database changes often break production systems
- **No rollback mechanism**: Recovering from bad database changes is complex and time-consuming
- **Environment inconsistencies**: Development, staging, and production databases often drift apart

### The Guepard Solution

Guepard solves these problems by providing:

- **Database snapshots**: Create point-in-time captures of your database state
- **Branching**: Work on database changes in parallel without conflicts
- **Easy rollbacks**: Instantly revert to any previous database state
- **Environment consistency**: Keep all environments synchronized
- **Collaborative workflows**: Multiple developers can work on database changes simultaneously

## Core Concepts

### Deployments

A **deployment** is a database instance managed by Guepard. Think of it as a repository for your database, similar to how Git repositories contain your code.

**Deployment characteristics:**
- **Database type**: PostgreSQL, MySQL, or MongoDB
- **Version**: Specific database version (e.g., PostgreSQL 16)
- **Region**: Geographic location (us-west, us-east, eu-west, asia-pacific)
- **Cloud provider**: AWS, GCP, or Azure
- **Performance profile**: Resource allocation and performance characteristics

**Example deployment:**
```bash
guepard deploy \
  --database-provider PostgreSQL \
  --database-version 16 \
  --region us-west \
  --datacenter aws \
  --repository-name my-app-db \
  --database-password secure_password
```

### Snapshots (Commits)

A **snapshot** is a point-in-time capture of your database state. It's similar to a Git commit but for your database.

**Snapshot characteristics:**
- **Immutable**: Once created, snapshots cannot be modified
- **Lightweight**: Only stores changes, not the entire database
- **Branchable**: Can be used as a starting point for new branches
- **Traceable**: Includes metadata like creation time and message

**Creating snapshots:**
```bash
guepard commit \
  --message "Add user authentication tables" \
  --deployment-id <deployment_id> \
  --branch-id <branch_id>
```

### Branches

A **branch** is a parallel line of development for your database. It allows you to experiment with changes without affecting the main database state.

**Branch characteristics:**
- **Independent**: Changes in one branch don't affect others
- **Ephemeral**: Can be temporary for experiments
- **Mergeable**: Changes can be applied to other branches
- **Checkoutable**: You can switch between branches

**Creating branches:**
```bash
guepard branch \
  --deployment-id <deployment_id> \
  --snapshot-id <snapshot_id> \
  feature/new-feature \
  --checkout \
  --ephemeral
```

### Checkout

**Checkout** allows you to switch between different database states. It's similar to `git checkout` but changes your entire database state.

**Checkout operations:**
- **Branch checkout**: Switch to a different branch
- **Snapshot checkout**: Restore to a specific point in time
- **Rollback**: Quickly revert to a previous state

**Checking out:**
```bash
guepard checkout \
  --deployment-id <deployment_id> \
  --branch-id <branch_id>
```

## How Guepard Works

### Database State Management

Guepard manages database states through a sophisticated system:

1. **Initial State**: When you create a deployment, Guepard captures the initial database state
2. **Change Tracking**: As you make changes, Guepard tracks what has changed
3. **Snapshot Creation**: When you commit, Guepard creates a snapshot of the current state
4. **Branch Management**: Branches allow parallel development without conflicts
5. **State Restoration**: Checkout operations restore your database to any previous state

### Version Control Model

Guepard uses a Git-like model adapted for databases:

```
Main Branch:     A---B---C---D
                      \
Feature Branch:        E---F---G
```

- **A, B, C, D**: Snapshots on the main branch
- **E, F, G**: Snapshots on a feature branch
- **B**: Branch point where feature branch diverged

### Storage and Performance

Guepard optimizes storage and performance through:

- **Incremental snapshots**: Only stores changes, not full database copies
- **Compression**: Efficient storage of database states
- **Caching**: Fast access to frequently used snapshots
- **Deduplication**: Shared data between snapshots is stored only once

## Use Cases

### Development Workflows

**Feature Development:**
```bash
# Create feature branch
guepard branch -x <deployment_id> -s <snapshot_id> feature/auth -k -e

# Make changes
guepard commit -m "Add user table" -x <deployment_id> -b <branch_id>
guepard commit -m "Add authentication logic" -x <deployment_id> -b <branch_id>

# Merge back to main
guepard checkout -x <deployment_id> -c main-branch-id
guepard commit -m "Merge auth feature" -x <deployment_id> -b main-branch-id
```

**Experimenting:**
```bash
# Create experimental branch
guepard branch -x <deployment_id> -s <snapshot_id> experiment/new-arch -e

# Try different approaches
guepard commit -m "Try microservices approach" -x <deployment_id> -b <branch_id>

# If successful, merge; if not, discard
```

### Production Management

**Safe Deployments:**
```bash
# Create pre-deployment snapshot
guepard commit -m "Pre-deployment snapshot" -x <deployment_id> -b <branch_id>

# Deploy changes
guepard commit -m "Deploy new features" -x <deployment_id> -b <branch_id>

# If issues occur, rollback
guepard checkout -x <deployment_id> -s <previous_snapshot_id>
```

**Disaster Recovery:**
```bash
# Create backup deployment
guepard deploy -p PostgreSQL -v 16 -r us-east -d aws -n app-backup -w password

# Regular backups
guepard commit -m "Daily backup" -x <backup_deployment_id> -b <branch_id>

# Disaster recovery
guepard checkout -x <prod_deployment_id> -s <backup_snapshot_id>
```

### Team Collaboration

**Parallel Development:**
```bash
# Developer A works on feature X
guepard branch -x <deployment_id> -s <snapshot_id> feature/user-profiles -k -e

# Developer B works on feature Y
guepard branch -x <deployment_id> -s <snapshot_id> feature/payments -k -e

# Both can work independently without conflicts
```

**Code Review:**
```bash
# Reviewer checks out feature branch
guepard checkout -x <deployment_id> -c <feature_branch_id>

# Tests the changes
# If approved, merges to main
guepard checkout -x <deployment_id> -c main-branch-id
guepard commit -m "Merge approved feature" -x <deployment_id> -b main-branch-id
```

## Benefits of Guepard

### For Developers

- **Familiar workflow**: Git-like commands you already know
- **Safe experimentation**: Try changes without risk
- **Easy rollbacks**: Quickly revert problematic changes
- **Parallel development**: Work on multiple features simultaneously
- **Environment consistency**: Keep all environments in sync

### For Teams

- **Better collaboration**: Multiple developers can work on database changes
- **Code review process**: Review database changes like code changes
- **Reduced conflicts**: Parallel development without interference
- **Knowledge sharing**: Clear history of database changes
- **Standardized workflows**: Consistent processes across the team

### For Organizations

- **Reduced risk**: Safe database changes with easy rollbacks
- **Faster development**: Parallel development and experimentation
- **Better compliance**: Complete audit trail of database changes
- **Cost savings**: Fewer production issues and faster recovery
- **Scalable processes**: Workflows that grow with your team

## Getting Started

### Prerequisites

- **Guepard account**: Sign up at [guepard.run](https://guepard.run)
- **Guepard CLI**: Install following the [Installation Guide](installation.md)
- **Database knowledge**: Basic understanding of your chosen database type

### First Steps

1. **Authenticate**: `guepard login`
2. **Create deployment**: `guepard deploy --interactive`
3. **Create snapshot**: `guepard commit -m "Initial state" -x <deployment_id> -b <branch_id>`
4. **Create branch**: `guepard branch -x <deployment_id> -s <snapshot_id> develop -k -e`
5. **Start developing**: Make changes and create snapshots

### Learning Path

1. **Start with basics**: Follow the [Quick Start Guide](quick-start.md)
2. **Learn commands**: Study the [Command Reference](commands.md)
3. **See examples**: Explore [Real-World Examples](examples.md)
4. **Master workflows**: Practice with [Workflow Guides](workflows.md)
5. **Troubleshoot issues**: Use the [Troubleshooting Guide](troubleshooting.md)

## Advanced Concepts

### Performance Profiles

Guepard offers different performance profiles to optimize your database for specific use cases:

- **Development**: Lower resources, cost-effective
- **Testing**: Balanced performance for testing
- **Production**: High performance and reliability
- **Analytics**: Optimized for read-heavy workloads

### Multi-Region Deployments

Deploy databases in different regions for:
- **Latency optimization**: Closer to your users
- **Disaster recovery**: Geographic redundancy
- **Compliance**: Data residency requirements
- **Performance**: Regional performance characteristics

### Integration Patterns

Guepard integrates with your existing workflows:

- **CI/CD pipelines**: Automated deployments and testing
- **Monitoring systems**: Health checks and alerting
- **Backup systems**: Integration with existing backup solutions
- **Development tools**: IDE integration and development workflows

## Best Practices

### Snapshot Management

- **Frequent snapshots**: Create snapshots often to enable easy rollbacks
- **Descriptive messages**: Use clear commit messages that explain changes
- **Atomic changes**: Each snapshot should represent one logical change
- **Regular cleanup**: Remove old snapshots to manage storage

### Branch Strategy

- **Feature branches**: Create branches for each feature or bug fix
- **Ephemeral branches**: Use ephemeral branches for experiments
- **Consistent naming**: Use consistent naming conventions for branches
- **Regular merging**: Merge completed features back to main regularly

### Environment Management

- **Environment parity**: Keep all environments synchronized
- **Staging validation**: Always test changes in staging before production
- **Production snapshots**: Create snapshots before major production changes
- **Rollback planning**: Always have a rollback plan ready

---

*Now that you understand Guepard's core concepts, you're ready to start using it effectively! Check out the [Quick Start Guide](quick-start.md) to get hands-on experience. 🐆*
