# Guepard CLI

A Git-like filesystem CLI for databases, providing version control and management capabilities for database schemas and data.

## Features

- **Git-like Interface**: Familiar commands for developers (`init`, `commit`, `branch`, `checkout`, etc.)
- **Multi-platform Support**: Built for Windows, macOS, and Linux on both AMD64 and ARM64 architectures
- **Database Management**: Deploy and manage database instances with version control
- **Cross-platform**: Consistent experience across all supported platforms

## Installation

### Homebrew (macOS)

The easiest way to install on macOS:

```bash
brew tap guepard-corp/guepard-cli
brew install gfs
```

### Pre-built Binaries

Download the appropriate binary for your platform from the [Releases](https://github.com/Guepard-Corp/guepard-cli/releases) page:

- **Linux**: `guepard-cli-<version>-linux-amd64.tar.gz` or `guepard-cli-<version>-linux-arm64.tar.gz`
- **Windows**: `guepard-cli-<version>-windows-amd64.zip` or `guepard-cli-<version>-windows-arm64.zip`
- **macOS**: `guepard-cli-<version>-macos-amd64.tar.gz` or `guepard-cli-<version>-macos-arm64.tar.gz`

### From Source

```bash
# Clone the repository
git clone https://github.com/Guepard-Corp/guepard-cli.git
cd guepard-cli

# Build for current platform
make build

# Or build for all platforms
make build-all

# Install locally
make install
```

## Usage

```bash
# Initialize a new Guepard repository
gfs init .

# Deploy with database configuration
gfs deploy --database_provider=MySQL --database_version=8

# Create a commit
gfs commit -m "Initial version"

# List branches
gfs branch

# Create a new branch
gfs branch develop

# Switch to a branch
gfs checkout develop

# View commit history
gfs log

# Manage compute instances
gfs compute status
gfs compute start
gfs compute stop
gfs compute restart
```

## Supported Platforms

| Platform | Architecture | Status |
|----------|-------------|--------|
| Linux    | AMD64 (x86_64) | ✅ |
| Linux    | ARM64 (aarch64) | ✅ |
| Windows  | AMD64 (x86_64) | ✅ |
| Windows  | ARM64 (aarch64) | ✅ |
| macOS    | AMD64 (x86_64) | ✅ |
| macOS    | ARM64 (aarch64) | ✅ |

## Development

### Prerequisites

- Rust 1.70+ with Cargo
- Cross-compilation tools (installed automatically via `make install-tools`)

### Building

```bash
# Install cross-compilation tools
make install-tools

# Build for current platform
make build

# Build for all platforms
make build-all

# Run tests
make test

# Check code formatting
make check-format

# Run clippy
make clippy
```

### Cross-compilation

The project supports cross-compilation to all target platforms. Use the build script:

```bash
# Build for all platforms
./build.sh all

# Install tools only
./build.sh install-tools

# Clean build artifacts
./build.sh clean
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the Guepard License - see the [LICENSE](LICENSE) file for details.

## Support

- Documentation: [https://docs.guepard.run](https://docs.guepard.run)
- Issues: [GitHub Issues](https://github.com/Guepard-Corp/guepard-cli/issues)
- Community: [Discord](https://discord.gg/guepard)