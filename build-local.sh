#!/bin/bash

# Local build script to test the release process
# This mimics what GitHub Actions does

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}🚀 Local Guepard CLI Build Script${NC}"
echo "=================================="

# Parse arguments
TAG=${1:-"v0.26.4-linux"}
VERSION=$(echo "$TAG" | sed 's/-.*$//')

echo -e "${YELLOW}Tag: $TAG${NC}"
echo -e "${YELLOW}Version: $VERSION${NC}"

# Detect platforms from tag
PLATFORMS=""
if [[ "$TAG" == *"-linux"* ]]; then
    PLATFORMS="$PLATFORMS linux"
fi
if [[ "$TAG" == *"-macos"* ]]; then
    PLATFORMS="$PLATFORMS macos"
fi
if [[ "$TAG" == *"-windows"* ]]; then
    PLATFORMS="$PLATFORMS windows"
fi

# If no platform specified, build all
if [[ -z "$PLATFORMS" ]]; then
    PLATFORMS="linux macos windows"
fi

# Trim leading space
PLATFORMS=$(echo "$PLATFORMS" | sed 's/^ *//')

echo -e "${YELLOW}Detected platforms: '$PLATFORMS'${NC}"

# Clean previous builds
echo -e "${GREEN}🧹 Cleaning previous builds...${NC}"
rm -rf dist/
mkdir -p dist/

# Install Rust targets
echo -e "${GREEN}🔧 Installing Rust targets...${NC}"
rustup target add x86_64-unknown-linux-gnu || true
rustup target add aarch64-unknown-linux-gnu || true
rustup target add x86_64-apple-darwin || true
rustup target add aarch64-apple-darwin || true
rustup target add x86_64-pc-windows-msvc || true

echo "Installed targets:"
rustup target list --installed

# Build Linux
if [[ "$PLATFORMS" == *"linux"* ]]; then
    echo -e "${GREEN}🐧 Building Linux...${NC}"
    
    echo "Building Linux AMD64..."
    cargo build --release --target x86_64-unknown-linux-gnu
    mkdir -p dist/linux-amd64
    cp target/x86_64-unknown-linux-gnu/release/guepard dist/linux-amd64/
    tar -czf dist/guepard-cli-$VERSION-linux-amd64.tar.gz -C dist linux-amd64/
    
    echo "Building Linux ARM64..."
    cargo build --release --target aarch64-unknown-linux-gnu
    mkdir -p dist/linux-arm64
    cp target/aarch64-unknown-linux-gnu/release/guepard dist/linux-arm64/
    tar -czf dist/guepard-cli-$VERSION-linux-arm64.tar.gz -C dist linux-arm64/
fi

# Build macOS
if [[ "$PLATFORMS" == *"macos"* ]]; then
    echo -e "${GREEN}🍎 Building macOS...${NC}"
    
    echo "Building macOS AMD64..."
    cargo build --release --target x86_64-apple-darwin
    mkdir -p dist/macos-amd64
    cp target/x86_64-apple-darwin/release/guepard dist/macos-amd64/
    tar -czf dist/guepard-cli-$VERSION-macos-amd64.tar.gz -C dist macos-amd64/
    
    echo "Building macOS ARM64..."
    cargo build --release --target aarch64-apple-darwin
    mkdir -p dist/macos-arm64
    cp target/aarch64-apple-darwin/release/guepard dist/macos-arm64/
    tar -czf dist/guepard-cli-$VERSION-macos-arm64.tar.gz -C dist macos-arm64/
fi

# Build Windows
if [[ "$PLATFORMS" == *"windows"* ]]; then
    echo -e "${GREEN}🪟 Building Windows...${NC}"
    
    echo "Building Windows AMD64..."
    cargo build --release --target x86_64-pc-windows-msvc
    mkdir -p dist/windows-amd64
    cp target/x86_64-pc-windows-msvc/release/guepard.exe dist/windows-amd64/
    cd dist
    zip -r guepard-cli-$VERSION-windows-amd64.zip windows-amd64/
    cd ..
fi

# Calculate SHA256 checksums
echo -e "${GREEN}🔐 Calculating SHA256 checksums...${NC}"
cd dist
for file in *.tar.gz *.zip; do
    if [ -f "$file" ]; then
        echo "$file: $(shasum -a 256 "$file" | cut -d' ' -f1)"
    fi
done
cd ..

echo -e "${GREEN}✅ Build completed successfully!${NC}"
echo -e "${YELLOW}📁 Artifacts created in dist/ directory:${NC}"
ls -la dist/

echo -e "${GREEN}🎉 Ready to test! You can now:${NC}"
echo "1. Test the binaries locally"
echo "2. Push the tag to trigger GitHub Actions"
echo "3. Compare local vs GitHub Actions results"
