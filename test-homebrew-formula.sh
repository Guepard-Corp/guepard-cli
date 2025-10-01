#!/bin/bash

# Test Homebrew Formula Locally
# This script helps you test your Homebrew formula before publishing

set -e

echo "🧪 Testing Homebrew formula locally"
echo "===================================="

# Check if we're in the right directory
if [ ! -f "homebrew-tap/Formula/gfs.rb" ]; then
    echo "❌ Error: homebrew-tap/Formula/gfs.rb not found. Please run this script from the project root."
    exit 1
fi

echo "📋 Formula validation:"
echo "----------------------"

# Test formula syntax
echo "1. Checking formula syntax..."
brew audit --strict homebrew-tap/Formula/gfs.rb

echo "2. Checking formula style..."
brew style homebrew-tap/Formula/gfs.rb

echo "3. Testing formula installation..."
echo "   Installing formula locally..."

# Create a temporary tap
TEMP_TAP_DIR=$(mktemp -d)
cp -r homebrew-tap/* "$TEMP_TAP_DIR"

# Test installation
brew install --build-from-source "$TEMP_TAP_DIR/Formula/gfs.rb"

echo "4. Testing binary functionality..."
gfs --version

echo "5. Testing formula uninstallation..."
brew uninstall gfs

# Cleanup
rm -rf "$TEMP_TAP_DIR"

echo ""
echo "✅ All tests passed! Your formula is ready for publishing."
echo ""
echo "Next steps:"
echo "1. Run ./setup-homebrew-tap.sh to create the GitHub repository"
echo "2. Update your main project's README.md with Homebrew installation instructions"
echo "3. Create GitHub releases with the proper naming convention"
