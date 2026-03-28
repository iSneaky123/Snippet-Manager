use std::{fs, io::ErrorKind, path::PathBuf};

use anyhow::{Context, Result};

use crate::models::Snippet;

fn get_storage_path() -> Result<PathBuf> {
    let mut path = dirs::data_dir()
        .context("Could not find the standard data directory on your Operating System")?;

    path.push("snip");
    path.push("snippets.json");

    Ok(path)
}

pub fn load_snippets() -> Result<Vec<Snippet>> {
    let path = get_storage_path()?;

    let contents = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(vec![]),
        Err(e) => return Err(e).context(format!("Failed to load snippets file at {:?}", path)),
    };

    let snippets = serde_json::from_str(&contents)
        .context("The snippets file is corrupted or not valid JSON")?;

    Ok(snippets)
}

pub fn save_snippets(snippets: &[Snippet]) -> Result<()> {
    let path = get_storage_path()?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to write snippets to disk at {:?}", path))?;
    };

    let mut json =
        serde_json::to_string(snippets).context("Failed to convert snippets to JSON Format")?;
    json.push('\n');

    fs::write(&path, &json)
        .with_context(|| format!("Failed to write snippets to disk at {:?}", path))?;

    Ok(())
}
