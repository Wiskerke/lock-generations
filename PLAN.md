# Implementation Plan for lock-generations

This document outlines the implementation plan for the lock-generations tool.

## Overview

Build a CLI tool that manages NixOS system generations with selective protection, acting as a smart wrapper around NixOS's built-in `nix-env` commands.

## Phase 1: Foundation & Architecture

### 1. Set up project dependencies
- Add `clap` for CLI argument parsing and subcommand handling
- Add `serde` and `serde_json` (or `toml`) for serializing/deserializing protection state
- Update `Cargo.toml` with necessary dependencies

### 2. Design and implement NixOsCommandRunner trait
Create a trait to abstract NixOS command execution:
```rust
trait NixOsCommandRunner {
    fn list_generations(&self) -> Result<Vec<Generation>>;
    fn get_current_generation(&self) -> Result<u32>;
    fn delete_generations(&self, generations: &[u32]) -> Result<()>;
}
```

### 3. Implement real NixOsCommandRunner
- Execute actual `nix-env` commands
- Parse command output to extract generation information
- Handle command failures and errors appropriately
- Consider profile paths (e.g., `/nix/var/nix/profiles/system`)

### 4. Implement mock NixOsCommandRunner for testing
- Simulate NixOS behavior without executing real commands
- Track internal state (available generations, current generation, deleted generations)
- Allow test setup with custom generation lists
- Simulate realistic error conditions

## Phase 2: State Management

### 5. Design protection state storage
- Decide on file format (JSON or TOML)
- Determine storage location (e.g., `~/.config/lock-generations/protected.json` or `/etc/lock-generations/protected.json`)
- Define data structure for storing protected generation numbers

### 6. Implement protection state persistence
- Implement `load_protected_generations()` function
- Implement `save_protected_generations()` function
- Handle missing config file gracefully (empty protection list)
- Ensure atomic writes to prevent corruption

## Phase 3: CLI Implementation

### 7. Set up CLI structure with clap
- Define main command with subcommands: `protect`, `unprotect`, `clean`
- Add help text and examples for each subcommand
- Set up argument parsing for generation numbers and flags

### 8. Implement 'protect' subcommand
- Accept generation number as argument
- Load current protected list
- Add generation to protected list
- Save updated protected list
- Provide user feedback

### 9. Implement 'unprotect' subcommand
- Accept generation number as argument
- Load current protected list
- Remove generation from protected list
- Save updated protected list
- Provide user feedback

### 10. Implement 'clean' subcommand core logic
- Get all available generations from NixOsCommandRunner
- Get current generation
- Load protected generations list
- Determine which generations to delete:
  - Exclude current generation
  - Exclude protected generations
  - (Later) Optionally exclude last N generations
- Invoke `delete_generations()` with the calculated list

### 11. Implement --keep-last N flag for clean command
- Add `--keep-last N` flag to clean subcommand
- Sort generations and identify the most recent N generations
- Add these to the exclusion list when determining deletable generations

## Phase 4: Testing & Polish

### 12. Write unit tests for generation selection logic
- Test with various scenarios:
  - No protected generations
  - All generations protected
  - Mixed protected/unprotected
  - `--keep-last N` with different values
  - Current generation handling
- Use mock NixOsCommandRunner

### 13. Write integration tests for CLI commands
- Test full command flows with mocked command runner
- Verify correct commands are invoked
- Test state persistence across multiple operations
- Test error handling (invalid generation numbers, permission errors)

### 14. Add error handling and user-friendly error messages
- Handle missing permissions
- Handle invalid generation numbers
- Handle NixOS command failures
- Provide clear, actionable error messages
- Consider exit codes for scriptability

### 15. Manual testing on real NixOS system (if available)
- Test on an actual NixOS installation
- Verify correct interaction with real `nix-env` commands
- Test with various system configurations
- Document any platform-specific issues

## Success Criteria

- [ ] All automated tests pass
- [ ] Tool correctly identifies deletable generations based on rules
- [ ] Protection state persists correctly across invocations
- [ ] Tool invokes correct NixOS commands with correct arguments
- [ ] Error messages are clear and helpful
- [ ] Code is well-documented and follows Rust best practices
- [ ] Manual testing on NixOS confirms expected behavior
