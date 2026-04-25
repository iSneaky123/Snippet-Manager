use std::{
    io::{self, Write},
    process::{Command, Stdio},
    sync::Arc,
};

use nucleo::{Config, Matcher, Utf32Str};

use crate::{
    errors::{Result, SnipError},
    models::{Shell, Snippet},
};

pub(super) fn print_snippets(snippets: &[&Snippet], verbose: bool) {
    let mut current_tag: Option<String> = None;

    for (idx, &s) in snippets.iter().enumerate() {
        let display_tag = s.display_tag();
        if current_tag.as_deref() != Some(&display_tag) {
            println!("\n[ {} ]", display_tag);
            current_tag = Some(display_tag);
        }

        println!("{}. ID: {}", idx + 1, s.id);

        for line in s.content.lines() {
            println!("    {}", line);
        }

        if verbose && !s.desc_or_default().trim().is_empty() {
            println!("Description:");
            println!("    {}", s.desc_or_default());
        }

        if verbose && s.shell.is_some() {
            if let Some(shell) = &s.shell {
                println!("Default shell: {}", shell.name);
            }
        }

        if idx + 1 < snippets.len() {
            println!();
        }
    }
}

pub(super) fn filter_and_sort_snippets<'a>(
    snippets: &'a [Snippet],
    search_term: Option<String>,
) -> Vec<&'a Snippet> {
    let Some(search_term) = search_term else {
        return sort_by_tag(snippets);
    };

    const AGRESSIVE_SEARCH: u16 = 49;
    const STRICT_SEARCH: u16 = 74;

    let mut matcher = Matcher::new(Config::DEFAULT);

    let mut query_buf = vec![];
    let query_utf32 = Utf32Str::new(&search_term, &mut query_buf);
    let mut haystack_buf = vec![];

    let results: Vec<&Snippet> = snippets
        .iter()
        .filter(|s| {
            let fields = [
                (s.tag_or_default(), AGRESSIVE_SEARCH),
                (s.desc_or_default(), AGRESSIVE_SEARCH),
                (s.content.as_str(), AGRESSIVE_SEARCH),
                (s.id.as_str(), STRICT_SEARCH),
            ];

            fields.iter().any(|(text, min_score)| {
                if text.trim().is_empty() {
                    return false;
                }

                haystack_buf.clear();
                let haystack = Utf32Str::new(text, &mut haystack_buf);
                matcher
                    .fuzzy_match(haystack, query_utf32)
                    .map(|score| score >= *min_score)
                    .unwrap_or(false)
            })
        })
        .collect();

    sort_by_tag(results)
}

pub(super) fn get_target_id(snippets: &[&Snippet]) -> Result<Option<String>> {
    print!("\nEnter Sr No. to select (or 'q' to cancel): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim();
    if input == "q" {
        println!("Aborted!");
        return Ok(None);
    }

    let sr_no: usize = input.parse().map_err(|_| {
        SnipError::InvalidInput(format!("Sr No. must be a number, got '{}'", input))
    })?;

    if !(1..=snippets.len()).contains(&sr_no) {
        return Err(SnipError::InvalidInput(format!(
            "Sr No. {} is out of range (expected 1-{})",
            sr_no,
            snippets.len()
        )));
    }

    // Clone the ID here because:
    // 1. The caller may need the mutable reference to update the snippet.
    // 2. Rust prevents holding both immutable and mutable refs to the same data.
    Ok(Some(snippets[sr_no - 1].id.clone()))
}

pub(super) fn get_confirmation() -> Result<bool> {
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm)?;

    Ok(confirm.trim().eq_ignore_ascii_case("y"))
}

pub(super) fn filter_and_display_snippets<'a>(
    snippets: &'a [Snippet],
    search_term: Option<String>,
    verbose: bool,
) -> Vec<&'a Snippet> {
    let filtered = filter_and_sort_snippets(snippets, search_term);

    if filtered.is_empty() {
        println!("No snippets were found");
    } else {
        print_snippets(&filtered, verbose);
    }

    filtered
}

fn sort_by_tag<'a, S>(snippets: S) -> Vec<&'a Snippet>
where
    S: IntoIterator<Item = &'a Snippet>,
{
    let mut items: Vec<_> = snippets
        .into_iter()
        .map(|s| (s, s.tag_or_default().to_lowercase()))
        .collect();

    items.sort_by(|a, b| a.1.cmp(&b.1));

    items.into_iter().map(|(s, _)| s).collect()
}

pub(super) fn execute_snippet(shell: &Shell, content: &str) -> Result<()> {
    let status = Command::new(&shell.name)
        .arg(&shell.command_flag)
        .arg(&content)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;

    if !status.success() {
        eprint!("Command exited with error: {}", status);
    }

    Ok(())
}

pub(super) fn update_default_shell(
    snippets: &mut [Snippet],
    shell: Shell,
    target_id: String,
    storage: Arc<dyn crate::storage::SnippetStorage>,
) -> Result<()> {
    print!(
        "Do you want to change the default shell for the selected snippet with '{}'? (y/N): ",
        shell.name
    );
    io::stdout().flush()?;

    if get_confirmation()? {
        let snippet = snippets
            .iter_mut()
            .find(|s| s.id == target_id)
            .ok_or_else(|| SnipError::SnippetNotFound(target_id))?;

        snippet.shell = Some(shell);
        storage.save(snippets)?;
        println!("Updated snippet successfully");
    }

    Ok(())
}
