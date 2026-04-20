mod helpers;
mod shell_manager;

use std::{
    io::{self, Write},
    process::{Command, Stdio},
};

use anyhow::{Result, bail};

use crate::{
    handlers::helpers::{filter_and_display_snippets, get_confirmation, get_target_id},
    models::{Shell, Snippet},
    storage::{load_snippets, save_snippets},
};

pub fn handle_add(
    content: String,
    tag: Option<String>,
    description: Option<String>,
    shell_type: Option<String>,
) -> Result<()> {
    let new_snippet = Snippet::new(content, tag, description, shell_type.map(Shell::new))?;

    let mut snippets = load_snippets()?;
    snippets.push(new_snippet);

    save_snippets(&snippets)?;
    println!("Saved Snippet successfully");
    Ok(())
}

pub fn handle_list(search_term: Option<String>, verbose: bool) -> Result<()> {
    let snippets = load_snippets()?;
    filter_and_display_snippets(&snippets, search_term, verbose);
    Ok(())
}

pub fn handle_remove(search_term: Option<String>, verbose: bool) -> Result<()> {
    let mut snippets = load_snippets()?;
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
        save_snippets(&snippets)?;
        println!("Removed snippet successfully");
    } else {
        println!("Aborted!");
    }

    Ok(())
}

pub fn handle_execute(
    search_term: Option<String>,
    _shell_type: Option<String>,
    verbose: bool,
) -> Result<()> {
    let snippets = load_snippets()?;
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

    if get_confirmation()? {
        let Some(snippet) = snippets.iter().find(|s| s.id == target_id) else {
            bail!("Couldn't find any snippet with the id: {}", target_id);
        };
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(&snippet.content)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;

        let status = child.wait()?;

        if !status.success() {
            eprintln!("Command exited with error: {}", status);
        }
    } else {
        println!("Aborted!");
    }

    Ok(())
}
