use anyhow::Result;

/// Represents a NixOS generation with its number and metadata
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Generation {
    pub number: u32,
}

/// Trait for abstracting NixOS command execution
/// This allows for both real command execution and mocked behavior for testing
pub trait NixOsCommandRunner {
    /// List all available NixOS system generations
    fn list_generations(&self) -> Result<Vec<Generation>>;

    /// Get the current active generation number
    fn get_current_generation(&self) -> Result<u32>;

    /// Delete the specified generations using nix-env commands
    fn delete_generations(&self, generations: &[u32]) -> Result<()>;
}
