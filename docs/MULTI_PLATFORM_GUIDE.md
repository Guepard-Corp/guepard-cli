# Multi-Platform Package Distribution Guide

This guide covers distributing your Guepard CLI across multiple package managers for different platforms.

## Current Status

✅ **macOS**: Available via Homebrew (`brew tap guepard-corp/guepard-cli && brew install guepard`)
🔄 **Linux**: Snap package ready (needs Linux binaries)
🔄 **Windows**: Chocolatey package ready (needs Windows binary checksum)

## Package Managers Setup

### 1. Homebrew (macOS) - ✅ COMPLETE

**Status**: Published and working
**Installation**: `brew tap guepard-corp/guepard-cli && brew install guepard`

**Files**:
- `homebrew-tap/Formula/guepard.rb` - Homebrew formula
- `homebrew-tap/README.md` - Tap documentation

### 2. Snap (Linux) - 🔄 READY FOR PUBLISHING

**Status**: Package configuration complete, needs Linux binaries
**Target**: Ubuntu, Debian, Fedora, Arch Linux, etc.

**Files**:
- `snap/snapcraft.yaml` - Snap package configuration
- `setup-snap.sh` - Setup script

**Next Steps**:
1. Build Linux binaries: `./build.sh linux-only` (needs Docker/cross-compilation setup)
2. Update SHA256 in `snap/snapcraft.yaml`
3. Build snap: `snapcraft`
4. Publish: `snapcraft upload gfs_0.25.1_amd64.snap`

**User Installation**: `snap install guepard`

### 3. Chocolatey (Windows) - 🔄 READY FOR PUBLISHING

**Status**: Package configuration complete, needs Windows binary checksum
**Target**: Windows 7+ with PowerShell

**Files**:
- `chocolatey/guepard.nuspec` - Package metadata
- `chocolatey/tools/chocolateyInstall.ps1` - Installation script
- `chocolatey/tools/chocolateyUninstall.ps1` - Uninstallation script
- `setup-chocolatey.sh` - Setup script

**Next Steps**:
1. Calculate Windows binary SHA256
2. Update `chocolatey/tools/chocolateyInstall.ps1` with correct checksum
3. Build package: `choco pack chocolatey/gfs.nuspec`
4. Publish: `choco push gfs.0.25.1.nupkg --source https://push.chocolatey.org/`

**User Installation**: `choco install guepard`

## Building Linux Binaries

The main blocker for Snap is building Linux binaries. Here are the options:

### Option A: Docker-based Cross-compilation (Recommended)
```bash
# Start Docker Desktop
# Then run:
./build.sh linux-only
```

### Option B: GitHub Actions CI/CD
Create a workflow that builds Linux binaries automatically on every release.

### Option C: Manual Linux Build
Build on a Linux machine or VM.

## Package Manager Comparison

| Platform | Package Manager | Status | User Command |
|----------|-----------------|--------|--------------|
| macOS    | Homebrew        | ✅ Live | `brew install guepard` |
| Linux    | Snap | 🔄 Ready | `snap install guepard` |
| Windows  | Chocolatey      | 🔄 Ready | `choco install guepard` |
| Linux    | APT/YUM         | ❌ Not setup | Manual download |
| Windows  | Scoop           | ❌ Not setup | Manual download |

## Current Installation Methods

### macOS
```bash
# Via Homebrew (Recommended)
brew tap guepard-corp/guepard-cli
brew install guepard

# Direct download
# Download from GitHub releases
```

### Linux
```bash
# Direct download (Current)
wget https://github.com/Guepard-Corp/guepard-cli/releases/download/v0.25.1/guepard-cli-0.25.1-linux-amd64.tar.gz
tar -xzf guepard-cli-0.25.1-linux-amd64.tar.gz
sudo mv gfs /usr/local/bin/

# Via Snap (Coming soon)
snap install guepard
```

### Windows
```bash
# Direct download (Current)
# Download from GitHub releases and extract

# Via Chocolatey (Coming soon)
choco install guepard
```

## Next Steps Priority

1. **High Priority**: Build Linux binaries for Snap
2. **Medium Priority**: Complete Chocolatey package
3. **Low Priority**: Add APT/YUM packages for Linux
4. **Low Priority**: Add Scoop package for Windows

## File Structure

```
guepard-cli-1/
├── homebrew-tap/           # ✅ Complete
│   ├── Formula/guepard.rb
│   └── README.md
├── snap/                   # 🔄 Ready
│   └── snapcraft.yaml
├── chocolatey/             # 🔄 Ready
│   ├── guepard.nuspec
│   └── tools/
│       ├── chocolateyInstall.ps1
│       └── chocolateyUninstall.ps1
├── setup-snap.sh           # Setup script
├── setup-chocolatey.sh     # Setup script
└── dist/                   # Binaries
    ├── guepard-cli-0.27.17-macos-*.tar.gz
    ├── guepard-cli-0.27.17-windows-*.zip
    └── guepard-cli-0.27.17-linux-*.tar.gz (needs building)
```

## Testing Commands

### Test Homebrew (macOS)
```bash
brew tap guepard-corp/guepard-cli
brew install guepard
guepard --version
```

### Test Snap (Linux) - After building
```bash
snapcraft
sudo snap install --dangerous guepard_0.27.17_amd64.snap
guepard --version
```

### Test Chocolatey (Windows) - After building
```bash
choco pack chocolatey/guepard.nuspec
choco install guepard --source . --force
guepard --version
```

Your CLI is well-positioned for multi-platform distribution! The main remaining work is building Linux binaries and completing the package publishing process.
