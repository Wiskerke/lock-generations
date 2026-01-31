# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`lock-generations` is a CLI tool for managing NixOS system generations with selective protection. It allows users to clean up old generations while preserving specific generations that should not be deleted.

### Core Functionality
- **Protect/Unprotect Generations**: Mark specific NixOS generations as protected or unprotected
- **Clean Command**: Remove all unprotected generations while keeping:
  - The current active generation
  - Explicitly protected generations
  - Optionally, the last N most recent generations (via `--keep-last` flag)

### Technical Details
- Written in Rust using edition 2024
- Command-line interface for all operations
- Acts as a smart wrapper around NixOS's built-in commands
- The tool determines which generations to delete, then invokes NixOS commands (e.g., `nix-env --delete-generations`) to perform the actual deletion
- Does NOT manually delete generation profiles; delegates to NixOS tooling for safety and proper cleanup

## Development Environment

The project uses Nix for dependency management via `shell.nix`. The development environment includes:
- rustc (Rust compiler)
- cargo (Rust package manager)
- rustfmt (code formatter)
- clippy (linter)
- rust-analyzer (LSP server)

To enter the development environment: `nix-shell`

## Common Commands

### Building
- `cargo build` - Build the project in debug mode
- `cargo build --release` - Build optimized release binary

### Running
- `cargo run` - Build and run the project
- `cargo run --release` - Run optimized release build

### Testing
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run a specific test
- `cargo test -- --nocapture` - Run tests with stdout/stderr output

### Code Quality
- `cargo fmt` - Format code according to Rust style guidelines
- `cargo clippy` - Run Clippy linter for code analysis
- `cargo clippy -- -D warnings` - Run Clippy and treat warnings as errors
- `cargo check` - Fast compile check without producing binaries

## Project Structure

- `src/main.rs` - Entry point for the binary application
- `Cargo.toml` - Project manifest and dependencies
- `shell.nix` - Nix shell configuration for development environment

## Planned CLI Interface

The tool should support these subcommands:
- `lock-generations protect <generation-number>` - Add protection to a generation
- `lock-generations unprotect <generation-number>` - Remove protection from a generation
- `lock-generations clean` - Remove all unprotected generations (except current)
- `lock-generations clean --keep-last N` - Clean while preserving the last N generations

## Implementation Notes

- Protected generations list is persisted in `~/.config/lock-generations/protected.json` (JSON format)
- The tool needs to identify NixOS system generations (typically from `/nix/var/nix/profiles/`)
- Deletion should be performed by invoking NixOS commands:
  - `nix-env --delete-generations <generation-numbers>` for deleting specific generations
  - `nix-collect-garbage` may be optionally run afterward to reclaim disk space
- Should require appropriate permissions to delete generations (likely needs sudo/root)
- Current generation must always be preserved regardless of protection status
- The tool's responsibility is to determine the list of deletable generations based on protection rules, not to perform the deletion itself
- **Sudo config handling**: When running with sudo, the tool detects the original user via `SUDO_USER` environment variable and uses their config file, allowing users to manage protections as a regular user and run cleanup with sudo

## Testing Strategy

**IMPORTANT**: The codebase must include automated tests that simulate NixOS command behavior without requiring an actual NixOS system.

### Architecture for Testability
- Use dependency injection or traits to abstract command execution
- Create a trait (e.g., `NixOsCommandRunner`) that defines interfaces for:
  - Listing available generations
  - Getting the current generation
  - Deleting specific generations
- Provide both a real implementation (that executes actual commands) and a mock/fake implementation for testing

### Test Structure
- **Unit tests**: Test business logic for determining which generations to delete
  - Given a set of generations, protected list, and current generation
  - Verify correct generations are selected for deletion
  - Test `--keep-last N` flag behavior
  - Test edge cases (all protected, none protected, current generation handling)
- **Integration tests**: Test with simulated NixOS environment
  - Mock the command runner to simulate NixOS responses
  - Verify correct commands are invoked with correct arguments
  - Test error handling when commands fail

### Mock Implementation Guidelines
- The mock should simulate realistic NixOS behavior:
  - Return a list of generation numbers (e.g., 1, 2, 5, 7, 10)
  - Track which generations have been "deleted"
  - Prevent deletion of the current generation
  - Simulate command failures (permissions, invalid generation numbers)
- Consider using a test fixture or builder pattern to set up test scenarios
