![Guepard](/resources/guepard-cover.png)

<div align="center">
    <h1>Guepard Platform 🐆 The turbocharged Git for databases</h1>
    <br />  
    <p align="center">
    <a href="https://youtu.be/WlOkLnoY2h8?si=hb6-7kLhlOvVL1u6">
        <img src="https://img.shields.io/badge/Watch-YouTube-%23ffcb51?logo=youtube&logoColor=black" alt="Watch on YouTube" />
    </a>
    <a href="https://discord.gg/nCXAsUd3hm">
        <img src="https://img.shields.io/badge/Join-Community-%23ffcb51?logo=discord&logoColor=black" alt="Join our Community" />
    </a>
    <a href="https://github.com/Guepard-Corp/guepard-engine-v2/actions/workflows/build_and_test.yml" target="_blank">
        <img src="https://img.shields.io/github/actions/workflow/status/Guepard-Corp/guepard-cli/build-release.yml?branch=main" alt="Build">
    </a>
    </p>
</div>

# Guepard CLI

A CLI to interact with Guepard Platform, providing version control and management capabilities for database schemas and data.

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
brew install guepard
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
# Login to you account and link the CLI
guepard login

# List all you available databases
guepard list
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

## **📩 Contact & Community**
Have questions, feedback, or just want to say hi? We’d love to hear from you! We ❤️ contributions!

- Documentation: [https://docs.guepard.run](https://docs.guepard.run)
- Issues: [GitHub Issues](https://github.com/Guepard-Corp/guepard-cli/issues)
- Community: [Discord](https://discord.gg/nCXAsUd3hm)