# Guepard CLI Documentation

Welcome to the comprehensive documentation for Guepard CLI - Git for databases! 🐆

## What is Guepard CLI?

Guepard CLI is a powerful command-line tool that brings Git-like version control capabilities to your databases. It allows you to create snapshots, manage branches, and deploy database instances with the same intuitive workflow you're already familiar with from Git.

## Key Features

- **Git-like Interface**: Familiar commands (`init`, `commit`, `branch`, `checkout`, `log`)
- **Database Version Control**: Create snapshots and manage database states
- **Branch Management**: Create and switch between database branches
- **Multi-Platform Support**: Works on Windows, macOS, and Linux
- **Beautiful Output**: Colorized tables and intuitive command responses
- **Interactive Mode**: Guided setup for complex operations

## Documentation Structure

### Getting Started
- [Installation Guide](installation.md) - Install Guepard CLI on any platform
- [Quick Start](quick-start.md) - Get up and running in minutes
- [Authentication](authentication.md) - Set up your Guepard account

### Core Concepts
- [Understanding Guepard](concepts.md) - How Guepard works with databases
- [Deployments](deployments.md) - Database instances and configurations
- [Branches and Snapshots](branches-snapshots.md) - Version control concepts

### Command Reference
- [Complete Command Reference](commands.md) - All available commands
- [Deploy Commands](deploy-commands.md) - Database deployment operations
- [Branch Commands](branch-commands.md) - Branch management
- [Compute Commands](compute-commands.md) - Instance management
- [Log Commands](log-commands.md) - Monitoring and debugging

### Examples & Use Cases
- [Real-World Examples](examples.md) - Practical scenarios and solutions
- [Development Workflows](workflows.md) - Common development patterns
- [Production Deployments](production.md) - Production-ready setups

### Advanced Topics
- [Performance Profiles](performance.md) - Optimizing database performance
- [API Integration](api-integration.md) - Using Guepard programmatically
- [Troubleshooting](troubleshooting.md) - Common issues and solutions

## Quick Commands

```bash
# Install Guepard CLI
brew install guepard-cli  # macOS
# or download from releases page

# Login to your account
guepard login

# Deploy a database
guepard deploy -p PostgreSQL -v 16 -r us-west -d aws -n myapp -w password

# Create a snapshot
guepard commit -m "Initial schema" -x <deployment_id> -b <branch_id>

# List branches
guepard branch -x <deployment_id>

# Checkout a branch
guepard checkout -x <deployment_id> -c <branch_id>
```

## Support

- **Community**: Join our [Discord](https://discord.gg/nCXAsUd3hm)
- **Issues**: Report bugs on [GitHub](https://github.com/Guepard-Corp/guepard-cli/issues)
- **Documentation**: This comprehensive guide covers everything you need

## Version

Current version: **0.27.17**

---

*Happy coding with Guepard! 🐆*
