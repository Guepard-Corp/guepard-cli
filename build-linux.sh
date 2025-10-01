#!/bin/bash

# Build Linux Binaries Script
# This script builds Linux binaries when Docker is available

set -e

echo "🐧 Building Linux binaries for Guepard CLI"
echo "=========================================="

# Check if Docker is running
if ! docker ps &> /dev/null; then
    echo "❌ Docker is not running. Please start Docker Desktop first."
    echo "   Then run: ./build-linux.sh"
    exit 1
fi

echo "✅ Docker is running. Building Linux binaries..."

# Build for Linux AMD64
echo "🔨 Building for x86_64-unknown-linux-gnu..."
cross build --release --target x86_64-unknown-linux-gnu --features linux-keyring

# Build for Linux ARM64
echo "🔨 Building for aarch64-unknown-linux-gnu..."
cross build --release --target aarch64-unknown-linux-gnu --features linux-keyring

# Create archives
echo "📦 Creating archives..."

# Linux AMD64
mkdir -p dist/linux-amd64
cp target/x86_64-unknown-linux-gnu/release/gfs dist/linux-amd64/
tar -czf dist/guepard-cli-0.25.1-linux-amd64.tar.gz -C dist/linux-amd64 .
echo "✅ Created: dist/guepard-cli-0.25.1-linux-amd64.tar.gz"

# Linux ARM64
mkdir -p dist/linux-arm64
cp target/aarch64-unknown-linux-gnu/release/gfs dist/linux-arm64/
tar -czf dist/guepard-cli-0.25.1-linux-arm64.tar.gz -C dist/linux-arm64 .
echo "✅ Created: dist/guepard-cli-0.25.1-linux-arm64.tar.gz"

# Calculate checksums
echo "🔍 Calculating checksums..."
echo "Linux AMD64 SHA256: $(shasum -a 256 dist/guepard-cli-0.25.1-linux-amd64.tar.gz | cut -d' ' -f1)"
echo "Linux ARM64 SHA256: $(shasum -a 256 dist/guepard-cli-0.25.1-linux-arm64.tar.gz | cut -d' ' -f1)"

echo ""
echo "🎉 Linux binaries built successfully!"
echo "📁 Files created:"
ls -la dist/guepard-cli-0.25.1-linux-*.tar.gz
