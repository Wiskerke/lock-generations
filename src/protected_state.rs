use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Protected generations state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectedState {
    pub protected_generations: HashSet<u32>,
}

impl ProtectedState {
    /// Create a new empty ProtectedState
    pub fn new() -> Self {
        Self {
            protected_generations: HashSet::new(),
        }
    }

    /// Load protected state from the default config file
    /// Returns empty state if file doesn't exist
    pub fn load() -> Result<Self> {
        let path = Self::default_config_path()?;
        Self::load_from(&path)
    }

    /// Load protected state from a specific path
    /// Returns empty state if file doesn't exist
    pub fn load_from(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::new());
        }

        let contents = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let state: ProtectedState = serde_json::from_str(&contents)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

        Ok(state)
    }

    /// Save protected state to the default config file
    pub fn save(&self) -> Result<()> {
        let path = Self::default_config_path()?;
        self.save_to(&path)
    }

    /// Save protected state to a specific path
    pub fn save_to(&self, path: &Path) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create config directory: {}", parent.display())
            })?;
        }

        // Serialize to JSON with pretty printing
        let contents = serde_json::to_string_pretty(self)
            .context("Failed to serialize protected state")?;

        // Write atomically by writing to a temp file and renaming
        let tmp_path = path.with_extension("tmp");
        fs::write(&tmp_path, contents)
            .with_context(|| format!("Failed to write temp file: {}", tmp_path.display()))?;

        fs::rename(&tmp_path, path)
            .with_context(|| format!("Failed to save config file: {}", path.display()))?;

        Ok(())
    }

    /// Add a generation to the protected list
    pub fn protect(&mut self, generation: u32) -> bool {
        self.protected_generations.insert(generation)
    }

    /// Remove a generation from the protected list
    pub fn unprotect(&mut self, generation: u32) -> bool {
        self.protected_generations.remove(&generation)
    }

    /// Check if a generation is protected
    #[allow(dead_code)]
    pub fn is_protected(&self, generation: u32) -> bool {
        self.protected_generations.contains(&generation)
    }

    /// Get the default config file path
    /// Uses XDG_CONFIG_HOME if set, otherwise ~/.config
    fn default_config_path() -> Result<PathBuf> {
        let config_dir = if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
            PathBuf::from(xdg_config)
        } else if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(".config")
        } else {
            anyhow::bail!("Could not determine config directory (HOME not set)");
        };

        Ok(config_dir
            .join("lock-generations")
            .join("protected.json"))
    }
}

impl Default for ProtectedState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_new_protected_state() {
        let state = ProtectedState::new();
        assert!(state.protected_generations.is_empty());
    }

    #[test]
    fn test_protect_unprotect() {
        let mut state = ProtectedState::new();

        assert!(state.protect(5));
        assert!(state.is_protected(5));
        assert!(!state.is_protected(3));

        assert!(!state.protect(5)); // Already protected
        assert!(state.unprotect(5));
        assert!(!state.is_protected(5));
        assert!(!state.unprotect(5)); // Already unprotected
    }

    #[test]
    fn test_save_and_load() {
        let tmp_dir = TempDir::new().unwrap();
        let config_path = tmp_dir.path().join("protected.json");

        let mut state = ProtectedState::new();
        state.protect(1);
        state.protect(5);
        state.protect(10);

        state.save_to(&config_path).unwrap();

        let loaded = ProtectedState::load_from(&config_path).unwrap();
        assert_eq!(loaded.protected_generations.len(), 3);
        assert!(loaded.is_protected(1));
        assert!(loaded.is_protected(5));
        assert!(loaded.is_protected(10));
    }

    #[test]
    fn test_load_nonexistent_file() {
        let tmp_dir = TempDir::new().unwrap();
        let config_path = tmp_dir.path().join("nonexistent.json");

        let state = ProtectedState::load_from(&config_path).unwrap();
        assert!(state.protected_generations.is_empty());
    }
}
