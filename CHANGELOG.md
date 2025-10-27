# Changelog

All notable changes to Guepard CLI will be documented in this file.

## v0.27.19 - v0.27.18 (2025-10-27)

### Changes
- Homebrew formula updates for v0.27.19

### Features
- **Enhanced deployment command**: Added compute information retrieval for accurate port display during deployment
- **Automatic password generation**: Introduced a new function to generate strong, random database passwords if not provided by the user
- **Improved deployment responses**: Modified deployment response structure to include port and connection string options

### Improvements
- Updated interactive deployment prompts to reflect changes in username and password handling
- Better compute information retrieval for more accurate port display

### Documentation
- Added release template for Guepard CLI
- Fixed documentation issues

### Infrastructure
- Added test pipeline with GitHub Actions
- Updated test pipeline workflow

## v0.27.17 (2025-10-08)

### Features
- **List command enhancement**: Added limit option to control displayed results
- **Branch command improvements**: Updated BranchResponse and related logic for optional fields

### Bug Fixes
- Fixed workflow path to release template in build process

## v0.27.16 (2025-10-08)

No changes (version bump only)

## v0.27.15 (2025-10-08)

### Features
- Added comprehensive testing workflow for Guepard CLI
- **Configuration management**: Added configuration management for API endpoint
- **Enhanced documentation**: Added comprehensive documentation for Guepard CLI
- **Log command improvements**:
  - Added date filtering options to log command
  - Enhanced log command with new options and structured log display
- **Branch improvements**: Updated branch_id to be optional and adjusted related logic

### Bug Fixes
- Updated Compute response structures for optional fields

## v0.27.14 (2025-10-08)

### Features
- **Logout enhancement**: Added user session check before logout
- **Login improvements**: Updated login command to accept arguments for direct access token input

### Testing
- Added unit tests

### Infrastructure
- Added issue templates
- Prepared repository for public release

## v0.27.13 (2025-10-06)

### Bug Fixes
- Fixed Homebrew download URLs for consistency
- Streamlined checksum retrieval from GitHub release body
- Fixed Homebrew formula versioning consistency

### Improvements
- Enhanced Homebrew publishing workflow with version processing improvements
- Updated .gitignore and added Cargo.lock file
- Refactored Homebrew formula generation

## v0.27.12 (2025-10-06)

### Infrastructure
- Refactored CI workflows for improved asset handling and cross-compilation setup
- Enhanced build-release workflow with platform selection options

## v0.27.11 (2025-10-06)

### Infrastructure
- Refactored CI workflows for version handling and Homebrew formula generation

## v0.27.10 (2025-10-06)

No changes (version bump only)

## v0.27.9 (2025-10-06)

### Infrastructure
- Updated snapcraft.yaml
- Refactored build-release workflow configuration for macOS targets

## v0.27.1 (2025-10-04)

### Major Features
- **Interactive deployment mode**: Introduced step-by-step deployment creation with guided prompts
  - Users can now select database provider, version, region, deployment type, datacenter, repository name, username, and performance profile interactively
  - Added confirmation prompts and summarized deployment details before proceeding
  - Improved usability and clarity in the deployment process

- **Enhanced deployment output**: Added detailed database connection information to deployment command output
- **Improved deployment functionality**: Enhanced deployment command functionality and improved output formatting
- **Refactored deployment structure**: Updated deployment command structure and enhanced deployment arguments

## v0.26.19 (2025-10-03)

### Improvements
- Updated version and refined project descriptions
- Enhanced macOS build process for cross-compilation

### Bug Fixes
- Fixed macOS cross-compilation: disabled keyring for cross-compile builds
- Fixed macOS cross-compilation: switched to cargo zigbuild with proper .cargo/config.toml configuration
- Fixed macOS cross-compilation for ring crate

### Infrastructure
- Switched to zig for macOS cross-compilation
- Fixed macOS Rust target installation

## v0.26.12 (2025-10-02)

### Infrastructure
- Added release workflows for Linux, macOS, and Windows
- Enhanced build-release workflow with platform selection options
- Added GitHub permissions for release creation

### Bug Fixes
- Fixed create-release job dependencies
- Fixed critical workflow issues
- Fixed Linux ARM64 and Windows cross-compilation issues
- Fixed Linux build structure - combined AMD64 and ARM64 into single job
- Fixed platform detection and Rust target installation

## v0.25.6 - v0.26.11 (2025-09)

### Infrastructure
- Implemented smart platform-specific release system
- Added automatic package manager updates for all platforms
- Fixed GitHub Actions workflow syntax
- Added Guepard CLI package configurations for Chocolatey and Homebrew
- Fixed binary name from gfs to guepard and added automated GitHub Actions

### Improvements
- Enhanced Homebrew formula with Linux SHA256 checksums
- Removed setup scripts for Homebrew tap and testing

## v0.25.2 (2025-09)

### Breaking Changes
- **Command renamed**: Changed command name from `gfs` to `guepard` throughout the codebase

### Features
- Multi-platform binaries for Linux, macOS, and Windows
- Package configurations for Homebrew and Chocolatey
- New deployment functionality
- Enhanced commit and logging features

### Improvements
- Updated all documentation and references to use `guepard` instead of `gfs`

## v0.25.1

Initial release with basic functionality.

---

*This changelog format will be maintained for all future releases.*

