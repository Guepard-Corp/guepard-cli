# Branches and Snapshots

Master the core version control concepts of Guepard CLI: branches and snapshots.

## Understanding Snapshots

A **snapshot** is a point-in-time capture of your database state. It's similar to a Git commit but for your database, allowing you to save and restore your database to any previous state.

### What Snapshots Capture

Snapshots capture:
- **Schema changes**: Table structures, indexes, constraints
- **Data changes**: All data modifications since the last snapshot
- **Configuration**: Database settings and parameters
- **Metadata**: Creation time, message, and branch information

### Creating Snapshots

```bash
guepard commit \
  --message "Add user authentication tables" \
  --deployment-id <deployment_id> \
  --branch-id <branch_id>
```

**Example output:**
```
✅ Created commit successfully!
┌─────────────────────────────────────────────────────────────────┐
│ Commit ID    │ Message                │ Status │ Created         │
├─────────────────────────────────────────────────────────────────┤
│ abc12345...  │ Add user authentication│ active │ 2025-01-08T...  │
└─────────────────────────────────────────────────────────────────┘
```

### Snapshot Best Practices

**Descriptive Messages:**
```bash
# Good
guepard commit -m "Add user table with email and password fields"
guepard commit -m "Fix foreign key constraint on orders table"
guepard commit -m "Add indexes for performance optimization"

# Bad
guepard commit -m "fix"
guepard commit -m "update"
guepard commit -m "changes"
```

**Frequent Snapshots:**
```bash
# Create snapshots often for easy rollbacks
guepard commit -m "Initial schema setup" -x <deployment_id> -b <branch_id>
guepard commit -m "Add user management" -x <deployment_id> -b <branch_id>
guepard commit -m "Add product catalog" -x <deployment_id> -b <branch_id>
guepard commit -m "Add order processing" -x <deployment_id> -b <branch_id>
```

**Atomic Changes:**
```bash
# Each snapshot should represent one logical change
guepard commit -m "Add users table" -x <deployment_id> -b <branch_id>
guepard commit -m "Add products table" -x <deployment_id> -b <branch_id>
guepard commit -m "Add orders table" -x <deployment_id> -b <branch_id>

# Not: "Add users, products, and orders tables"
```

## Understanding Branches

A **branch** is a parallel line of development for your database. It allows you to experiment with changes without affecting the main database state.

### Branch Concepts

**Branch characteristics:**
- **Independent**: Changes in one branch don't affect others
- **Ephemeral**: Can be temporary for experiments
- **Mergeable**: Changes can be applied to other branches
- **Checkoutable**: You can switch between branches

### Creating Branches

```bash
guepard branch \
  --deployment-id <deployment_id> \
  --snapshot-id <snapshot_id> \
  feature/user-authentication \
  --checkout \
  --ephemeral
```

**Parameters:**
- `--deployment-id`: The deployment to create the branch in
- `--snapshot-id`: The snapshot to branch from
- Branch name is **positional** (no `-n`): pass the name after the options
- `--checkout`: Automatically checkout the new branch
- `--ephemeral`: Mark as temporary (can be cleaned up later)

### Branch Types

#### Feature Branches
For developing new features:

```bash
guepard branch \
  --deployment-id <deployment_id> \
  --snapshot-id <snapshot_id> \
  feature/payment-integration \
  --checkout \
  --ephemeral
```

#### Bug Fix Branches
For fixing issues:

```bash
guepard branch \
  --deployment-id <deployment_id> \
  --snapshot-id <snapshot_id> \
  bugfix/login-error \
  --checkout \
  --ephemeral
```

#### Hotfix Branches
For urgent production fixes:

```bash
guepard branch \
  --deployment-id <deployment_id> \
  --snapshot-id <production_snapshot_id> \
  hotfix/security-patch \
  --checkout \
  --ephemeral
```

#### Experimental Branches
For testing new ideas:

```bash
guepard branch \
  --deployment-id <deployment_id> \
  --snapshot-id <snapshot_id> \
  experiment/new-architecture \
  --checkout \
  --ephemeral
```

### Branch Naming Conventions

Establish consistent naming for your team:

```bash
# Feature branches
feature/user-authentication
feature/payment-integration
feature/admin-dashboard

# Bug fix branches
bugfix/login-error
bugfix/payment-timeout
bugfix/data-validation

# Hotfix branches
hotfix/security-patch
hotfix/critical-bug

# Release branches
release/v1.2.0
release/v2.0.0-beta

# Experimental branches
experiment/new-database-design
experiment/performance-optimization
```

## Branch Management

### Listing Branches

```bash
guepard branch --deployment-id <deployment_id>
```

**Output:**
```
✅ Found 4 branches for deployment: 12345678-1234-1234-1234-123456789abc
┌─────────────────────────────────────────────────────────────────┐
│ Branch ID   │ Name              │ Status │ Snapshot ID │ Environment │ Ephemeral │
├─────────────────────────────────────────────────────────────────┤
│ a7d373a3... │ main             │ active │ abc12345... │ development │ No        │
│ def67890... │ feature/auth     │ active │ def67890... │ development │ Yes       │
│ ghi90123... │ feature/payments │ active │ ghi90123... │ development │ Yes       │
│ jkl45678... │ bugfix/login     │ active │ jkl45678... │ development │ Yes       │
└─────────────────────────────────────────────────────────────────┘
```

### Checking Out Branches

```bash
guepard checkout \
  --deployment-id <deployment_id> \
  --branch-id <branch_id>
```

**Example:**
```bash
guepard checkout \
  --deployment-id 12345678-1234-1234-1234-123456789abc \
  --branch-id def67890-1234-5678-9012-345678901234
```

### Switching Between Branches

```bash
# List available branches
guepard branch --deployment-id <deployment_id>

# Switch to a different branch
guepard checkout \
  --deployment-id <deployment_id> \
  --branch-id <target_branch_id>

# Your database state changes to match the target branch
```

## Snapshot Management

### Viewing Snapshot History

```bash
guepard list commits --deployment-id <deployment_id>
```

**Output:**
```
✅ Found 5 commits for deployment: 12345678-1234-1234-1234-123456789abc
┌─────────────────────────────────────────────────────────────────┐
│ ID            │ Name    │ Message                │ Created       │ Dataset ID │ Parent ID │
├─────────────────────────────────────────────────────────────────┤
│ abc12345...   │ commit1 │ Initial schema setup   │ 2025-01-08... │ a7d373a3... │ null      │
│ def67890...   │ commit2 │ Add user management    │ 2025-01-08... │ a7d373a3... │ abc12345... │
│ ghi90123...   │ commit3 │ Add product catalog    │ 2025-01-08... │ a7d373a3... │ def67890... │
│ jkl45678...   │ commit4 │ Add order processing   │ 2025-01-08... │ a7d373a3... │ ghi90123... │
│ mno78901...   │ commit5 │ Add payment system     │ 2025-01-08... │ a7d373a3... │ jkl45678... │
└─────────────────────────────────────────────────────────────────┘
```

### Git-Style Graph View

```bash
guepard list commits --deployment-id <deployment_id> --graph
```

**Output:**
```
✅ Found 5 commits for deployment: 12345678-1234-1234-1234-123456789abc

* abc12345 (main) 📝 Initial schema setup                    2025-01-08
| * def67890 (feature/auth) 📝 Add user authentication       2025-01-08
| * ghi90123 (feature/payments) 📝 Add payment system       2025-01-08
* jkl45678 (main) 📝 Add product catalog                     2025-01-08
* mno78901 (main) 📝 Add order processing                   2025-01-08
```

### Checking Out Snapshots

```bash
guepard checkout \
  --deployment-id <deployment_id> \
  --snapshot-id <snapshot_id>
```

**Example:**
```bash
# Restore to a previous state
guepard checkout \
  --deployment-id 12345678-1234-1234-1234-123456789abc \
  --snapshot-id abc12345-6789-1234-5678-123456789abc
```

## Workflow Examples

### Feature Development Workflow

```bash
# 1. Start from main branch
guepard checkout --deployment-id <deployment_id> --branch-id main-branch-id

# 2. Create feature branch
guepard branch \
  --deployment-id <deployment_id> \
  --snapshot-id latest-main-snapshot-id \
  feature/user-profiles \
  --checkout \
  --ephemeral

# 3. Develop feature (make database changes via your application)
# Add tables, modify schema, etc.

# 4. Create intermediate commits
guepard commit \
  --message "Add user profiles table" \
  --deployment-id <deployment_id> \
  --branch-id feature/user-profiles-branch-id

# 5. Continue development
guepard commit \
  --message "Add profile images support" \
  --deployment-id <deployment_id> \
  --branch-id feature/user-profiles-branch-id

# 6. Finalize feature
guepard commit \
  --message "Complete user profiles feature" \
  --deployment-id <deployment_id> \
  --branch-id feature/user-profiles-branch-id

# 7. Merge back to main (conceptually)
guepard checkout --deployment-id <deployment_id> --branch-id main-branch-id
guepard commit \
  --message "Merge user profiles feature" \
  --deployment-id <deployment_id> \
  --branch-id main-branch-id
```

### Bug Fix Workflow

```bash
# 1. Create bug fix branch from main
guepard branch \
  --deployment-id <deployment_id> \
  --snapshot-id latest-main-snapshot-id \
  bugfix/login-error \
  --checkout \
  --ephemeral

# 2. Apply fix
guepard commit \
  --message "Fix login validation error" \
  --deployment-id <deployment_id> \
  --branch-id bugfix/login-error-branch-id

# 3. Test fix
# Run tests to verify the fix works

# 4. Merge to main
guepard checkout --deployment-id <deployment_id> --branch-id main-branch-id
guepard commit \
  --message "Merge bug fix: login validation" \
  --deployment-id <deployment_id> \
  --branch-id main-branch-id
```

### Hotfix Workflow

```bash
# 1. Create hotfix branch from production snapshot
guepard branch \
  --deployment-id <deployment_id> \
  --snapshot-id production-snapshot-id \
  hotfix/security-patch \
  --checkout \
  --ephemeral

# 2. Apply urgent fix
guepard commit \
  --message "Fix critical security vulnerability" \
  --deployment-id <deployment_id> \
  --branch-id hotfix/security-patch-branch-id

# 3. Deploy to production immediately
guepard checkout --deployment-id <deployment_id> --branch-id main-branch-id
guepard commit \
  --message "Deploy hotfix: security patch" \
  --deployment-id <deployment_id> \
  --branch-id main-branch-id

# 4. Merge back to develop
guepard checkout --deployment-id <deployment_id> --branch-id develop-branch-id
guepard commit \
  --message "Merge hotfix to develop" \
  --deployment-id <deployment_id> \
  --branch-id develop-branch-id
```

### Experimental Development

```bash
# 1. Create experimental branch
guepard branch \
  --deployment-id <deployment_id> \
  --snapshot-id latest-snapshot-id \
  experiment/new-architecture \
  --checkout \
  --ephemeral

# 2. Try different approaches
guepard commit \
  --message "Experiment: try microservices approach" \
  --deployment-id <deployment_id> \
  --branch-id experiment/new-architecture-branch-id

# 3. If successful, merge to feature branch
# If not successful, simply discard the branch
```

## Advanced Branch Operations

### Branch from Specific Snapshot

```bash
# Create branch from a specific point in history
guepard branch \
  --deployment-id <deployment_id> \
  --snapshot-id <specific_snapshot_id> \
  feature/from-specific-point \
  --checkout
```

### Multiple Branch Management

```bash
# Create multiple branches for parallel development
guepard branch -x <deployment_id> -s <snapshot_id> feature/auth -k -e
guepard branch -x <deployment_id> -s <snapshot_id> feature/payments -k -e
guepard branch -x <deployment_id> -s <snapshot_id> feature/admin -k -e

# Each developer can work on their branch independently
```

### Branch Cleanup

```bash
# List all branches to see what can be cleaned up
guepard branch --deployment-id <deployment_id>

# Ephemeral branches can be safely removed after merging
# Regular branches should be kept for reference
```

## Best Practices

### Snapshot Best Practices

1. **Frequent snapshots**: Create snapshots often to enable easy rollbacks
2. **Descriptive messages**: Use clear commit messages that explain changes
3. **Atomic changes**: Each snapshot should represent one logical change
4. **Regular cleanup**: Remove old snapshots to manage storage

### Branch Best Practices

1. **Feature branches**: Create branches for each feature or bug fix
2. **Ephemeral branches**: Use ephemeral branches for experiments
3. **Consistent naming**: Use consistent naming conventions for branches
4. **Regular merging**: Merge completed features back to main regularly

### Workflow Best Practices

1. **Start from main**: Always branch from the latest main snapshot
2. **Test before merging**: Verify changes work before merging to main
3. **Keep branches small**: Avoid long-running branches that diverge significantly
4. **Document changes**: Use descriptive commit messages and branch names

## Troubleshooting

### Common Issues

**Branch not found:**
```bash
# List available branches
guepard branch --deployment-id <deployment_id>

# Verify branch ID
guepard checkout --deployment-id <deployment_id> --branch-id <correct_branch_id>
```

**Snapshot not found:**
```bash
# List available snapshots
guepard list commits --deployment-id <deployment_id>

# Verify snapshot ID
guepard branch --deployment-id <deployment_id> --snapshot-id <correct_snapshot_id> test-branch
```

**Cannot checkout:**
```bash
# Check if you're authenticated
guepard login

# Verify deployment ID
guepard deploy --deployment-id <deployment_id>
```

### Getting Help

If you encounter issues:

1. **Check command syntax**: `guepard <command> --help`
2. **Verify parameters**: Ensure all required parameters are provided
3. **Check authentication**: Run `guepard login` if needed
4. **Community support**: Join our [Discord](https://discord.gg/NYsNzQGvZT)
5. **Report issues**: Create an issue on [GitHub](https://github.com/Guepard-Corp/guepard-cli/issues)

---

*Now that you understand branches and snapshots, explore [Real-World Examples](examples.md) to see these concepts in action! 🐆*
