# Makefile for Guepard CLI multi-platform builds

.PHONY: help build build-all clean test install-tools check-format clippy

# Default target
help:
	@echo "Available targets:"
	@echo "  build          - Build for current platform"
	@echo "  build-all      - Build for all supported platforms"
	@echo "  clean          - Clean build artifacts"
	@echo "  test           - Run tests"
	@echo "  install-tools  - Install cross-compilation tools"
	@echo "  check-format   - Check code formatting"
	@echo "  clippy         - Run clippy linter"
	@echo "  release        - Create release builds"

# Build for current platform
build:
	cargo build --release

# Build for all platforms using the build script
build-all:
	./build.sh all

# Clean build artifacts
clean:
	cargo clean
	rm -rf dist/

# Run tests
test:
	cargo test

# Install cross-compilation tools
install-tools:
	./build.sh install-tools

# Check code formatting
check-format:
	cargo fmt -- --check

# Run clippy
clippy:
	cargo clippy -- -D warnings

# Format code
format:
	cargo fmt

# Create release builds
release: clean
	cargo build --release
	./build.sh all

# Quick development build
dev:
	cargo build

# Run the CLI with help
help-cli: build
	./target/release/gfs --help

# Test specific platform builds
test-linux:
	cargo build --release --target x86_64-unknown-linux-gnu

test-windows:
	cargo build --release --target x86_64-pc-windows-gnu

test-macos:
	cargo build --release --target x86_64-apple-darwin

# Install binary to local system (requires sudo)
install: build
	sudo cp target/release/gfs /usr/local/bin/
	sudo chmod +x /usr/local/bin/gfs

# Uninstall binary from local system
uninstall:
	sudo rm -f /usr/local/bin/gfs
