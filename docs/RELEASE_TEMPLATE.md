## Guepard CLI {{VERSION}}

### Platforms Released
{{PLATFORMS}}

### Installation

#### macOS (Homebrew)
```bash
brew tap guepard-corp/guepard-cli
brew install guepard
```

#### Linux (Homebrew)
```bash
brew tap guepard-corp/guepard-cli
brew install guepard
```

#### Windows (Chocolatey)
```powershell
choco install guepard
```

#### Linux (Snap)
```bash
sudo snap install guepard
```

### Direct Downloads

#### macOS (Intel)
```bash
# Download and extract
curl -L https://github.com/Guepard-Corp/guepard-cli/releases/download/{{VERSION}}/guepard-cli-{{VERSION}}-macos-amd64.tar.gz | tar -xz

# Move to PATH
sudo mv guepard /usr/local/bin/

# Verify installation
guepard --version
```

#### macOS (Apple Silicon)
```bash
# Download and extract
curl -L https://github.com/Guepard-Corp/guepard-cli/releases/download/{{VERSION}}/guepard-cli-{{VERSION}}-macos-arm64.tar.gz | tar -xz

# Move to PATH
sudo mv guepard /usr/local/bin/

# Verify installation
guepard --version
```

#### Windows
```powershell
# Download and extract
Invoke-WebRequest -Uri "https://github.com/Guepard-Corp/guepard-cli/releases/download/{{VERSION}}/guepard-cli-{{VERSION}}-windows-amd64.zip" -OutFile "guepard-cli.zip"
Expand-Archive -Path "guepard-cli.zip" -DestinationPath "."

# Add to PATH (optional)
# Add the directory containing guepard.exe to your system PATH

# Verify installation
.\guepard.exe --version
```

### SHA256 Checksums
- Linux AMD64: `{{LINUX_AMD64_SHA256}}`
- Linux ARM64: `{{LINUX_ARM64_SHA256}}`
- macOS AMD64: `{{MACOS_AMD64_SHA256}}`
- macOS ARM64: `{{MACOS_ARM64_SHA256}}`
- Windows AMD64: `{{WINDOWS_AMD64_SHA256}}`

### What's New

#### Bug Fixes
- **Fixed connection port display**: Now correctly shows the assigned proxy port instead of defaulting to 5432
- **Improved interactive deployment**: All fields now have sensible defaults, making it easier to deploy databases
- **Enhanced password generation**: Auto-generated passwords are now 13 characters with special characters for better security

#### Improvements
- **Interactive mode default confirmation**: Changed from "y/N" to "Y/n" for faster deployments
- **Smart port fetching**: Automatically fetches the correct proxy port from compute endpoint
- **Better field defaults**: Region defaults to "global", datacenter to "us-west-aws", username to "postgres"
- **Auto-generated passwords**: When creating deployments, passwords can now be auto-generated if not provided

#### Features
- Multi-platform support (macOS, Windows, Linux)
- Git-like workflow for databases
- Familiar command interface
- Database version control and management
- API v0.4.4 integration
- Enhanced performance profiles
- Cross-platform keyring support

### Quick Start
```bash
# Login to Guepard
guepard login

# Interactive deployment (easy guided setup)
guepard deploy -I

# Create a deployment with custom settings
guepard deploy -p PostgreSQL -v 16 -r us-west -i REPOSITORY -d us-west-aws -w "your-password"

# View deployment details (with correct port)
guepard deploy -x <deployment-id>

# Create a commit
guepard commit -m "Initial version"

# List branches
guepard branch

# Create a new branch
guepard branch develop

# Switch to a branch
guepard checkout develop

# View commit history
guepard log

# Manage compute instances
guepard compute status
guepard compute start
guepard compute stop
guepard compute restart
```

### Technical Details
- **Rust version**: Latest stable
- **Target platforms**: macOS, Windows, Linux
- **Features**: Platform-specific keyring support
- **Dependencies**: Self-contained binaries
- **API compatibility**: v0.4.4

### Documentation
- Website: https://www.guepard.run
- Documentation: https://docs.guepard.run
- GitHub: https://github.com/Guepard-Corp/guepard-cli
