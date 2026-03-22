mod handlers;
mod models;
mod storage;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.as_slice() {
        [_, cmd, rest @ ..] if cmd == "list" => {
            let verbose = rest.iter().any(|arg| arg == "-v" || arg == "--verbose");
            let search_term = rest
                .iter()
                .find(|arg| !arg.starts_with("-"))
                .cloned()
                .unwrap_or_default();

            handlers::handle_list(search_term, verbose);
        }
        [_, cmd, rest @ ..] if cmd == "remove" || cmd == "rm" => {
            let verbose = rest.iter().any(|arg| arg == "-v" || arg == "--verbose");
            let search_term = rest
                .iter()
                .find(|arg| !arg.starts_with("-"))
                .cloned()
                .unwrap_or_default();

            handlers::handle_remove(search_term, verbose);
        }
        [_, cmd, content, rest @ ..] if cmd == "add" => {
            let content = content.to_string();
            let tag = rest.get(0).cloned().unwrap_or_default();
            let desc = rest.get(1).cloned().unwrap_or_default();

            handlers::handle_add(content, tag, desc);
        }
        _ => {
            print!("Usage: add <content> [tag] [desc]");
        }
    }
}
