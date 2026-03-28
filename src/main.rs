mod handlers;
mod models;
mod storage;

use clap::{Parser, Subcommand};

use crate::handlers::{handle_add, handle_list, handle_remove};

#[derive(Parser)]
#[command(name = "snip")]
/// A tiny snippet manager for your terminal
#[command(about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new snippet in the snippet manager
    Add {
        /// The actual content of the snippet
        content: String,
        /// Categorize your snippets with a tag (e.g. -t rust)
        #[arg(short, long, default_value = "")]
        tag: String,
        /// Add extra context or explaination for your snippet
        #[arg(short, long, default_value = "")]
        description: String,
    },
    /// List and search through your saved snippets
    #[command(alias = "li")]
    List {
        /// Optional Search query
        #[arg(default_value = "")]
        query: String,
        /// Show snippets along with their descriptions
        #[arg(short, long)]
        verbose: bool,
    },
    /// Interactive remove a snippet matching a search query
    #[command(alias = "rm")]
    Remove {
        /// Search query to find the snippet you want to delete
        #[arg(default_value = "")]
        query: String,
        /// Show snippets along with their descriptions
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Add {
            content,
            tag,
            description,
        }) => {
            handle_add(content, tag, description)?;
        }
        Some(Commands::Remove { query, verbose }) => {
            handle_remove(&query, verbose)?;
        }
        Some(Commands::List { query, verbose }) => {
            handle_list(&query, verbose)?;
        }
        None => {
            use clap::CommandFactory;
            Cli::command().print_help()?;
        }
    }

    Ok(())
}
