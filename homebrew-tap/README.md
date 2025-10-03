# Guepard Homebrew Tap

This tap provides the Guepard CLI (`guepard`) - Git, but for Data.

## Installation

```bash
# Add the tap
brew tap guepard-corp/guepard-cli

# Install guepard
brew install guepard
```

## Usage

After installation, you can use `guepard` commands:

```bash
# Initialize a new Guepard repository
guepard init .

# Deploy with database configuration
guepard deploy --database_provider=MySQL --database_version=8

# Create a commit
guepard commit -m "Initial version"

# List branches
guepard branch

# Create a new branch
guepard branch develop

# Switch to a branch
guepard checkout develop

# View commit history
guepard log

# Manage compute instances
guepard compute status
guepard compute start
guepard compute stop
guepard compute restart
```

## Documentation

- Website: https://www.guepard.run
- Documentation: https://docs.guepard.run
- GitHub: https://github.com/Guepard-Corp/guepard-cli

## License

This project is licensed under the Guepard License.
