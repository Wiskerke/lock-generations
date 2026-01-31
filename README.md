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
- **Dry Run Mode**: Preview what would be deleted and see the exact nix commands that would be executed
- **List Protected**: View all currently protected generations
- **Command-line Interface**: Simple CLI for managing generation protection

## Goals

This tool addresses the NixOS use case where you want to clean up disk space by removing old generations, but need to preserve certain known-good configurations for rollback or reference purposes.

The tool acts as a smart wrapper around NixOS's built-in generation management commands (such as `nix-env --delete-generations`), determining which generations should be deleted based on your protection rules, then invoking the appropriate NixOS commands to perform the actual deletion.

## Usage

### Basic Commands

```bash
# Add protection to a specific generation
lock-generations protect <generation-number>

# Remove protection from a generation
lock-generations unprotect <generation-number>

# List all protected generations
lock-generations list

# Preview what would be deleted without actually deleting
lock-generations clean --dry-run

# Clean up all unprotected generations (requires sudo)
sudo lock-generations clean

# Clean up while keeping the last N generations
sudo lock-generations clean --keep-last N
```

### Typical Workflow

The typical workflow is to manage protections as your regular user, then run the actual cleanup with sudo:

```bash
# 1. Protect important generations (as regular user)
lock-generations protect 42
lock-generations protect 50

# 2. Preview what would be deleted (as regular user)
lock-generations clean --dry-run

# 3. Actually perform the cleanup (with sudo)
sudo lock-generations clean
```

**Note**: The tool automatically finds your user's config file even when running with sudo, so protected generations set as your regular user will be respected when running `sudo lock-generations clean`.

### Config File Location

Protected generations are stored in `~/.config/lock-generations/protected.json` (or `$XDG_CONFIG_HOME/lock-generations/protected.json` if set).

## Development

### Project Structure

The codebase is organized into focused modules:
- `src/main.rs` - CLI interface and business logic
- `src/command_runner.rs` - Trait abstraction for command execution
- `src/real_runner.rs` - Real NixOS command implementation
- `src/mock_runner.rs` - Mock implementation for testing
- `src/protected_state.rs` - State persistence and config management

### Testing

The project includes comprehensive automated tests (18 tests) that simulate NixOS command behavior, allowing development and testing without requiring an actual NixOS system. Tests use mock implementations of NixOS commands to verify correct behavior.

Run tests with:
```bash
cargo test
```

Run code quality checks:
```bash
cargo clippy -- -D warnings  # Linting with no warnings allowed
cargo fmt --check             # Check code formatting
```

### Development Status

✅ Core functionality is fully implemented and tested
✅ All 18 tests passing
✅ No clippy warnings
✅ Code properly formatted and documented
✅ Ready for use on NixOS systems
