# gits - A Git Wrapper for Sensitive Files

`gits` is a Rust CLI tool that enables developers to version-control sensitive files (like `.env`) in a completely separate Git repository (`.gits`) within their project, without affecting or leaving any trace in the main repository.

## Features

- **Separate Repository**: All sensitive files are tracked in a `.gits` directory, not `.git`. This repo is fully independent and invisible to the main project repo.
- **Full Git CLI Support**: Transparently supports all Git commands (add, commit, log, status, etc.) for the secrets repo.
- **Environment Variable Control**: All Git subprocesses are invoked with `GIT_DIR` and `GIT_WORK_TREE` set via environment variables.
- **Automatic Exclusion**: During `gits init`, `.gits` is automatically added to the main repo's `.git/info/exclude` file.
- **No Trace in Main Repo**: The main repo's `.gitignore` and `.git/info/exclude` ensure secrets and the `.gits` directory remain untracked and invisible to collaborators.

## Installation

```bash
cargo install --path .
```

Or install from crates.io:

```bash
cargo install gits
```

## Usage

### Initialize a new gits repository

```bash
# First, make sure you're in a git repository
git init

# Then initialize the gits repository
gits init
```

### Use standard Git commands with gits

```bash
# Add sensitive files
gits add .env

# Commit changes
gits commit -m "Add environment variables"

# View status
gits status

# View history
gits log
```

### Best Practices

1. Add sensitive files to your main repo's `.gitignore` as well for extra safety.
2. Never expose `.gits` or its contents in the main repo, either by tracking or by documentation.

## How It Works

`gits` works by setting the `GIT_DIR` environment variable to `.gits` and `GIT_WORK_TREE` to the current directory when executing Git commands. This ensures that all Git operations are performed on the separate `.gits` repository while working with files in your main project directory.

## License

MIT
