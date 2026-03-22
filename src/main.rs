mod handlers;
mod models;
mod parser;
mod storage;

use parser::ArgParser;

use crate::handlers::{handle_add, handle_help, handle_list, handle_remove};

fn main() {
    let mut parser = ArgParser::new();
    let cmd = parser.cmd().map(|s| s.to_lowercase());

    let exit_on_error = |e: String| -> ! {
        eprintln!("{e}");
        std::process::exit(1)
    };

    match cmd.as_deref() {
        Some("add") => {
            let tag = parser
                .get_value("-t", "--tag")
                .unwrap_or_else(|e| exit_on_error(e))
                .unwrap_or_default();
            let desc = parser
                .get_value("-d", "--description")
                .unwrap_or_else(|e| exit_on_error(e))
                .unwrap_or_default();
            let content = parser.get_content().unwrap_or_else(|e| exit_on_error(e));

            if content.is_empty() {
                panic!("Cannot add a snippet without content");
            }

            handle_add(content, tag, desc);
        }
        Some("list") | Some("li") => {
            let verbose = parser.has_flag("-v", "--verbose");
            let search_term = parser.get_content().unwrap_or_else(|e| exit_on_error(e));

            handle_list(search_term, verbose);
        }
        Some("remove") | Some("rm") => {
            let verbose = parser.has_flag("-v", "--verbose");
            let search_term = parser.get_content().unwrap_or_else(|e| exit_on_error(e));

            handle_remove(search_term, verbose);
        }
        _ => {
            handle_help(parser.bin_name().as_str());
        }
    }
}
