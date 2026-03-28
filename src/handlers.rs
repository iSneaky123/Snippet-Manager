use std::io::{self, Write};

use anyhow::{Result, anyhow, bail};
use nucleo::{Config, Matcher, Utf32Str};

use crate::{
    models::Snippet,
    storage::{load_snippets, save_snippets},
};

pub fn handle_add(content: String, tag: String, description: String) -> Result<()> {
    let new_snippet = Snippet::new(content, tag, description);

    let mut snippets = load_snippets()?;
    snippets.push(new_snippet);

    save_snippets(&snippets)?;
    println!("Saved Snippet suffessfully");
    Ok(())
}

pub fn handle_list(search_term: &str, verbose: bool) -> Result<()> {
    let snippets = load_snippets()?;
    let filtered = filter_and_sort_snippets(&search_term, &snippets);

    if filtered.is_empty() {
        println!("No snippets were found");
        return Ok(());
    }

    print_snippets(&filtered, verbose);
    Ok(())
}

pub fn handle_remove(search_term: &str, verbose: bool) -> Result<()> {
    let mut snippets = load_snippets()?;
    let filtered = filter_and_sort_snippets(&search_term, &snippets);

    if filtered.is_empty() {
        println!("No snippets were found");
        return Ok(());
    }

    let all_snippet_refs: Vec<&Snippet> = snippets.iter().collect();
    print_snippets(&all_snippet_refs, verbose);

    print!("\nEnter Sr No. to delete (or 'q' to cancel): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim();
    if input == "q" {
        println!("Aborted!");
        return Ok(());
    }

    let sr_no: usize = input
        .parse()
        .map_err(|_| anyhow!("'{}' is not a valid number", input))?;

    if sr_no <= 0 || sr_no > filtered.len() {
        bail!("Sr No. {} is out of range (1-{})", sr_no, filtered.len());
    }

    let target_id = filtered[sr_no - 1].id.clone();

    print!("Confirm deleteion of ID: {}? (y/N): ", target_id);
    io::stdout().flush()?;

    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm)?;

    if confirm.trim().to_lowercase() == "y" {
        snippets.retain(|s| s.id != target_id);
        save_snippets(&snippets)?;
        println!("Removed snippet successfully");
    } else {
        println!("Aborted!");
    };

    Ok(())
}

fn print_snippets(snippets: &[&Snippet], verbose: bool) {
    let mut current_tag: Option<String> = None;

    for (idx, &s) in snippets.iter().enumerate() {
        let display_tag = if s.tag.trim().is_empty() {
            "UNTAGGED".to_string()
        } else {
            s.tag.to_ascii_uppercase()
        };

        if current_tag.as_deref() != Some(&display_tag) {
            println!("\n[ {} ]", display_tag);
            current_tag = Some(display_tag);
        }

        println!("{}. ID: {}", idx + 1, s.id);

        for line in s.content.lines() {
            println!("    {}", line);
        }

        if verbose && !s.description.trim().is_empty() {
            println!("Description:");
            println!("    {}", s.description);
        }

        if idx < snippets.len() - 1 {
            println!();
        }
    }
}

fn filter_and_sort_snippets<'a>(search_term: &str, snippets: &'a [Snippet]) -> Vec<&'a Snippet> {
    if search_term.trim().is_empty() {
        let mut all: Vec<&Snippet> = snippets.iter().collect();
        all.sort_by_key(|s| s.tag.to_lowercase());
        return all;
    }

    let mut matcher = Matcher::new(Config::DEFAULT);

    let mut query_buf = vec![];
    let query_utf32 = Utf32Str::new(search_term, &mut query_buf);

    let mut results: Vec<&Snippet> = snippets
        .iter()
        .filter(|s| {
            let fields = [
                (s.tag.as_str(), 50),
                (s.description.as_str(), 75),
                (s.content.as_str(), 75),
                (s.id.as_str(), 25),
            ];

            fields.iter().any(|(text, min_score)| {
                if text.trim().is_empty() {
                    return false;
                }

                let mut haystack_buf = vec![];
                let haystack = Utf32Str::new(text, &mut haystack_buf);
                matcher
                    .fuzzy_match(haystack, query_utf32)
                    .map(|score| score >= *min_score)
                    .unwrap_or(false)
            })
        })
        .collect();

    results.sort_by_key(|s| s.tag.to_lowercase());
    results
}
