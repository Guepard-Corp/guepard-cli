# Guepard CLI v0.25.1 - Multi-Platform Release

## 🎉 Successfully Released!

Your Guepard CLI has been successfully built and packaged for multiple platforms and architectures.

## 📦 Available Downloads

### macOS
- **macOS AMD64 (Intel)**: `guepard-cli-0.25.1-macos-amd64.tar.gz` (2.3 MB)
- **macOS ARM64 (Apple Silicon)**: `guepard-cli-0.25.1-macos-arm64.tar.gz` (2.3 MB)

### Windows
- **Windows AMD64**: `guepard-cli-0.25.1-windows-amd64.zip` (3.6 MB)

## 🔧 Binary Details

| Platform | Architecture | Binary Size | Archive Size |
|----------|-------------|-------------|--------------|
| macOS | AMD64 (Intel) | 5.6 MB | 2.3 MB |
| macOS | ARM64 (Apple Silicon) | 5.5 MB | 2.3 MB |
| Windows | AMD64 | 9.7 MB | 3.6 MB |

## 🚀 Installation Instructions

### macOS (Intel)
```bash
# Download and extract
curl -L https://github.com/your-repo/releases/download/v0.25.1/guepard-cli-0.25.1-macos-amd64.tar.gz | tar -xz

# Move to PATH
sudo mv gfs /usr/local/bin/

# Verify installation
gfs --version
```

### macOS (Apple Silicon)
```bash
# Download and extract
curl -L https://github.com/your-repo/releases/download/v0.25.1/guepard-cli-0.25.1-macos-arm64.tar.gz | tar -xz

# Move to PATH
sudo mv gfs /usr/local/bin/

# Verify installation
gfs --version
```

### Windows
```powershell
# Download and extract
Invoke-WebRequest -Uri "https://github.com/your-repo/releases/download/v0.25.1/guepard-cli-0.25.1-windows-amd64.zip" -OutFile "guepard-cli.zip"
Expand-Archive -Path "guepard-cli.zip" -DestinationPath "."

# Add to PATH (optional)
# Add the directory containing gfs.exe to your system PATH

# Verify installation
.\gfs.exe --version
```

## ✨ What's New in v0.25.1

### 🔄 API v0.4.4 Integration
- **Updated to latest API**: Full support for Guepard API v0.4.4
- **Removed clone_id**: Simplified API structure without clone_id references
- **Enhanced responses**: Support for new API response formats
- **Performance profiles**: Configurable performance profiles for deployments

### 🎯 Git-like Interface
- **Familiar commands**: `gfs init`, `gfs commit`, `gfs branch`, `gfs checkout`
- **Dual-mode support**: Both Git-like and original API commands work
- **Backward compatibility**: All existing workflows continue to work

### 🏗️ Multi-Platform Support
- **Cross-platform builds**: Native binaries for macOS and Windows
- **Architecture support**: Both AMD64 and ARM64 where available
- **Platform-specific features**: Keyring integration per platform

## 🛠️ Build Status

| Platform | Architecture | Status | Notes |
|----------|-------------|--------|-------|
| macOS | AMD64 | ✅ Success | Native build |
| macOS | ARM64 | ✅ Success | Native build |
| Windows | AMD64 | ✅ Success | Cross-compiled |
| Windows | ARM64 | ❌ Failed | Requires Visual Studio |
| Linux | AMD64 | ❌ Failed | Requires Docker/cross-compilation |
| Linux | ARM64 | ❌ Failed | Requires Docker/cross-compilation |

## 🔧 Technical Details

- **Rust version**: Latest stable
- **Target platforms**: macOS, Windows
- **Features**: Platform-specific keyring support
- **Dependencies**: Self-contained binaries
- **API compatibility**: v0.4.4

## 📋 Next Steps

1. **Upload to GitHub Releases**: Upload the built archives to your GitHub repository
2. **Update documentation**: Update installation instructions with download links
3. **Test on target platforms**: Verify functionality on macOS and Windows
4. **Linux builds**: Set up Docker or cross-compilation environment for Linux builds
5. **Windows ARM64**: Install Visual Studio Build Tools for Windows ARM64 support

## 🎯 Usage Examples

### Git-like Workflow
```bash
# Initialize repository
gfs init .

# Deploy database
gfs deploy --database-provider=PostgreSQL --database-version=16

# Create and commit changes
gfs commit -m "Initial database setup"

# Create and switch branches
gfs branch develop
gfs checkout develop
```

### Original API Workflow
```bash
# Create deployment
gfs deploy create -p PostgreSQL -v 16 -r us-west-aws -d us-west-aws -n myrepo -w password

# Create branch from snapshot
gfs branch create -x <deployment_id> -s <snapshot_id> -d false -k

# Manage compute
gfs compute start -x <deployment_id>
gfs compute status -x <deployment_id>
```

---

**Built with ❤️ using Rust and the Guepard CLI**
