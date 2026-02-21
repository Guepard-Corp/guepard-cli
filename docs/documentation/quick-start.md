# Quick Start Guide

Get up and running with Guepard CLI in just a few minutes! This guide will walk you through the essential steps to create your first database deployment and start using version control.

## Prerequisites

- Guepard CLI installed (see [Installation Guide](installation.md))
- Internet connection
- Guepard account (sign up at [guepard.run](https://guepard.run))

## Step 1: Authentication

First, authenticate with your Guepard account:

```bash
guepard login
```

This will:
1. Open your browser to the Guepard login page
2. Prompt you to enter a verification code
3. Save your authentication token for future use

**Example output:**
```
🐆 Starting login process...
✅ Login URL generated successfully!
🔗 URL: https://guepard.run/auth/login?code=abc123
Press Enter to open the login page in your browser... 
Enter the verification code from the webpage: 123456
✅ Completing login...
✅ Login successful. Happy coding! 🐆
```

## Step 2: Create Your First Deployment

### Option A: Interactive Mode (Recommended)

Use interactive mode for a guided setup:

```bash
guepard deploy --interactive
```

This will walk you through each configuration step with helpful prompts and defaults.

### Option B: Command Line

Create a deployment directly with command-line flags:

```bash
guepard deploy \
  --database-provider PostgreSQL \
  --database-version 16 \
  --region us-west \
  --instance-type REPOSITORY \
  --datacenter aws \
  --repository-name my-first-app \
  --database-password mypassword123
```

**Example output:**
```
✅ Deployment created successfully!

📋 Deployment Details
  🆔 ID: 12345678-1234-1234-1234-123456789abc
  📝 Name: my-first-app
  📁 Repository: my-first-app
  🗄️ Provider: PostgreSQL
  🔢 Version: 16
  ✅ Status: active
  🌐 FQDN: my-first-app.guepard.run
  🌍 Region: us-west
  🏢 Datacenter: aws
  📅 Created: 2025-01-08T10:30:00Z

🔗 Database Connection
  🏠 Host: my-first-app.guepard.run
  🔌 Port: 5432
  🗃️ Database: my-first-app
  👤 Username: guepard
  🔐 Password: mypassword123

💡 Ready-to-use Connection URI:
postgresql://guepard:mypassword123@my-first-app.guepard.run:5432/my-first-app

📝 Connect with psql:
$ psql 'postgresql://guepard:mypassword123@my-first-app.guepard.run:5432/my-first-app'
```

## Step 3: Create Your First Snapshot

Now let's create a snapshot (like a Git commit) of your database:

```bash
guepard commit \
  --message "Initial database setup" \
  --deployment-id 12345678-1234-1234-1234-123456789abc \
  --branch-id a7d373a3-4244-47b7-aacb-ad366f2520f6
```

**Example output:**
```
✅ Created commit successfully!
┌─────────────────────────────────────────────────────────────────┐
│ Commit ID    │ Message                │ Status │ Created         │
├─────────────────────────────────────────────────────────────────┤
│ abc12345...  │ Initial database setup │ active │ 2025-01-08T...  │
└─────────────────────────────────────────────────────────────────┘
```

## Step 4: Explore Your Deployment

### List Deployments
```bash
guepard list deployments
```

### View Deployment Details
```bash
guepard deploy --deployment-id 12345678-1234-1234-1234-123456789abc
```

### Check Compute Status
```bash
guepard compute status --deployment-id 12345678-1234-1234-1234-123456789abc
```

## Step 5: Create Branches

Create a development branch for experimentation:

```bash
guepard branch \
  --deployment-id 12345678-1234-1234-1234-123456789abc \
  --snapshot-id abc12345-6789-1234-5678-123456789abc \
  develop \
  --checkout \
  --ephemeral
```

**Example output:**
```
✅ Branch created successfully!
┌─────────────────────────────────────────────────────────────────┐
│ Branch ID   │ Name    │ Status │ Snapshot ID │ Environment │ Ephemeral │
├─────────────────────────────────────────────────────────────────┤
│ def67890... │ develop │ active │ abc12345... │ development│ Yes       │
└─────────────────────────────────────────────────────────────────┘
```

## Step 6: Switch Between Branches

List available branches:
```bash
guepard branch --deployment-id 12345678-1234-1234-1234-123456789abc
```

Checkout a different branch:
```bash
guepard checkout \
  --deployment-id 12345678-1234-1234-1234-123456789abc \
  --branch-id def67890-1234-5678-9012-345678901234
```

## Step 7: View History

See your commit history:
```bash
guepard log --deployment-id 12345678-1234-1234-1234-123456789abc
```

For a Git-style graph view:
```bash
guepard list commits --deployment-id 12345678-1234-1234-1234-123456789abc --graph
```

## Step 8: Monitor Your Database

### View Logs
```bash
guepard log --deployment-id 12345678-1234-1234-1234-123456789abc
```

### Follow Logs in Real-time
```bash
guepard log --deployment-id 12345678-1234-1234-1234-123456789abc --follow
```

### Check Usage
```bash
guepard usage
```

## Common Workflows

### Development Workflow
```bash
# 1. Create a feature branch
guepard branch -x <deployment_id> -s <snapshot_id> feature-auth -k -e

# 2. Make changes to your database (via your application)
# 3. Create a snapshot
guepard commit -m "Add user authentication tables" -x <deployment_id> -b <branch_id>

# 4. Switch back to main
guepard checkout -x <deployment_id> -c <main_branch_id>
```

### Production Deployment
```bash
# 1. Create production branch from stable snapshot
guepard branch -x <deployment_id> -s <stable_snapshot_id> production

# 2. Deploy to production environment
guepard compute start -x <deployment_id>

# 3. Monitor logs
guepard log -x <deployment_id> --follow
```

## Next Steps

Now that you have the basics down:

1. **Explore [Command Reference](commands.md)** - Learn all available commands
2. **Read [Real-World Examples](examples.md)** - See practical use cases
3. **Check out [Workflows](workflows.md)** - Advanced development patterns
4. **Join our [Community](https://discord.gg/NYsNzQGvZT)** - Get help and share experiences

## Tips for Success

- **Use descriptive commit messages**: Like Git, good commit messages help track changes
- **Create branches for experiments**: Use ephemeral branches for testing
- **Monitor your usage**: Check `guepard usage` regularly
- **Keep snapshots frequent**: Create commits often to track changes
- **Use interactive mode**: When in doubt, use `--interactive` flags

## Troubleshooting

If you run into issues:

1. **Check authentication**: Run `guepard login` if you get auth errors
2. **Verify deployment ID**: Use `guepard list deployments` to find correct IDs
3. **Check network**: Ensure you have internet connectivity
4. **Review logs**: Use `guepard log` to see what's happening
5. **Get help**: Join our [Discord](https://discord.gg/NYsNzQGvZT) community

---

*Congratulations! You're now ready to use Guepard CLI like a pro! 🐆*
