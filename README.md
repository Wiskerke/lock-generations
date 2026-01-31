# lock-generations

A command-line tool for managing NixOS system generations with selective protection.

## Overview

`lock-generations` is a utility that helps clean up old NixOS generations while preserving specific generations you want to keep. Unlike the standard `nix-collect-garbage` commands, this tool allows you to explicitly protect certain generations from deletion.

## Features

- **Add Protection**: Mark specific generations as protected to prevent deletion
- **Remove Protection**: Unmark generations to allow them to be cleaned up
- **Clean Generations**: Delete all unprotected generations while keeping:
  - Currently active generation
  - Explicitly protected generations
  - Optionally, the last N most recent generations
- **Command-line Interface**: Simple CLI for managing generation protection

## Goals

This tool addresses the common NixOS use case where you want to clean up disk space by removing old generations, but need to preserve certain known-good configurations for rollback or reference purposes.

The tool acts as a smart wrapper around NixOS's built-in generation management commands (such as `nix-env --delete-generations`), determining which generations should be deleted based on your protection rules, then invoking the appropriate NixOS commands to perform the actual deletion.

## Usage (Planned)

```bash
# Add protection to a specific generation
lock-generations protect <generation-number>

# Remove protection from a generation
lock-generations unprotect <generation-number>

# Clean up all unprotected generations
lock-generations clean

# Clean up while keeping the last N generations
lock-generations clean --keep-last N
```

## Development

### Testing

The project includes automated tests that simulate NixOS command behavior, allowing development and testing without requiring an actual NixOS system. Tests use mock implementations of NixOS commands to verify correct behavior.

Run tests with:
```bash
cargo test
```

### Development Status

This project is in early development. Core functionality is being implemented.
