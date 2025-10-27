# Homebrew Core Submission for Guepard vv0.26.19

## Formula Details
- **Name**: guepard
- **Version**: v0.27.17
- **Description**: Guepard CLI - Git for Data
- **Homepage**: https://www.guepard.run
- **License**: Guepard (c) 2025

## Checksums
- macOS ARM64: `0019dfc4b32d63c1392aa264aed2253c1e0c2fb09216f8e2cc269bbfb8bb49b5`
- macOS AMD64: `0019dfc4b32d63c1392aa264aed2253c1e0c2fb09216f8e2cc269bbfb8bb49b5`
- Linux ARM64: `0019dfc4b32d63c1392aa264aed2253c1e0c2fb09216f8e2cc269bbfb8bb49b5`
- Linux AMD64: `0019dfc4b32d63c1392aa264aed2253c1e0c2fb09216f8e2cc269bbfb8bb49b5`

## Submission Steps

1. **Fork homebrew-core**:
   ```bash
   # Go to https://github.com/Homebrew/homebrew-core and fork it
   git clone https://github.com/YOUR_USERNAME/homebrew-core.git
   cd homebrew-core
   ```

2. **Copy the formula**:
   ```bash
   cp Formula/guepard.rb Formula/
   ```

3. **Create branch and commit**:
   ```bash
   git checkout -b add-guepard-v0.27.17
   git add Formula/guepard.rb
   git commit -m "guepard v0.27.17 (new formula)"
   git push origin add-guepard-v0.27.17
   ```

4. **Create Pull Request**:
   - Go to your fork on GitHub
   - Create PR against Homebrew/homebrew-core
   - Title: "guepard v0.27.17 (new formula)"
   - Use the description template below

## PR Description Template

```
guepard v0.27.17 (new formula)

Guepard CLI - Git for Data. Provides version control and management 
capabilities for databases with familiar Git-like commands like init, 
commit, branch, checkout, and more.

Features:
- Git-like interface with familiar commands
- Multi-platform support (Linux, macOS, Windows)
- Database management and deployment
- Cross-platform consistency

Homepage: https://www.guepard.run
License: Guepard (c) 2025
```

## Files Ready for Submission
- ✅ Formula: `Formula/guepard.rb`
- ✅ Checksums: All calculated and verified
- ✅ Instructions: This file

Once the PR is merged, users can install with: `brew install guepard`
