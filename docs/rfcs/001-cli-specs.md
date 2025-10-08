# RFC: Guepard CLI

- **RFC Number**: 001-cli-specs
- **Status**: Draft

## Summary

This RFC proposes the design and implementation of a `guepard` CLI tool to interact with the Guepard platform locally or remotely. 
The CLI will allow users:
- Create local databases with versionning enabled
- Auhtenticated with Guepard Cloud
- Create and manage remote databases

---

## Motivation

Guepard aims to empower developers and non-developers to build and manage data infrastructure with zero friction. A dedicated CLI will:
- Enable headless, scriptable interactions
- Provide faster feedback loops than web interfaces
- Unlock advanced workflows (e.g. CI/CD integrations, offline edits, Git-based operations)
- Serve as a foundational interface for agents and automations
- Increase adoption

---

## Concepts

- Provide a GIT like CLI
    - Users with prior knowledge of GIT will get same developer experience. No need for knowledge rampup to use Guepard CLI
    - Handle a database closely to how we handle code by enabling branching, versionning and time traveling 
- A branch is a pointer to a commit


# Usage

Example of how we can use the CLI

```
brew install guepard-cli

gfs init .
gfs deploy --database_provider=MySQL --database_version=8

```


# CLI Specs
| Command | Description | Example |
|--------------------------|-------------|---------|
| `gfs init <path>` | Initialize a new Guepard environment at the specified path | `gfs init .` |
| `gfs deploy` | Deploy the Guepard filesystem with specified database configuration | `gfs deploy --database_provider=MySQL --database_version=8 [--user myuser] [--password mypassword]` |
| `gfs commit -m "<message>"` | Create a new snapshot with the given message | `gfs commit -m "Initial version"` |
| `gfs branch` | list your branches. a * will appear next to the currently active branch | `gfs branch` |
| `gfs branch <branch>` | Create a new branch | `gfs branch develop` |
| `gfs log` | Show all commits in the current branch’s history | `gfs branch develop` |
| `gfs rev-parse` | Traverse parents folder until finds .gfs | `gfs rev-parse` |
| `gfs checkout <branch>` | Switch to the specified branch | `gfs checkout develop` |
| `gfs checkout <commit>` | Checkout a specific commit | `gfs checkout abc123` |
| `gfs compute status` | Show the current status of the database compute instance including state, resources, and connection info | `gfs compute status` |
| `gfs compute start` | Start the database compute instance | `gfs compute start` |
| `gfs compute stop` | Stop the database compute instance | `gfs compute stop` |
| `gfs compute restart` | Restart the database compute instance | `gfs compute restart` |



# Git output examples

commit f93ebaa9ad685d216e6532dd2ce7133df05cd0cf (HEAD -> master, develop)
Author: Hani CHALOUATI <hani.chalouati@gmail.com>
Date:   Sun May 25 21:51:59 2025 +0200

    Initial commit