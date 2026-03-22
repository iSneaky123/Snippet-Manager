use std::io::{self, Write};
use std::usize;

use crate::models::Snippet;
use crate::storage::{load_snippets, save_snippets};

use nucleo::{Config, Matcher, Utf32Str};

pub fn handle_add(content: String, tag: String, desc: String) {
    let new_snip = Snippet::new(content, tag, desc);

    let mut snippets = load_snippets();
    snippets.push(new_snip);

    save_snippets(&snippets);
    println!("Saved Snippet successfully!");
}

pub fn handle_list(search_term: String, verbose: bool) {
    let snippets = load_snippets();
    let filtered_snippets = filter_and_sort_snippets(&search_term, &snippets);
    print_snippets(&filtered_snippets, verbose);
}

pub fn handle_remove(search_term: String, verbose: bool) {
    let mut snippets = load_snippets();
    let filtered_snippets = filter_and_sort_snippets(&search_term, &snippets);

    print_snippets(&filtered_snippets, verbose);

    print!("\nEnter Sr No. to delete (or 'q' to cancel): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();
    if input == "q" {
        println!("No snippets were removed!");
        return;
    }

    match input.parse::<usize>() {
        Ok(sr_no) if sr_no > 0 && sr_no <= filtered_snippets.len() => {
            let target_id = &filtered_snippets[sr_no - 1].id.clone();

            print!(
                "Are you sure you want to delete the following Snippet (ID: {})? (Y/N): ",
                target_id
            );
            io::stdout().flush().unwrap();

            let mut confirm = String::new();
            io::stdin().read_line(&mut confirm).ok();

            if confirm.trim().to_lowercase() == "y" {
                snippets.retain(|s| &s.id != target_id);
                save_snippets(&snippets);
                println!("Removed Snippet successfully!");
            } else {
                println!("Aborted! No snippets were removed");
            }
        }
        _ => println!("Invalid input. Please enter a valid Sr No."),
    }
}

fn print_snippets(snippets: &[&Snippet], verbose: bool) {
    if snippets.is_empty() {
        eprintln!("No snippets found...");
        std::process::exit(0)
    }

    let mut current_tag: Option<String> = None;

    for (idx, s) in snippets.iter().enumerate() {
        let display_tag = if s.tag.is_empty() { "NO TAG" } else { &s.tag };

        // Print tags only once
        if current_tag.as_deref() != Some(display_tag) {
            println!("\n=== TAG: {} ===", display_tag.to_uppercase());
            current_tag = Some(display_tag.to_string());
        }

        println!("Sr No.: {}", idx + 1);
        println!("ID: {}", s.id);
        println!("Snippet: ");
        for line in s.content.lines() {
            println!("  {}", line);
        }
        if verbose {
            println!("Description: ");
            for line in s.description.lines() {
                println!("  {}", line);
            }
        }
        println!()
    }
}

fn filter_and_sort_snippets<'a>(query_str: &str, snippets: &'a [Snippet]) -> Vec<&'a Snippet> {
    let mut results: Vec<&Snippet> = if query_str.is_empty() {
        snippets.iter().collect()
    } else {
        let mut matcher = Matcher::new(Config::DEFAULT);
        let mut query_buf = vec![];
        let query_utf32 = Utf32Str::new(query_str, &mut query_buf);
        let mut snapshot_buf = vec![];

        snippets
            .iter()
            .filter(|s| {
                let mut check = |text: &str, match_score: u16| {
                    let haystack = Utf32Str::new(text, &mut snapshot_buf);
                    if let Some(score) = matcher.fuzzy_match(haystack, query_utf32) {
                        score > match_score
                    } else {
                        false
                    }
                };

                check(&s.tag, 50) || check(&s.description, 75) || check(&s.id, 25)
            })
            .collect()
    };

    results.sort_by(|a, b| a.tag.to_lowercase().cmp(&b.tag.to_lowercase()));

    results
}

pub fn handle_help(bin: &str) {
    let help_text = format!(
        r#"
{bin} - A tiny snippet manager for your terminal

USAGE:
    {bin} <SUBCOMMAND> [OPTIONS] [CONTENT]

SUBCOMMANDS:
    add                 Create a new snippet
    list, li            List all snippets (supports searching)
    remove, rm          Delete snippets matching a search term

OPTIONS:
    -t, --tag <tag>           Categorize your snippet
    -d, --description <desc>  Add extra context to a snippet
    -v, --verbose             Show extra details in output

EXAMPLES:
    {bin} add "git clone ssh:url" -t git -d "clone repo"
    {bin} list "git" --verbose
    {bin} rm "git clone"
"#,
        bin = bin
    );
    println!("{help_text}");
}
