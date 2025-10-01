#!/bin/bash

# Snap Package Setup Script for Guepard CLI
# This script helps you publish your CLI to the Snap Store

set -e

echo "🐧 Setting up Snap package for Guepard CLI"
echo "=========================================="

# Check if snapcraft is installed
if ! command -v snapcraft &> /dev/null; then
    echo "❌ snapcraft is not installed. Please install it first:"
    echo "   sudo snap install snapcraft --classic"
    echo "   snapcraft login"
    exit 1
fi

echo "📋 Snap package configuration created:"
echo "   - snap/snapcraft.yaml"
echo ""
echo "📝 Next steps to publish to Snap Store:"
echo ""
echo "1. Build Linux binaries first:"
echo "   ./build.sh linux-only"
echo ""
echo "2. Update snapcraft.yaml with correct SHA256:"
echo "   # Replace PLACEHOLDER_LINUX_AMD64_SHA256 with actual checksum"
echo ""
echo "3. Build the snap package:"
echo "   snapcraft"
echo ""
echo "4. Test the snap package locally:"
echo "   sudo snap install --dangerous gfs_0.25.1_amd64.snap"
echo ""
echo "5. Publish to Snap Store:"
echo "   snapcraft upload gfs_0.25.1_amd64.snap"
echo ""
echo "6. Release to stable channel:"
echo "   snapcraft release gfs 1 stable"
echo ""
echo "🎉 Users can then install with:"
echo "   snap install gfs"
