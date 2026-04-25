mod helpers;

use std::{
    io::{self, Write},
    sync::Arc,
};

use anyhow::{Context, Result, anyhow, bail};

use crate::{
    handlers::helpers::{
        execute_snippet, filter_and_display_snippets, get_confirmation, get_target_id,
        update_default_shell,
    },
    models::{Shell, Snippet},
    storage::SnippetStorage,
};

/// Add a new snippet.
pub fn handle_add(
    content: String,
    tag: Option<String>,
    description: Option<String>,
    shell_type: Option<String>,
    storage: Arc<dyn SnippetStorage>,
) -> Result<()> {
    let new_snippet = Snippet::new(content, tag, description, shell_type.map(Shell::new))?;

    let mut snippets = storage.load()?;
    snippets.push(new_snippet);

    storage.save(&snippets)?;
    println!("Saved Snippet successfully");
    Ok(())
}

/// List and search snippets.
pub fn handle_list(
    search_term: Option<String>,
    verbose: bool,
    storage: Arc<dyn SnippetStorage>,
) -> Result<()> {
    let snippets = storage.load()?;
    filter_and_display_snippets(&snippets, search_term, verbose);
    Ok(())
}

/// Remove a snippet interactively.
pub fn handle_remove(
    search_term: Option<String>,
    verbose: bool,
    storage: Arc<dyn SnippetStorage>,
) -> Result<()> {
    let mut snippets = storage.load()?;
    let filtered = filter_and_display_snippets(&snippets, search_term, verbose);

    if filtered.is_empty() {
        return Ok(());
    }

    let Some(target_id) = get_target_id(&filtered)? else {
        return Ok(());
    };

    print!(
        "Confirm deletion of the snippet with ID: {}? (y/N): ",
        target_id
    );
    io::stdout().flush()?;

    if get_confirmation()? {
        snippets.retain(|s| s.id != target_id);
        storage.save(&snippets)?;
        println!("Removed snippet successfully");
    } else {
        println!("Aborted!");
    }

    Ok(())
}

/// Execute a snippet interactively.
pub fn handle_execute(
    search_term: Option<String>,
    shell_type: Option<String>,
    verbose: bool,
    storage: Arc<dyn SnippetStorage>,
) -> Result<()> {
    let mut snippets = storage.load()?;
    let filtered = filter_and_display_snippets(&snippets, search_term, verbose);

    if filtered.is_empty() {
        return Ok(());
    }

    let Some(target_id) = get_target_id(&filtered)? else {
        return Ok(());
    };

    print!(
        "Confirm execution of the snippet with ID: {}? (y/N): ",
        target_id
    );
    io::stdout().flush()?;

    if !get_confirmation()? {
        println!("Aborted!");
        return Ok(());
    }

    let snippet = snippets
        .iter()
        .find(|s| s.id == target_id)
        .ok_or_else(|| anyhow!("Couldn't find any snippet with the id: {}", target_id))?;

    let shell = match &shell_type {
        Some(name) => Shell::new(name.clone()),
        None => snippet
            .shell
            .as_ref()
            .context("No shell specified for the current snippet")?
            .clone(),
    };

    if !shell.is_supported {
        bail!(
            "'{}' is not a valid shell or is not currently supported",
            shell.name
        );
    }

    execute_snippet(&shell, &snippet.content)?;
    if let Some(_) = shell_type {
        update_default_shell(&mut snippets, shell, target_id, storage.clone())?;
    }

    Ok(())
}
