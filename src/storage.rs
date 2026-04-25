//! Storage abstraction layer for snippet persistence
//!
//! This module defines a trait-based storage system tgat allows different
//! backends (filesystem, memory, etc.) to be swapped without
//! changing handler code.

use crate::errors::Result;
use crate::models::Snippet;

pub mod file_storage;
pub mod memory_storage;

/// Trait that all storage backends must implement.
///
/// This is the contract: any type implementing SnippetStorage can load
/// and save snippets. Handlers don't care how data is accessed
/// They just call these methods
pub trait SnippetStorage {
    /// Load all snippets from storage.
    ///
    /// Returns:
    /// - Ok(Vec<Snippet>) with snippets (empty vec if none exist yet)
    /// - Err if storage is unreachable or corrupted
    fn load(&self) -> Result<Vec<Snippet>>;

    fn save(&self, snippets: &[Snippet]) -> Result<()>;
}
