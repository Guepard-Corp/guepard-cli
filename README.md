# Guepard CLI

`guepard-cli` is a tool to manage your target environment. This CLI allows you to manage volumes and snapshots efficiently.

## Installation

1. Clone the repository
2. Navigate to the project directory
3. Build the project using Cargo:

    ```sh
    cargo build --release
    ```

4. Run the CLI tool:

    ```sh
    ./target/release/gprd -h
    ```

## Usage

### General CLI Structure

```sh
gprd <SUBCOMMAND>
```


## Branch Management Commands
### List Branches
Lists all branches.

```sh
gprd branch list
```

## Bookmark Management Commands
### List Bookmarks
Lists all bookmarks.

```sh
gprd bookmark list
```

### Make Bookmark
Creates a new bookmark for a branch.

```sh
gprd bookmark make --branch <BRANCH_NAME> --bookmark <BOOKMARK_NAME>
```

## Environment Variables
The following environment variables can be set to configure the CLI:

SCRIPTS_PATH: Path to the scripts directory. Default is /home/guepard/guepard-cli/scripts.
ENGINE_HOST: Host of the engine. This must be set.
You can create a .env file in the root directory of the project to set these variables:

```env
SCRIPTS_PATH=/path/to/scripts
```
