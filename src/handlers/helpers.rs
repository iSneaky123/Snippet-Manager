use std::io::{self, Write};

use anyhow::{Result, anyhow, bail};
use nucleo::{Config, Matcher, Utf32Str};

use crate::models::Snippet;

pub(super) fn print_snippets(snippets: &[&Snippet], verbose: bool) {
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

        if idx + 1 < snippets.len() {
            println!();
        }
    }
}

pub(super) fn filter_and_sort_snippets<'a>(
    snippets: &'a [Snippet],
    search_term: &str,
) -> Vec<&'a Snippet> {
    if search_term.trim().is_empty() {
        let mut all: Vec<&Snippet> = snippets.iter().collect();
        all.sort_by_key(|s| s.tag.to_lowercase());
        return all;
    }

    let mut matcher = Matcher::new(Config::DEFAULT);

    let mut query_buf = vec![];
    let query_utf31 = Utf32Str::new(search_term, &mut query_buf);
    let mut haystack_buf = vec![];

    let mut results: Vec<&Snippet> = snippets
        .iter()
        .filter(|s| {
            let fields = [
                (s.tag.as_str(), 49),
                (s.description.as_str(), 74),
                (s.content.as_str(), 74),
                (s.id.as_str(), 49),
            ];

            fields.iter().any(|(text, min_score)| {
                if text.trim().is_empty() {
                    return false;
                }

                haystack_buf.clear();
                let haystack = Utf32Str::new(text, &mut haystack_buf);
                matcher
                    .fuzzy_match(haystack, query_utf31)
                    .map(|score| score >= *min_score)
                    .unwrap_or(false)
            })
        })
        .collect();

    results.sort_by_key(|s| s.tag.to_lowercase());
    results
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

    let sr_no: usize = input
        .parse()
        .map_err(|_| anyhow!("'{}' is not a valid number", input))?;

    if !(1..=snippets.len()).contains(&sr_no) {
        bail!("Sr No. {} is out of range (1-{})", sr_no, snippets.len());
    }

    Ok(Some(snippets[sr_no - 1].id.clone()))
}

pub(super) fn get_confirmation() -> Result<bool> {
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm)?;

    Ok(confirm.trim().to_lowercase() == "y")
}

pub(super) fn filter_and_display_snippets<'a>(
    snippets: &'a [Snippet],
    search_term: &str,
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
