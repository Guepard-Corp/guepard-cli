# Authentication Guide

Learn how to authenticate with Guepard CLI and manage your account access.

## Overview

Guepard CLI requires authentication to access your Guepard account and manage deployments. The authentication system uses secure tokens that are stored locally on your machine.

## Getting Started

### Prerequisites
- Guepard CLI installed (see [Installation Guide](installation.md))
- Guepard account (sign up at [guepard.run](https://guepard.run))
- Internet connection

## Authentication Methods

### 1. Interactive Login (Recommended)

The easiest way to authenticate is through interactive login:

```bash
guepard login
```

This will:
1. Generate a unique login URL
2. Open your browser to the Guepard login page
3. Prompt you for a verification code
4. Save your authentication token locally

**Step-by-step process:**

```bash
$ guepard login
🐆 Starting login process...
✅ Login URL generated successfully!
🔗 URL: https://guepard.run/auth/login?code=abc123def456
Press Enter to open the login page in your browser... 

# After pressing Enter, your browser opens to the login page
# Complete the login process in your browser
# Then return to the terminal

Enter the verification code from the webpage: 123456
🐆 Completing login...
✅ Login successful. Happy coding! 🐆
You can now use the Guepard CLI to interact with your Guepard account.🐆
💡 To get started, run: `guepard --help`
```

### 2. Direct Token Login

If you have an access token, you can authenticate directly:

```bash
guepard login --code your-access-token-here
```

**Example:**
```bash
guepard login --code eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

## Token Management

### Token Storage

Authentication tokens are stored securely on your local machine:

- **macOS**: `~/Library/Application Support/guepard/token`
- **Linux**: `~/.config/guepard/token`
- **Windows**: `%APPDATA%\guepard\token`

### Token Security

- Tokens are stored in plain text files (consider using keyring integration)
- Tokens have expiration dates
- Never share your tokens with others
- Use environment variables for CI/CD systems

### Token Refresh

Tokens automatically refresh when they expire. If you encounter authentication errors:

1. Try running `guepard login` again
2. Check your internet connection
3. Verify your Guepard account is active

## Checking Authentication Status

### Verify Login Status

Check if you're currently authenticated:

```bash
# Try any command that requires authentication
guepard list deployments
```

If you're not authenticated, you'll see:
```
❌ Authentication required. Please run 'guepard login' first.
```

### View Account Information

Once authenticated, you can view your account details:

```bash
guepard usage
```

This shows your current usage and quotas:
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

## Logging Out

### Sign Out

To log out and clear your stored credentials:

```bash
guepard logout
```

**Example output:**
```bash
$ guepard logout
✅ Logged out successfully! 🐆
```

### Verify Logout

After logging out, commands requiring authentication will fail:

```bash
$ guepard list deployments
❌ Authentication required. Please run 'guepard login' first.
```

## Troubleshooting Authentication

### Common Issues

#### "Authentication required" Error
```bash
❌ Authentication required. Please run 'guepard login' first.
```

**Solution:**
```bash
guepard login
```

#### "Invalid token" Error
```bash
❌ Invalid or expired token. Please run 'guepard login' again.
```

**Solution:**
```bash
guepard logout
guepard login
```

#### Browser Doesn't Open
If the browser doesn't open automatically:

1. Copy the URL from the terminal
2. Paste it into your browser manually
3. Complete the login process
4. Enter the verification code

#### Network Issues
If you encounter network-related errors:

1. Check your internet connection
2. Verify firewall settings
3. Try using a different network
4. Check if Guepard services are accessible

### Manual Token Management

#### View Stored Token
```bash
# macOS/Linux
cat ~/.config/guepard/token

# Windows
type %APPDATA%\guepard\token
```

#### Clear Stored Token
```bash
# macOS/Linux
rm ~/.config/guepard/token

# Windows
del %APPDATA%\guepard\token
```

#### Set Token via Environment Variable
```bash
# Set environment variable
export GUEPARD_TOKEN="your-token-here"

# Use the token
guepard login --code $GUEPARD_TOKEN
```

## CI/CD Integration

### GitHub Actions

For automated deployments, use environment variables:

```yaml
name: Deploy to Guepard
on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Setup Guepard CLI
        run: |
          wget https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.27.17/guepard-cli-0.27.17-linux-amd64.tar.gz
          tar -xzf guepard-cli-0.27.17-linux-amd64.tar.gz
          sudo mv guepard /usr/local/bin/
      
      - name: Authenticate with Guepard
        run: guepard login --code ${{ secrets.GUEPARD_TOKEN }}
        env:
          GUEPARD_TOKEN: ${{ secrets.GUEPARD_TOKEN }}
      
      - name: Deploy
        run: guepard deploy --interactive
```

### Other CI/CD Systems

For other CI/CD systems, follow the same pattern:

1. Install Guepard CLI
2. Set the `GUEPARD_TOKEN` environment variable
3. Run `guepard login --code $GUEPARD_TOKEN`
4. Execute your deployment commands

## Security Best Practices

### Token Security
- **Never commit tokens** to version control
- **Use environment variables** for CI/CD
- **Rotate tokens regularly** if possible
- **Monitor token usage** in your Guepard account

### Account Security
- **Use strong passwords** for your Guepard account
- **Enable two-factor authentication** if available
- **Monitor account activity** regularly
- **Log out** from shared machines

### Network Security
- **Use HTTPS** for all Guepard communications
- **Verify SSL certificates** in corporate environments
- **Use VPN** if required by your organization
- **Check firewall rules** for Guepard domains

## Account Management

### Creating a Guepard Account

If you don't have a Guepard account:

1. Visit [guepard.run](https://guepard.run)
2. Click "Sign Up" or "Get Started"
3. Complete the registration process
4. Verify your email address
5. Start using Guepard CLI

### Account Limits

Free accounts include:
- Limited number of deployments
- Limited snapshots
- Limited clones/branches

Upgrade your account for:
- More deployments
- Higher quotas
- Priority support
- Advanced features

### Support

If you encounter authentication issues:

1. **Check this guide** for common solutions
2. **Join our Discord** community for help
3. **Create a GitHub issue** for bugs
4. **Contact support** for account-specific issues

---

*Ready to authenticate? Run `guepard login` to get started! 🐆*
