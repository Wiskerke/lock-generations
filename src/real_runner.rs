use crate::command_runner::{Generation, NixOsCommandRunner};
use anyhow::{Context, Result};
use std::process::Command;

/// Real implementation of NixOsCommandRunner that executes actual nix-env commands
pub struct RealNixOsRunner {
    profile_path: String,
}

impl RealNixOsRunner {
    /// Create a new RealNixOsRunner with the default system profile path
    pub fn new() -> Self {
        Self {
            profile_path: "/nix/var/nix/profiles/system".to_string(),
        }
    }

    /// Create a new RealNixOsRunner with a custom profile path (useful for testing)
    #[allow(dead_code)]
    pub fn with_profile(profile_path: String) -> Self {
        Self { profile_path }
    }
}

impl Default for RealNixOsRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl NixOsCommandRunner for RealNixOsRunner {
    fn list_generations(&self) -> Result<Vec<Generation>> {
        // Execute: nix-env --list-generations -p /nix/var/nix/profiles/system
        let output = Command::new("nix-env")
            .arg("--list-generations")
            .arg("-p")
            .arg(&self.profile_path)
            .output()
            .context("Failed to execute nix-env --list-generations")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("nix-env --list-generations failed: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut generations = Vec::new();

        // Parse output format:
        //   1   2024-01-15 10:30:45
        //   2   2024-01-16 14:20:10
        //   3   2024-01-17 09:15:30   (current)
        for line in stdout.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Extract the generation number (first token)
            if let Some(number_str) = line.split_whitespace().next() {
                if let Ok(number) = number_str.parse::<u32>() {
                    generations.push(Generation { number });
                }
            }
        }

        Ok(generations)
    }

    fn get_current_generation(&self) -> Result<u32> {
        // Execute: nix-env --list-generations -p /nix/var/nix/profiles/system
        let output = Command::new("nix-env")
            .arg("--list-generations")
            .arg("-p")
            .arg(&self.profile_path)
            .output()
            .context("Failed to execute nix-env --list-generations")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("nix-env --list-generations failed: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Find the line with "(current)" marker
        for line in stdout.lines() {
            if line.contains("(current)") {
                if let Some(number_str) = line.trim().split_whitespace().next() {
                    if let Ok(number) = number_str.parse::<u32>() {
                        return Ok(number);
                    }
                }
            }
        }

        anyhow::bail!("Could not determine current generation")
    }

    fn delete_generations(&self, generations: &[u32]) -> Result<()> {
        if generations.is_empty() {
            return Ok(());
        }

        // Build the generation list string: "1 2 3 4"
        let gen_list: Vec<String> = generations.iter().map(|g| g.to_string()).collect();
        let gen_arg = gen_list.join(" ");

        // Execute: nix-env --delete-generations 1 2 3 -p /nix/var/nix/profiles/system
        let output = Command::new("nix-env")
            .arg("--delete-generations")
            .arg(&gen_arg)
            .arg("-p")
            .arg(&self.profile_path)
            .output()
            .context("Failed to execute nix-env --delete-generations")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("nix-env --delete-generations failed: {}", stderr);
        }

        Ok(())
    }
}
