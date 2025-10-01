# Guepard Homebrew Tap

This tap provides the Guepard CLI (`gfs`) - a Git-like filesystem for databases.

## Installation

```bash
# Add the tap
brew tap guepard-corp/guepard-cli

# Install gfs
brew install gfs
```

## Usage

After installation, you can use `gfs` commands:

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

## Documentation

- Website: https://www.guepard.run
- Documentation: https://docs.guepard.run
- GitHub: https://github.com/Guepard-Corp/guepard-cli

## License

This project is licensed under the Guepard License.
