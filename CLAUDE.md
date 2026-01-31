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

- `src/main.rs` - Entry point and CLI interface
- `src/command_runner.rs` - Trait definition for command execution abstraction
- `src/real_runner.rs` - Real NixOS command implementation
- `src/mock_runner.rs` - Mock implementation for testing
- `src/protected_state.rs` - State persistence and config file management
- `Cargo.toml` - Project manifest and dependencies
- `shell.nix` - Nix shell configuration for development environment

## CLI Interface

The tool supports these subcommands:
- `lock-generations protect <generation-number>` - Add protection to a generation
- `lock-generations unprotect <generation-number>` - Remove protection from a generation
- `lock-generations list` - List all currently protected generations
- `lock-generations clean` - Remove all unprotected generations (except current)
- `lock-generations clean --keep-last N` - Clean while preserving the last N generations
- `lock-generations clean --dry-run` - Preview what would be deleted without actually deleting

## Implementation Details

### Core Architecture
- **Trait-based design**: Uses `NixOsCommandRunner` trait to abstract command execution
  - `RealNixOsRunner`: Executes actual NixOS commands
  - `MockNixOsRunner`: Mock implementation for testing without a NixOS system
- **Command delegation**: The tool determines which generations to delete, then delegates to `nix-env --delete-generations` for actual deletion
- **Module organization**: Separation of concerns with dedicated modules for CLI, command execution, and state management

### State Management
- Protected generations are persisted in `~/.config/lock-generations/protected.json` (JSON format with pretty printing)
- Respects `XDG_CONFIG_HOME` environment variable if set
- **Sudo config handling**: When running with sudo, automatically detects the original user via `SUDO_USER` environment variable and uses their config file
- Atomic file writes to prevent corruption (write to temp file, then rename)

### Generation Management
- Uses `/nix/var/nix/profiles/system` as the default profile path
- Current generation is always preserved regardless of protection status
- Deletion performed via: `nix-env --delete-generations <numbers> -p /nix/var/nix/profiles/system`
- Requires appropriate permissions (typically sudo/root) for deletion

## Testing

The codebase includes comprehensive automated tests that simulate NixOS command behavior without requiring an actual NixOS system.

### Test Coverage (18 tests total)
- **MockRunner tests** (5 tests): Verify mock implementation behavior
- **ProtectedState tests** (3 tests): Test state persistence and serialization
- **Clean command tests** (10 tests):
  - Basic functionality (no protected, with keep-last, dry-run)
  - Integration with protected state
  - Edge cases (empty lists, all protected, non-sequential generations)
  - Combined scenarios (protected + keep-last)

### Code Quality Status
- All tests passing (`cargo test`)
- No clippy warnings (`cargo clippy -- -D warnings`)
- Code properly formatted (`cargo fmt`)
- Public APIs documented with doc comments
- Error handling with `anyhow::Result` throughout
