//! Filesystem-base storage implementation.
//!
//! Persists snuppets to a JSON File in the user's data directory.
//! THis is what gets used in production

use std::{fs, io::ErrorKind, path::PathBuf};

use anyhow::{Context, Result};

use super::SnippetStorage;
use crate::models::Snippet;

/// Storage backend that persists to filesystem.
pub struct FileStorage {
    path: PathBuf,
}

impl FileStorage {
    /// Create a new FileStorage using the standard data directory.
    ///
    /// This will use `~/.local/share/snip/snippets.json` on Linux,
    /// ~/Library/Application Support/snip/snippets.json1 on macOS, etc.
    pub fn new() -> Result<Self> {
        let mut path = dirs::data_dir()
            .context("Could not find the standard data directory on your Operating System")?;

        path.push("snip");
        path.push("snippets.json");

        Ok(FileStorage { path })
    }

    /// Create a FileStorage at a custom path (useful for testing with real files).
    pub fn with_path(path: PathBuf) -> Self {
        FileStorage { path }
    }

    /// Get the path where snippets are stored.
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

impl SnippetStorage for FileStorage {
    fn load(&self) -> Result<Vec<Snippet>> {
        let contents = match fs::read_to_string(&self.path) {
            Ok(c) => c,
            Err(e) if e.kind() == ErrorKind::NotFound => {
                return Ok(vec![]);
            }
            Err(e) => {
                return Err(e).context(format!("Failed to load snippets file at {:?}", self.path));
            }
        };

        serde_json::from_str(&contents).context("The snippets file is corrupted or not valid JSON")
    }

    fn save(&self, snippets: &[Snippet]) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir(parent)
                .with_context(|| format!("Failed to write snippets to disk at {:?}", self.path))?;
        }

        let mut json =
            serde_json::to_string(snippets).context("Failed to convert snippets to JSON format")?;
        json.push('\n');

        fs::write(&self.path, &json)
            .with_context(|| format!("Failed to write snippets to dist at {:?}", self.path))
    }
}
