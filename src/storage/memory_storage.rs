//! In-memory storage implementation.
//!
//! Stores snippets in RAM only. Perfect for unit tests - no disk I/O,
//! no side effects, no real user data modified.

use std::cell::RefCell;

use anyhow::Result;

use super::SnippetStorage;
use crate::models::Snippet;

/// Storage backend that keeps everything in RAM.
///
/// Uses REfCell for interior mutability - allows us to call `save()` with
/// `&self` (immutable reference) but still modify the inner Vec.
///
/// Why RefCell and not Mutex? Because testsw are single-threaded.
/// Mutex is for multi-threaded code. RefCell is simpler, faster, and sufficient here.
pub struct MemoryStorage {
    /// The actual data, protected by RefCell for interior mutability.
    ///
    /// RefCell::new() creates a RefCell wrapping a Vec.
    /// Later, we'll use .borrow and .borrow_mut() to access it.
    snippets: RefCell<Vec<Snippet>>,
}

impl MemoryStorage {
    /// Create a new empty MemoryStorage.
    pub fn new() -> Self {
        MemoryStorage {
            snippets: RefCell::new(Vec::new()),
        }
    }

    /// Create a MemoryStorage pre-populated with snippets (useful for setup in tests).
    pub fn with_snippets(snippets: Vec<Snippet>) -> Self {
        MemoryStorage {
            snippets: RefCell::new(snippets),
        }
    }

    /// Get a clone of all stored snippets (for inspection in tests).
    ///
    /// Why clone? we return a Vec<Snippet> instead of &Vec<Snippet> because
    /// we can't hold a borrow across a function boundary safely.
    pub fn get_all(&self) -> Vec<Snippet> {
        self.snippets.borrow().clone()
    }
}

impl SnippetStorage for MemoryStorage {
    fn load(&self) -> Result<Vec<Snippet>> {
        Ok(self.snippets.borrow().clone())
    }

    fn save(&self, snippets: &[Snippet]) -> Result<()> {
        *self.snippets.borrow_mut() = snippets.to_vec();
        Ok(())
    }
}

impl Clone for MemoryStorage {
    fn clone(&self) -> Self {
        MemoryStorage::with_snippets(self.get_all())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_storage_round_trip() {
        let storage = MemoryStorage::new();

        let snippet = Snippet::new(
            "echo hello".to_string(),
            Some("bash".to_string()),
            None,
            None,
        )
        .unwrap();

        storage.save(&[snippet.clone()]).unwrap();

        let loaded = storage.load().unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].content, "echo hello");
    }

    #[test]
    fn test_memory_storage_empty_initially() {
        let storage = MemoryStorage::new();
        let loaded = storage.load().unwrap();
        assert!(loaded.is_empty());
    }
}
