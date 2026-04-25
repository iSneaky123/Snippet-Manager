mod handlers;
mod models;
mod storage;

use std::sync::Arc;

use clap::{Parser, Subcommand};

use crate::{
    handlers::{handle_add, handle_execute, handle_list, handle_remove},
    storage::file_storage::FileStorage,
};

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
        #[arg(short, long)]
        tag: Option<String>,
        /// Add extra context or explaination for your snippet
        #[arg(short, long)]
        description: Option<String>,
        /// Sets default shell which is to be used for the following command's execution
        #[arg(short, long)]
        shell_type: Option<String>,
    },
    /// List and search through your saved snippets
    #[command(alias = "li")]
    List {
        /// Optional Search query
        #[arg()]
        query: Option<String>,
        /// Show snippets along with their descriptions
        #[arg(short, long)]
        verbose: bool,
    },
    /// Interactive remove a snippet matching a search query
    #[command(alias = "rm")]
    Remove {
        /// Search query to find the snippet you want to delete
        #[arg()]
        query: Option<String>,
        /// Show snippets along with their descriptions
        #[arg(short, long)]
        verbose: bool,
    },
    /// Interactive execute a snippet matching a search query
    #[command(alias = "ex", alias = "exec")]
    Execute {
        /// Search query to find the snippet you want to execute
        #[arg()]
        query: Option<String>,
        /// Overrides the default shell for command execution if provided
        #[arg(short, long)]
        shell_type: Option<String>,
        /// Show snippets along with their descriptions
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let storage = Arc::new(FileStorage::new()?);
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Add {
            content,
            tag,
            description,
            shell_type,
        }) => {
            handle_add(content, tag, description, shell_type, storage)?;
        }
        Some(Commands::Remove { query, verbose }) => {
            handle_remove(query, verbose, storage)?;
        }
        Some(Commands::List { query, verbose }) => {
            handle_list(query, verbose, storage)?;
        }
        Some(Commands::Execute {
            query,
            shell_type,
            verbose,
        }) => {
            handle_execute(query, shell_type, verbose, storage)?;
        }

        None => {
            use clap::CommandFactory;
            Cli::command().print_help()?;
        }
    }

    Ok(())
}
