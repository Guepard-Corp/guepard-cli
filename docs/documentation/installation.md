# Installation Guide

This guide covers installing Guepard CLI on all supported platforms. Guepard CLI is available for Windows, macOS, and Linux on both AMD64 and ARM64 architectures.

## Prerequisites

- **Operating System**: Windows 10+, macOS 10.15+, or Linux (Ubuntu 18.04+, CentOS 7+)
- **Architecture**: AMD64 (x86_64) or ARM64 (aarch64)
- **Network**: Internet connection for authentication and API access
- **Terminal**: Command-line interface access

## Installation Methods

### 1. Homebrew (macOS) - Recommended

The easiest way to install Guepard CLI on macOS:

```bash
# Add the Guepard tap
brew tap guepard-corp/guepard-cli

# Install Guepard CLI
brew install gfs
```

**Note**: The Homebrew package installs as `gfs` but you can also use `guepard` command.

### 2. Pre-built Binaries

Download the appropriate binary for your platform from our [Releases](https://github.com/Guepard-Corp/guepard-cli/releases) page:

#### macOS
```bash
# For Intel Macs (AMD64)
wget https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.27.14/guepard-cli-0.27.14-macos-amd64.tar.gz
tar -xzf guepard-cli-0.27.14-macos-amd64.tar.gz
sudo mv guepard /usr/local/bin/

# For Apple Silicon Macs (ARM64)
wget https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.27.14/guepard-cli-0.27.14-macos-arm64.tar.gz
tar -xzf guepard-cli-0.27.14-macos-arm64.tar.gz
sudo mv guepard /usr/local/bin/
```

#### Linux
```bash
# For AMD64
wget https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.27.14/guepard-cli-0.27.14-linux-amd64.tar.gz
tar -xzf guepard-cli-0.27.14-linux-amd64.tar.gz
sudo mv guepard /usr/local/bin/

# For ARM64
wget https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.27.14/guepard-cli-0.27.14-linux-arm64.tar.gz
tar -xzf guepard-cli-0.27.14-linux-arm64.tar.gz
sudo mv guepard /usr/local/bin/
```

#### Windows
1. Download the appropriate ZIP file:
   - [Windows AMD64](https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.27.14/guepard-cli-0.27.14-windows-amd64.zip)
   - [Windows ARM64](https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.27.14/guepard-cli-0.27.14-windows-arm64.zip)

2. Extract the ZIP file
3. Move `guepard.exe` to a directory in your PATH (e.g., `C:\Windows\System32`)

### 3. Package Managers

#### Chocolatey (Windows)
```powershell
# Install via Chocolatey
choco install guepard
```

#### Snap (Linux)
```bash
# Install via Snap
sudo snap install guepard
```

### 4. From Source

If you want to build from source or contribute to development:

```bash
# Prerequisites: Rust 1.70+ and Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone the repository
git clone https://github.com/Guepard-Corp/guepard-cli.git
cd guepard-cli

# Build the project
cargo build --release

# Install globally
cargo install --path .
```

## Verification

After installation, verify that Guepard CLI is working:

```bash
# Check version
guepard --version
# Output: guepard 0.27.14

# Check help
guepard --help
```

## Post-Installation Setup

### 1. Authentication

Before using Guepard CLI, you need to authenticate with your Guepard account:

```bash
# Start interactive login
guepard login
```

This will:
1. Open your browser to the Guepard login page
2. Prompt you for a verification code
3. Save your authentication token locally

### 2. First Deployment

Create your first database deployment:

```bash
# Interactive deployment (recommended for beginners)
guepard deploy --interactive

# Or use command-line flags
guepard deploy -p PostgreSQL -v 16 -r us-west -d aws -n myapp -w password
```

## Platform-Specific Notes

### macOS
- **Apple Silicon**: Use the ARM64 binary for optimal performance
- **Intel Macs**: Use the AMD64 binary
- **Security**: You may need to allow the binary in System Preferences > Security & Privacy

### Linux
- **Permissions**: Ensure the binary has execute permissions (`chmod +x guepard`)
- **PATH**: Add `/usr/local/bin` to your PATH if not already included
- **Dependencies**: Most modern Linux distributions have all required dependencies

### Windows
- **Antivirus**: Some antivirus software may flag the binary; add it to exclusions if needed
- **PATH**: Ensure the directory containing `guepard.exe` is in your system PATH
- **PowerShell**: Works in both Command Prompt and PowerShell

## Troubleshooting Installation

### Common Issues

**Command not found**
```bash
# Check if guepard is in your PATH
which guepard
# or on Windows
where guepard

# If not found, add to PATH or use full path
/usr/local/bin/guepard --version
```

**Permission denied**
```bash
# Make binary executable
chmod +x guepard

# Or run with sudo (not recommended for regular use)
sudo guepard --version
```

**Network issues**
- Ensure you have internet connectivity
- Check firewall settings
- Verify DNS resolution

### Getting Help

If you encounter issues during installation:

1. **Check the logs**: Look for error messages in the terminal output
2. **Verify prerequisites**: Ensure your system meets the requirements
3. **Try alternative methods**: If one installation method fails, try another
4. **Community support**: Join our [Discord](https://discord.gg/NYsNzQGvZT) for help
5. **Report issues**: Create an issue on [GitHub](https://github.com/Guepard-Corp/guepard-cli/issues)

## Next Steps

After successful installation:

1. **Read the [Quick Start Guide](quick-start.md)** to get up and running
2. **Explore [Command Reference](commands.md)** to learn all available commands
3. **Check out [Examples](examples.md)** for real-world use cases
4. **Join our community** for support and updates

---

*Ready to start using Guepard CLI? Let's move on to the [Quick Start Guide](quick-start.md)! 🐆*
