use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

use crate::models::Snippet;

fn get_storage_path() -> PathBuf {
    let mut path = dirs::data_dir().expect("Could not find standard data directory on this OS");
    path.push("snip");
    path.push("snippets.json");

    path
}

pub fn load_snippets() -> Vec<Snippet> {
    let path = get_storage_path();

    match std::fs::read_to_string(path) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(e) if e.kind() == ErrorKind::NotFound => vec![],
        Err(e) => {
            eprint!("Error reading snippets: {}", e);
            vec![]
        }
    }
}

pub fn save_snippets(snippets: &[Snippet]) {
    let path = get_storage_path();

    if let Some(parent) = path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            eprintln!(
                "Fatal Error: Coul not create data directory at {:?}",
                parent
            );
            eprintln!("Reason: {}", e);
            std::process::exit(1);
        }
    }

    let mut json = serde_json::to_string_pretty(snippets).expect("Failed to serialize snippets");
    json.push('\n');

    if let Err(e) = fs::write(&path, json) {
        eprintln!("Fatal Error: Failed to save the snippet to {:?}", path);
        eprintln!("Reason: {}", e);
        std::process::exit(1);
    };
}
