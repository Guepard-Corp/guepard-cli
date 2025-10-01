#!/bin/bash

# Multi-platform build script for Guepard CLI
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Build configuration
PROJECT_NAME="guepard-cli"
BINARY_NAME="gfs"
VERSION=$(cargo metadata --format-version 1 | jq -r '.packages[0].version')
BUILD_DIR="dist"

# Target platforms
TARGETS=(
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu"
    "x86_64-pc-windows-gnu"
    "aarch64-pc-windows-gnu"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
)

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to get platform name
get_platform_name() {
    case $1 in
        "x86_64-unknown-linux-gnu") echo "linux-amd64" ;;
        "aarch64-unknown-linux-gnu") echo "linux-arm64" ;;
        "x86_64-pc-windows-gnu") echo "windows-amd64" ;;
        "aarch64-pc-windows-gnu") echo "windows-arm64" ;;
        "x86_64-apple-darwin") echo "macos-amd64" ;;
        "aarch64-apple-darwin") echo "macos-arm64" ;;
        *) echo "unknown" ;;
    esac
}

# Function to get binary extension
get_binary_extension() {
    case $1 in
        *windows*) echo ".exe" ;;
        *) echo "" ;;
    esac
}

# Function to check if target is installed
check_target() {
    local target=$1
    if ! rustup target list --installed | grep -q "^${target}$"; then
        print_status "Installing target: ${target}"
        rustup target add "${target}"
    else
        print_success "Target ${target} is already installed"
    fi
}

# Function to build for a specific target
build_target() {
    local target=$1
    local platform_name=$(get_platform_name "$target")
    local binary_ext=$(get_binary_extension "$target")
    
    print_status "Building for ${target} (${platform_name})"
    
    # Set platform-specific features
    local features=""
    case $target in
        *apple-darwin*)
            features="--features apple-keyring"
            ;;
        *windows*)
            features="--features windows-keyring"
            ;;
        *linux*)
            features="--features linux-keyring"
            ;;
    esac
    
    # Build the binary using cross if available, otherwise cargo
    if command -v cross &> /dev/null && [[ $target != *apple-darwin* ]]; then
        print_status "Using cross for ${target}"
        cross build --release --target "${target}" ${features}
    else
        print_status "Using cargo for ${target}"
        cargo build --release --target "${target}" ${features}
    fi
    
    # Create output directory
    local output_dir="${BUILD_DIR}/${platform_name}"
    mkdir -p "${output_dir}"
    
    # Copy binary
    local binary_path="target/${target}/release/${BINARY_NAME}${binary_ext}"
    local output_path="${output_dir}/${BINARY_NAME}${binary_ext}"
    
    if [ -f "${binary_path}" ]; then
        cp "${binary_path}" "${output_path}"
        print_success "Built ${platform_name}: ${output_path}"
        
        # Create archive
        local archive_name="${PROJECT_NAME}-${VERSION}-${platform_name}"
        if [[ $target == *windows* ]]; then
            # Create ZIP for Windows
            cd "${output_dir}"
            zip -r "../${archive_name}.zip" .
            cd - > /dev/null
            print_success "Created archive: ${BUILD_DIR}/${archive_name}.zip"
        else
            # Create tar.gz for Unix-like systems
            tar -czf "${BUILD_DIR}/${archive_name}.tar.gz" -C "${output_dir}" .
            print_success "Created archive: ${BUILD_DIR}/${archive_name}.tar.gz"
        fi
    else
        print_error "Binary not found: ${binary_path}"
        return 1
    fi
}

# Function to install cross-compilation tools
install_cross_tools() {
    print_status "Installing cross-compilation tools..."
    
    # Install cross for easier cross-compilation
    if ! command -v cross &> /dev/null; then
        print_status "Installing cross..."
        cargo install cross --git https://github.com/cross-rs/cross
    else
        print_success "cross is already installed"
    fi
    
    # Install additional tools for Windows targets
    if [[ " ${TARGETS[@]} " =~ " x86_64-pc-windows-gnu " ]] || [[ " ${TARGETS[@]} " =~ " aarch64-pc-windows-gnu " ]]; then
        print_status "Setting up Windows cross-compilation..."
        
        # Install mingw-w64 for Windows targets
        if command -v brew &> /dev/null; then
            if ! brew list mingw-w64 &> /dev/null; then
                brew install mingw-w64
            fi
        fi
    fi
}

# Main build function
main() {
    print_status "Starting multi-platform build for ${PROJECT_NAME} v${VERSION}"
    
    # Clean previous builds
    if [ -d "${BUILD_DIR}" ]; then
        print_status "Cleaning previous builds..."
        rm -rf "${BUILD_DIR}"
    fi
    
    mkdir -p "${BUILD_DIR}"
    
    # Install cross-compilation tools
    install_cross_tools
    
    # Build for each target
    local failed_targets=""
    for target in "${TARGETS[@]}"; do
        if check_target "${target}"; then
            if ! build_target "${target}"; then
                failed_targets="${failed_targets} ${target}"
            fi
        else
            failed_targets="${failed_targets} ${target}"
        fi
    done
    
    # Summary
    print_status "Build completed!"
    print_status "Built binaries are available in: ${BUILD_DIR}/"
    
    if [ -z "$failed_targets" ]; then
        print_success "All targets built successfully!"
        
        # List all created archives
        print_status "Created archives:"
        ls -la "${BUILD_DIR}"/*.tar.gz "${BUILD_DIR}"/*.zip 2>/dev/null || true
    else
        print_warning "Some targets failed to build:"
        for target in $failed_targets; do
            print_error "  - ${target}"
        done
        exit 1
    fi
}

# Parse command line arguments
case "${1:-all}" in
    "all")
        main
        ;;
    "install-tools")
        install_cross_tools
        ;;
    "clean")
        print_status "Cleaning build artifacts..."
        cargo clean
        rm -rf "${BUILD_DIR}"
        print_success "Clean completed!"
        ;;
    *)
        echo "Usage: $0 [all|install-tools|clean]"
        echo "  all          - Build for all targets (default)"
        echo "  install-tools - Install cross-compilation tools only"
        echo "  clean        - Clean build artifacts"
        exit 1
        ;;
esac