# Troubleshooting and FAQ

Get help with common issues and find answers to frequently asked questions about Guepard CLI.

## Table of Contents

- [Common Issues](#common-issues)
- [Authentication Problems](#authentication-problems)
- [Deployment Issues](#deployment-issues)
- [Command Errors](#command-errors)
- [Performance Issues](#performance-issues)
- [Network Problems](#network-problems)
- [Platform-Specific Issues](#platform-specific-issues)
- [FAQ](#faq)
- [Getting Help](#getting-help)

## Common Issues

### Command Not Found

**Problem**: `guepard: command not found`

**Solutions**:

1. **Check installation**:
   ```bash
   which guepard
   # or
   which guepard
   ```

2. **Add to PATH**:
   ```bash
   # macOS/Linux
   export PATH="/usr/local/bin:$PATH"
   
   # Windows
   # Add C:\path\to\guepard to your PATH environment variable
   ```

3. **Reinstall**:
   ```bash
   # macOS
   brew reinstall guepard
   
   # Or download from releases page
   ```

### Permission Denied

**Problem**: `Permission denied` when running commands

**Solutions**:

1. **Make executable**:
   ```bash
   chmod +x guepard
   ```

2. **Use sudo** (not recommended for regular use):
   ```bash
   sudo guepard --version
   ```

3. **Check file permissions**:
   ```bash
   ls -la guepard
   ```

### Version Mismatch

**Problem**: Commands fail with version-related errors

**Solutions**:

1. **Check version**:
   ```bash
   guepard --version
   ```

2. **Update to latest version**:
   ```bash
   # macOS
   brew upgrade guepard
   
   # Or download latest from releases page
   ```

3. **Check compatibility**:
   - Ensure you're using a supported version
   - Check the [releases page](https://github.com/Guepard-Corp/guepard-cli/releases) for latest version

## Authentication Problems

### Authentication Required Error

**Problem**: `❌ Authentication required. Please run 'guepard login' first.`

**Solutions**:

1. **Login**:
   ```bash
   guepard login
   ```

2. **Check token**:
   ```bash
   # macOS/Linux
   ls -la ~/.config/guepard/token
   
   # Windows
   dir %APPDATA%\guepard\token
   ```

3. **Clear and re-login**:
   ```bash
   guepard logout
   guepard login
   ```

### Invalid Token Error

**Problem**: `❌ Invalid or expired token. Please run 'guepard login' again.`

**Solutions**:

1. **Re-authenticate**:
   ```bash
   guepard logout
   guepard login
   ```

2. **Check token file**:
   ```bash
   # macOS/Linux
   cat ~/.config/guepard/token
   
   # Windows
   type %APPDATA%\guepard\token
   ```

3. **Manual token input**:
   ```bash
   guepard login --code your-access-token
   ```

### Browser Doesn't Open

**Problem**: Browser doesn't open automatically during login

**Solutions**:

1. **Manual URL access**:
   - Copy the URL from the terminal
   - Paste it into your browser manually
   - Complete the login process

2. **Check browser settings**:
   - Ensure default browser is set
   - Check if browser is blocked by security software

3. **Use direct token login**:
   ```bash
   guepard login --code your-access-token
   ```

## Deployment Issues

### Deployment Creation Fails

**Problem**: `guepard deploy` fails with errors

**Solutions**:

1. **Check required parameters**:
   ```bash
   guepard deploy --help
   ```

2. **Use interactive mode**:
   ```bash
   guepard deploy --interactive
   ```

3. **Verify authentication**:
   ```bash
   guepard login
   ```

4. **Check network connection**:
   ```bash
   ping api.guepard.run
   ```

### Deployment Not Found

**Problem**: `❌ Deployment not found` when using deployment ID

**Solutions**:

1. **List deployments**:
   ```bash
   guepard list deployments
   ```

2. **Check deployment ID**:
   ```bash
   # Verify the ID is correct
   guepard deploy -x <deployment_id>
   ```

3. **Check permissions**:
   - Ensure you have access to the deployment
   - Verify you're logged in with the correct account

### Compute Issues

**Problem**: Compute instance won't start or stops unexpectedly

**Solutions**:

1. **Check status**:
   ```bash
   guepard compute status -x <deployment_id>
   ```

2. **View logs**:
   ```bash
   guepard log -x <deployment_id>
   ```

3. **Restart compute**:
   ```bash
   guepard compute restart -x <deployment_id>
   ```

4. **Check usage limits**:
   ```bash
   guepard usage
   ```

## Command Errors

### Invalid Arguments

**Problem**: `error: invalid value for argument`

**Solutions**:

1. **Check command syntax**:
   ```bash
   guepard <command> --help
   ```

2. **Verify required parameters**:
   ```bash
   # Example: commit requires message, deployment-id, and branch-id
   guepard commit -m "message" -x <deployment_id> -b <branch_id>
   ```

3. **Use correct format**:
   ```bash
   # Correct
   guepard deploy -p PostgreSQL -v 16
   
   # Incorrect
   guepard deploy --database-provider PostgreSQL --database-version 16
   ```

### Branch Not Found

**Problem**: `❌ Branch not found` when checking out

**Solutions**:

1. **List available branches**:
   ```bash
   guepard branch -x <deployment_id>
   ```

2. **Check branch ID**:
   ```bash
   # Verify the branch ID is correct
   guepard checkout -x <deployment_id> -c <branch_id>
   ```

3. **Create branch first**:
   ```bash
   guepard branch -x <deployment_id> -s <snapshot_id> <branch_name>
   ```

### Snapshot Not Found

**Problem**: `❌ Snapshot not found` when creating branches

**Solutions**:

1. **List commits/snapshots**:
   ```bash
   guepard list commits -x <deployment_id>
   ```

2. **Check snapshot ID**:
   ```bash
   # Verify the snapshot ID is correct
   guepard branch -x <deployment_id> -s <snapshot_id> <branch_name>
   ```

3. **Create snapshot first**:
   ```bash
   guepard commit -m "message" -x <deployment_id> -b <branch_id>
   ```

## Performance Issues

### Slow Commands

**Problem**: Commands take a long time to execute

**Solutions**:

1. **Check network connection**:
   ```bash
   ping api.guepard.run
   ```

2. **Check system resources**:
   ```bash
   # macOS/Linux
   top
   
   # Windows
   taskmgr
   ```

3. **Use smaller operations**:
   ```bash
   # Instead of large operations, break them down
   guepard log -x <deployment_id> --lines 10
   ```

### High Memory Usage

**Problem**: Guepard CLI uses too much memory

**Solutions**:

1. **Check system resources**:
   ```bash
   # macOS/Linux
   ps aux | grep guepard
   
   # Windows
   tasklist | findstr guepard
   ```

2. **Restart CLI**:
   ```bash
   # Close and reopen terminal
   # Or kill the process and restart
   ```

3. **Use smaller operations**:
   ```bash
   # Limit log output
   guepard log -x <deployment_id> --lines 50
   ```

## Network Problems

### Connection Timeout

**Problem**: `Connection timeout` or network errors

**Solutions**:

1. **Check internet connection**:
   ```bash
   ping google.com
   ```

2. **Check firewall settings**:
   - Ensure Guepard domains are not blocked
   - Check corporate firewall rules

3. **Try different network**:
   - Switch to different WiFi
   - Use mobile hotspot

4. **Check DNS**:
   ```bash
   nslookup api.guepard.run
   ```

### SSL/TLS Errors

**Problem**: SSL certificate errors

**Solutions**:

1. **Check system time**:
   ```bash
   date
   ```

2. **Update certificates**:
   ```bash
   # macOS
   sudo update-ca-certificates
   
   # Linux
   sudo apt-get update && sudo apt-get install ca-certificates
   ```

3. **Check corporate proxy**:
   - Configure proxy settings if behind corporate firewall
   - Contact IT department for assistance

## Platform-Specific Issues

### macOS Issues

**Problem**: Security warnings or permission issues

**Solutions**:

1. **Allow in Security & Privacy**:
   - Go to System Preferences > Security & Privacy
   - Click "Allow" for Guepard CLI

2. **Check Gatekeeper**:
   ```bash
   # Allow Guepard CLI
   sudo spctl --add /usr/local/bin/guepard
   ```

3. **Check permissions**:
   ```bash
   ls -la /usr/local/bin/guepard
   ```

### Windows Issues

**Problem**: Antivirus blocking or Windows Defender warnings

**Solutions**:

1. **Add to exclusions**:
   - Add Guepard CLI to antivirus exclusions
   - Add to Windows Defender exclusions

2. **Run as administrator**:
   ```cmd
   # Right-click Command Prompt > Run as administrator
   guepard --version
   ```

3. **Check PATH**:
   ```cmd
   echo %PATH%
   ```

### Linux Issues

**Problem**: Missing dependencies or library errors

**Solutions**:

1. **Install dependencies**:
   ```bash
   # Ubuntu/Debian
   sudo apt-get update
   sudo apt-get install libssl-dev libc6-dev
   
   # CentOS/RHEL
   sudo yum install openssl-devel glibc-devel
   ```

2. **Check library versions**:
   ```bash
   ldd guepard
   ```

3. **Use package manager**:
   ```bash
   # Use snap if available
   sudo snap install guepard
   ```

## FAQ

### General Questions

**Q: What is Guepard CLI?**
A: Guepard CLI is a command-line tool that provides Git-like version control capabilities for databases. It allows you to create snapshots, manage branches, and deploy database instances.

**Q: How is Guepard CLI different from Git?**
A: While Git manages code files, Guepard CLI manages database states. It provides similar commands (`commit`, `branch`, `checkout`) but works with database snapshots instead of code commits.

**Q: What databases are supported?**
A: Guepard CLI supports PostgreSQL, MySQL, and MongoDB. More database types may be added in future versions.

**Q: Is Guepard CLI free?**
A: Guepard CLI has a free tier with limited deployments, snapshots, and clones. Paid plans offer higher quotas and additional features.

### Installation Questions

**Q: How do I install Guepard CLI?**
A: See the [Installation Guide](installation.md) for detailed instructions for your platform.

**Q: Can I install Guepard CLI on multiple machines?**
A: Yes, you can install Guepard CLI on multiple machines and use the same account across all of them.

**Q: Do I need to install anything else?**
A: No, Guepard CLI is a standalone binary. However, you'll need internet connectivity to authenticate and manage deployments.

### Usage Questions

**Q: How do I get started?**
A: See the [Quick Start Guide](quick-start.md) for a step-by-step introduction.

**Q: Can I use Guepard CLI without a Guepard account?**
A: No, you need a Guepard account to use the CLI. Sign up at [guepard.run](https://guepard.run).

**Q: How do I create my first deployment?**
A: Use `guepard deploy --interactive` for a guided setup, or see the [Command Reference](commands.md) for command-line options.

**Q: Can I use Guepard CLI in CI/CD pipelines?**
A: Yes, see the [Workflows Guide](workflows.md) for CI/CD integration examples.

### Technical Questions

**Q: How are snapshots different from database backups?**
A: Snapshots are lightweight, incremental captures of database state that can be branched and merged, similar to Git commits. They're optimized for version control rather than disaster recovery.

**Q: Can I restore from snapshots?**
A: Yes, you can checkout any snapshot to restore your database to that state.

**Q: How do branches work with databases?**
A: Branches allow you to create parallel versions of your database for experimentation without affecting the main version.

**Q: What happens to my data when I switch branches?**
A: When you checkout a different branch, your database state changes to match that branch's snapshot. Your data is preserved in the snapshots.

**Q: Can I merge branches?**
A: Currently, Guepard CLI doesn't have automatic branch merging. You can manually apply changes from one branch to another by checking out snapshots.

### Billing and Limits

**Q: What are the usage limits?**
A: Check your current usage with `guepard usage`. Limits vary by plan.

**Q: How do I upgrade my plan?**
A: Visit your Guepard account dashboard at [guepard.run](https://guepard.run) to upgrade your plan.

**Q: What happens if I exceed my limits?**
A: You'll receive notifications when approaching limits. Some operations may be restricted when limits are exceeded.

## Getting Help

### Self-Help Resources

1. **Documentation**: This comprehensive guide covers all aspects of Guepard CLI
2. **Command Help**: Use `guepard <command> --help` for command-specific help
3. **Examples**: See [Real-World Examples](examples.md) for practical use cases
4. **Workflows**: Check [Workflows Guide](workflows.md) for best practices

### Community Support

1. **Discord Community**: Join our [Discord server](https://discord.gg/NYsNzQGvZT) for real-time help
2. **GitHub Issues**: Report bugs and request features on [GitHub](https://github.com/Guepard-Corp/guepard-cli/issues)
3. **GitHub Discussions**: Ask questions and share experiences in [GitHub Discussions](https://github.com/Guepard-Corp/guepard-cli/discussions)

### Professional Support

1. **Account Issues**: Contact support through your Guepard account dashboard
2. **Enterprise Support**: Contact sales for enterprise support options
3. **Bug Reports**: Use GitHub Issues for bug reports with detailed information

### Reporting Issues

When reporting issues, include:

1. **Guepard CLI version**: `guepard --version`
2. **Operating system**: macOS, Linux, or Windows version
3. **Command that failed**: Exact command and parameters
4. **Error message**: Complete error output
5. **Steps to reproduce**: What you did before the error occurred
6. **Expected behavior**: What should have happened
7. **Actual behavior**: What actually happened

### Example Issue Report

```
Title: Authentication fails on macOS Monterey

Description:
I'm unable to authenticate with Guepard CLI on macOS Monterey.

Steps to reproduce:
1. Install Guepard CLI via Homebrew
2. Run `guepard login`
3. Browser opens but login page shows error

Expected behavior:
Login page should load successfully and allow authentication.

Actual behavior:
Login page shows "Error loading page" message.

Environment:
- Guepard CLI version: 0.27.17
- macOS version: 12.6 Monterey
- Browser: Safari 15.6
- Network: Corporate WiFi (may have proxy)

Error message:
❌ Authentication required. Please run 'guepard login' first.
```

---

*If you can't find the answer to your question here, don't hesitate to reach out to our community or support team! 🐆*
