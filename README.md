# mkunit

[![CI](https://github.com/uradical/mkunit/actions/workflows/ci.yml/badge.svg)](https://github.com/uradical/mkunit/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust: 1.70+](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![systemd: 249+](https://img.shields.io/badge/systemd-249%2B-green.svg)](https://systemd.io/)

A CLI tool for generating and managing systemd unit files.

## Features

- **Unit Creation**: Generate service, timer, path, socket, mount, and target units
- **Interactive Mode**: Prompts guide you through configuration
- **Scriptable**: Non-interactive mode with full CLI control
- **Dry Run**: Preview changes before applying
- **Unit Management**: Edit, show, validate, and remove existing units
- **Security Hardening**: Apply best-practice security defaults
- **Dual Scope**: Support for both user and system units
- **Git-friendly**: Keep unit files in version control with symlink support
- **Shell Completions**: bash, zsh, fish, and PowerShell

## Requirements

- **systemd 249+** (Ubuntu 22.04+, Debian 12+, RHEL 9+, Fedora 35+)
- **Rust 1.70+** (for building from source)

## Installation

### From source

```bash
git clone https://github.com/alan/mkunit.git
cd mkunit
cargo install --path .
```

### From crates.io

```bash
cargo install mkunit
```

### Pre-built binaries

Download from [GitHub Releases](https://github.com/alan/mkunit/releases).

## Quick Start

```bash
# Create a simple service (interactive)
mkunit service myapp

# Create a service with options (scriptable)
mkunit service myapp \
  --exec "/usr/bin/myapp" \
  --description "My Application" \
  --hardening \
  --install

# Preview what would be generated
mkunit service myapp --exec "/usr/bin/myapp" --dry-run
```

## Usage Examples

### Services

```bash
# Basic service
mkunit service myapp --exec "/usr/bin/myapp" --install

# Service with environment variables
mkunit service myapp \
  --exec "/usr/bin/myapp" \
  --env "PORT=8080" \
  --env "LOG_LEVEL=info" \
  --env-file "/etc/myapp/config"

# Hardened service (recommended for production)
mkunit service myapp \
  --exec "/usr/bin/myapp" \
  --user myapp \
  --workdir /var/lib/myapp \
  --hardening \
  --install --start
```

### Timers

```bash
# Daily backup at 4 AM
mkunit timer backup --on-calendar "*-*-* 04:00:00" --persistent

# Every 5 minutes
mkunit timer healthcheck --on-calendar "*:0/5" --unit healthcheck.service

# 30 seconds after boot
mkunit timer startup-task --on-boot 30s
```

### Path Watchers

```bash
# Trigger on file changes
mkunit path config-watcher --path-modified /etc/myapp/config.yml

# Trigger when directory has files
mkunit path upload-processor --directory-not-empty /var/uploads
```

### Socket Activation

```bash
# TCP socket
mkunit socket myapp --listen-stream 8080

# Unix socket
mkunit socket myapp --listen-stream /run/myapp.sock

# Accept mode (one instance per connection)
mkunit socket myapp --listen-stream 8080 --accept
```

### Version Control Workflow

Keep unit files in your git repository and symlink them to systemd:

```bash
# Create unit file in your repo
mkunit service myapp \
  --exec "/usr/bin/myapp" \
  --output ./systemd/myapp.service \
  --no-interactive

# Commit to git
git add systemd/myapp.service
git commit -m "Add myapp service unit"

# Link to systemd (creates symlink, runs daemon-reload)
mkunit link ./systemd/myapp.service --install

# For system services (requires sudo)
sudo mkunit link ./systemd/myapp.service --system --install
```

This approach lets you:
- Track unit file changes in git
- Deploy the same units across multiple machines
- Review unit changes in pull requests

### Managing Units

```bash
# Link an existing unit file (symlink to systemd dir)
mkunit link ./myapp.service --install

# Edit in $EDITOR
mkunit edit myapp

# View unit file with syntax highlighting
mkunit show myapp

# Validate a unit file
mkunit validate ./myapp.service

# Check status
mkunit status myapp

# Follow logs
mkunit logs myapp --follow --lines 100

# Remove (stops, disables, deletes)
mkunit remove myapp

# List all mkunit-created units
mkunit list
mkunit list --system
mkunit list --all
```

## Command Reference

### Global Options

| Flag | Short | Description |
|------|-------|-------------|
| `--verbose` | `-v` | Enable verbose output |
| `--dry-run` | | Preview without making changes |
| `--no-interactive` | | Fail instead of prompting |
| `--no-color` | | Disable colored output |
| `--help` | `-h` | Show help |
| `--version` | `-V` | Show version |

### Service Options

| Flag | Short | Description | Default |
|------|-------|-------------|---------|
| `--exec` | `-e` | Command to run | (required) |
| `--description` | `-d` | Unit description | `<name> service` |
| `--workdir` | `-w` | Working directory | |
| `--user` | `-u` | Run as user | |
| `--group` | `-g` | Run as group | |
| `--restart` | `-r` | Restart policy | `on-failure` |
| `--restart-sec` | | Restart delay (seconds) | `5` |
| `--type` | `-t` | Service type | `simple` |
| `--env` | | Environment `KEY=VALUE` | |
| `--env-file` | | Path to env file | |
| `--after` | | Start after unit(s) | `network.target` |
| `--wants` | | Weak dependency | |
| `--requires` | | Strong dependency | |
| `--wanted-by` | | Install target | `default.target` |
| `--system` | | System unit (vs user) | `false` |
| `--install` | `-i` | Enable immediately | `false` |
| `--start` | | Start after install | `false` |
| `--output` | `-o` | Write to file path | |
| `--hardening` | | Apply security options | `false` |

### Link Options

| Flag | Short | Description | Default |
|------|-------|-------------|---------|
| `--system` | | Link as system unit | `false` |
| `--install` | `-i` | Enable after linking | `false` |
| `--start` | | Start after enabling | `false` |
| `--force` | `-f` | Overwrite existing symlink | `false` |

### Hardening Options

When `--hardening` is enabled, these systemd security directives are applied:

```ini
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=read-only
PrivateTmp=true
ProtectKernelTunables=true
ProtectControlGroups=true
RestrictSUIDSGID=true
```

## Shell Completions

```bash
# Bash
mkunit completions bash > ~/.local/share/bash-completion/completions/mkunit

# Zsh
mkunit completions zsh > ~/.zfunc/_mkunit

# Fish
mkunit completions fish > ~/.config/fish/completions/mkunit.fish

# PowerShell
mkunit completions powershell > mkunit.ps1
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `NO_COLOR` | Disable colored output |
| `EDITOR` | Editor for `mkunit edit` |
| `VISUAL` | Preferred over `EDITOR` if set |

## Exit Codes

| Code | Description |
|------|-------------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | Unit not found |
| 4 | Permission denied |
| 5 | Systemd error |

## Development

```bash
# Run all checks
just check

# Run tests
cargo test

# Run lints
cargo clippy

# Format code
cargo fmt
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines.

## Security

See [SECURITY.md](SECURITY.md) for security policy and reporting vulnerabilities.

## License

MIT License - see [LICENSE](LICENSE) for details.

Copyright 2025 Alan
