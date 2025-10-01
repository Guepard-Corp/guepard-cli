# Homebrew Distribution Guide for Guepard CLI

This guide will help you publish your Guepard CLI (`gfs`) to Homebrew for easy installation on macOS.

## What We've Set Up

✅ **Homebrew Formula**: Created `homebrew-tap/Formula/gfs.rb` with proper configuration
✅ **Installation Scripts**: Created setup and test scripts
✅ **Documentation**: Updated README.md with Homebrew installation instructions
✅ **Formula Validation**: Tested formula syntax and style

## Prerequisites

Before publishing, you need:

1. **GitHub Releases**: Your CLI binaries must be available as GitHub releases
2. **GitHub Repository**: A repository for your Homebrew tap
3. **GitHub CLI**: For automated setup (optional but recommended)

## Step-by-Step Publishing Process

### 1. Create GitHub Releases

First, you need to create GitHub releases with your binaries:

```bash
# Create a GitHub release with your binaries
# The release should be named v0.1.0 (matching your Cargo.toml version)
# Upload these files:
# - guepard-cli-0.1.0-macos-arm64.tar.gz
# - guepard-cli-0.1.0-macos-amd64.tar.gz
```

**Important**: The release URL in the formula must match exactly:
- Release tag: `v0.1.0`
- File names: `guepard-cli-0.1.0-macos-arm64.tar.gz` and `guepard-cli-0.1.0-macos-amd64.tar.gz`

### 2. Create Homebrew Tap Repository

#### Option A: Automated Setup (Recommended)

```bash
# Install GitHub CLI if you haven't already
brew install gh
gh auth login

# Run the automated setup script
./setup-homebrew-tap.sh
```

#### Option B: Manual Setup

1. Create a new GitHub repository named `homebrew-guepard-cli`
2. Clone it: `git clone https://github.com/Guepard-Corp/homebrew-guepard-cli.git`
3. Copy files: `cp -r homebrew-tap/* homebrew-guepard-cli/`
4. Commit and push:
   ```bash
   cd homebrew-guepard-cli
   git add .
   git commit -m "Initial Homebrew tap for Guepard CLI"
   git push origin main
   ```

### 3. Test Installation

Once your tap is published, test the installation:

```bash
# Add the tap
brew tap guepard-corp/guepard-cli

# Install gfs
brew install gfs

# Test it works
gfs --version
```

### 4. Update Documentation

Your README.md already includes Homebrew installation instructions. You may want to add them to other documentation as well.

## File Structure Created

```
guepard-cli-1/
├── homebrew-tap/
│   ├── Formula/
│   │   └── gfs.rb          # Homebrew formula
│   └── README.md           # Tap documentation
├── setup-homebrew-tap.sh   # Automated setup script
├── test-homebrew-formula.sh # Testing script
└── README.md               # Updated with Homebrew instructions
```

## Formula Details

The formula (`gfs.rb`) includes:

- **Multi-architecture support**: ARM64 and AMD64 for macOS
- **Proper SHA256 checksums**: For security verification
- **Version management**: Matches your Cargo.toml version
- **Test block**: Verifies installation works
- **License information**: Proper licensing

## Updating the Formula

When you release new versions:

1. Update the version in `homebrew-tap/Formula/gfs.rb`
2. Update the SHA256 checksums for both architectures
3. Update the URL to point to the new release
4. Commit and push changes to your tap repository

## Troubleshooting

### Common Issues

1. **404 Error**: Make sure GitHub releases exist with exact file names
2. **SHA256 Mismatch**: Recalculate checksums when updating binaries
3. **Formula Syntax**: Run `brew style` to check for issues

### Testing Commands

```bash
# Check formula style
brew style homebrew-tap/Formula/gfs.rb

# Test installation (after creating releases)
brew tap guepard-corp/guepard-cli
brew install gfs
```

## Next Steps

1. **Create GitHub Release**: Upload your binaries to GitHub releases
2. **Run Setup Script**: Execute `./setup-homebrew-tap.sh`
3. **Test Installation**: Verify everything works
4. **Announce**: Let users know they can install via Homebrew!

## User Installation

Once published, users can install your CLI with:

```bash
brew tap guepard-corp/guepard-cli
brew install gfs
```

This provides a much better user experience than manual binary downloads!
