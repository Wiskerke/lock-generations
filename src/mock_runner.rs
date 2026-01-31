use crate::command_runner::{Generation, NixOsCommandRunner};
use anyhow::Result;
use std::cell::RefCell;
use std::collections::HashSet;

/// Mock implementation of NixOsCommandRunner for testing
/// Simulates NixOS behavior without executing real commands
pub struct MockNixOsRunner {
    generations: Vec<u32>,
    current_generation: u32,
    deleted_generations: RefCell<HashSet<u32>>,
    fail_on_delete: bool,
}

impl MockNixOsRunner {
    /// Create a new MockNixOsRunner with the given generations
    /// The last generation in the list is treated as the current generation
    pub fn new(generations: Vec<u32>) -> Self {
        let current = *generations.last().unwrap_or(&1);
        Self {
            generations,
            current_generation: current,
            deleted_generations: RefCell::new(HashSet::new()),
            fail_on_delete: false,
        }
    }

    /// Create a new MockNixOsRunner with specified current generation
    pub fn with_current(generations: Vec<u32>, current: u32) -> Self {
        Self {
            generations,
            current_generation: current,
            deleted_generations: RefCell::new(HashSet::new()),
            fail_on_delete: false,
        }
    }

    /// Configure the mock to fail when delete_generations is called
    pub fn fail_on_delete(mut self) -> Self {
        self.fail_on_delete = true;
        self
    }

    /// Get the set of deleted generation numbers (for test verification)
    pub fn get_deleted_generations(&self) -> HashSet<u32> {
        self.deleted_generations.borrow().clone()
    }

    /// Check if a generation was deleted
    pub fn was_deleted(&self, generation: u32) -> bool {
        self.deleted_generations.borrow().contains(&generation)
    }
}

impl NixOsCommandRunner for MockNixOsRunner {
    fn list_generations(&self) -> Result<Vec<Generation>> {
        let deleted = self.deleted_generations.borrow();
        Ok(self
            .generations
            .iter()
            .filter(|g| !deleted.contains(g))
            .map(|&number| Generation { number })
            .collect())
    }

    fn get_current_generation(&self) -> Result<u32> {
        Ok(self.current_generation)
    }

    fn delete_generations(&self, generations: &[u32]) -> Result<()> {
        if self.fail_on_delete {
            anyhow::bail!("Simulated deletion failure");
        }

        // Prevent deletion of current generation
        if generations.contains(&self.current_generation) {
            anyhow::bail!(
                "Cannot delete current generation: {}",
                self.current_generation
            );
        }

        // Mark generations as deleted
        let mut deleted = self.deleted_generations.borrow_mut();
        for &gen_num in generations {
            deleted.insert(gen_num);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_list_generations() {
        let runner = MockNixOsRunner::new(vec![1, 2, 3, 5, 7]);
        let gens = runner.list_generations().unwrap();
        assert_eq!(gens.len(), 5);
        assert_eq!(gens[0].number, 1);
        assert_eq!(gens[4].number, 7);
    }

    #[test]
    fn test_mock_current_generation() {
        let runner = MockNixOsRunner::with_current(vec![1, 2, 3], 2);
        assert_eq!(runner.get_current_generation().unwrap(), 2);
    }

    #[test]
    fn test_mock_delete_generations() {
        let runner = MockNixOsRunner::with_current(vec![1, 2, 3, 4, 5], 5);
        runner.delete_generations(&[1, 3]).unwrap();

        assert!(runner.was_deleted(1));
        assert!(!runner.was_deleted(2));
        assert!(runner.was_deleted(3));

        let remaining = runner.list_generations().unwrap();
        assert_eq!(remaining.len(), 3);
    }

    #[test]
    fn test_mock_cannot_delete_current() {
        let runner = MockNixOsRunner::with_current(vec![1, 2, 3], 3);
        let result = runner.delete_generations(&[3]);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Cannot delete current generation"));
    }

    #[test]
    fn test_mock_fail_on_delete() {
        let runner = MockNixOsRunner::new(vec![1, 2, 3]).fail_on_delete();
        let result = runner.delete_generations(&[1]);
        assert!(result.is_err());
    }
}
