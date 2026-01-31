mod command_runner;
#[cfg(test)]
mod mock_runner;
mod protected_state;
mod real_runner;

use anyhow::Result;
use clap::{Parser, Subcommand};
use command_runner::NixOsCommandRunner;
use protected_state::ProtectedState;
use real_runner::RealNixOsRunner;
use std::collections::HashSet;

#[derive(Parser)]
#[command(name = "lock-generations")]
#[command(about = "Manage NixOS system generations with selective protection", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add protection to a generation to prevent deletion
    Protect {
        /// Generation number to protect
        generation: u32,
    },
    /// Remove protection from a generation
    Unprotect {
        /// Generation number to unprotect
        generation: u32,
    },
    /// Remove all unprotected generations (except current)
    Clean {
        /// Keep the last N most recent generations
        #[arg(long)]
        keep_last: Option<usize>,
        /// Show what would be done without actually deleting
        #[arg(long)]
        dry_run: bool,
    },
    /// List all protected generations
    List,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let runner = RealNixOsRunner::new();

    match cli.command {
        Commands::Protect { generation } => protect_generation(generation),
        Commands::Unprotect { generation } => unprotect_generation(generation),
        Commands::Clean { keep_last, dry_run } => clean_generations(&runner, keep_last, dry_run),
        Commands::List => list_protected(),
    }
}

fn protect_generation(generation: u32) -> Result<()> {
    let mut state = ProtectedState::load()?;

    if state.protect(generation) {
        state.save()?;
        println!("Protected generation {}", generation);
    } else {
        println!("Generation {} is already protected", generation);
    }

    Ok(())
}

fn unprotect_generation(generation: u32) -> Result<()> {
    let mut state = ProtectedState::load()?;

    if state.unprotect(generation) {
        state.save()?;
        println!("Unprotected generation {}", generation);
    } else {
        println!("Generation {} was not protected", generation);
    }

    Ok(())
}

fn clean_generations(runner: &dyn NixOsCommandRunner, keep_last: Option<usize>, dry_run: bool) -> Result<()> {
    let state = ProtectedState::load()?;
    let current = runner.get_current_generation()?;
    let all_generations = runner.list_generations()?;

    // Collect all generation numbers
    let mut gen_numbers: Vec<u32> = all_generations.iter().map(|g| g.number).collect();
    gen_numbers.sort_unstable();

    // Determine which generations to keep
    let mut keep: HashSet<u32> = HashSet::new();

    // Always keep current generation
    keep.insert(current);

    // Keep protected generations
    for &protected in &state.protected_generations {
        keep.insert(protected);
    }

    // Keep last N generations if specified
    if let Some(n) = keep_last {
        let start_index = gen_numbers.len().saturating_sub(n);
        for &gen_num in &gen_numbers[start_index..] {
            keep.insert(gen_num);
        }
    }

    // Determine which generations to delete
    let to_delete: Vec<u32> = gen_numbers
        .iter()
        .filter(|&&g| !keep.contains(&g))
        .copied()
        .collect();

    if to_delete.is_empty() {
        println!("No generations to delete");
        return Ok(());
    }

    if dry_run {
        println!("[DRY RUN] Would delete {} generation(s): {:?}", to_delete.len(), to_delete);
        println!();
        println!("Command that would be executed:");
        let gen_list: Vec<String> = to_delete.iter().map(|g| g.to_string()).collect();
        let gen_arg = gen_list.join(" ");
        println!("  nix-env --delete-generations {} -p /nix/var/nix/profiles/system", gen_arg);
    } else {
        println!("Deleting {} generation(s): {:?}", to_delete.len(), to_delete);
        runner.delete_generations(&to_delete)?;
        println!("Successfully deleted {} generation(s)", to_delete.len());
    }

    Ok(())
}

fn list_protected() -> Result<()> {
    let state = ProtectedState::load()?;

    if state.protected_generations.is_empty() {
        println!("No protected generations");
    } else {
        let mut protected: Vec<u32> = state.protected_generations.iter().copied().collect();
        protected.sort_unstable();
        println!("Protected generations:");
        for gen_num in protected {
            println!("  {}", gen_num);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock_runner::MockNixOsRunner;

    #[test]
    fn test_clean_no_protected() {
        let runner = MockNixOsRunner::with_current(vec![1, 2, 3, 4, 5], 5);
        clean_generations(&runner, None, false).unwrap();

        // Should delete all except current (5)
        assert!(runner.was_deleted(1));
        assert!(runner.was_deleted(2));
        assert!(runner.was_deleted(3));
        assert!(runner.was_deleted(4));
        assert!(!runner.was_deleted(5));
    }

    #[test]
    fn test_clean_with_keep_last() {
        let runner = MockNixOsRunner::with_current(vec![1, 2, 3, 4, 5], 5);
        clean_generations(&runner, Some(2), false).unwrap();

        // Should delete 1, 2, 3 and keep 4, 5 (last 2)
        assert!(runner.was_deleted(1));
        assert!(runner.was_deleted(2));
        assert!(runner.was_deleted(3));
        assert!(!runner.was_deleted(4));
        assert!(!runner.was_deleted(5));
    }

    #[test]
    fn test_clean_dry_run() {
        let runner = MockNixOsRunner::with_current(vec![1, 2, 3, 4, 5], 5);
        clean_generations(&runner, None, true).unwrap();

        // Dry run should not delete anything
        assert!(!runner.was_deleted(1));
        assert!(!runner.was_deleted(2));
        assert!(!runner.was_deleted(3));
        assert!(!runner.was_deleted(4));
        assert!(!runner.was_deleted(5));
    }
}
