//! Command line surface of `amfs`.
//!
//! The subcommands here define the shape of the tool; none of them are wired to
//! a storage engine yet, so every one of them fails with [`Error::NotImplemented`].

use clap::{Parser, Subcommand};

/// Agent Memory File System — store, search, and manage agent memories locally.
#[derive(Debug, Parser)]
#[command(name = "amfs", version = version(), about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

/// Everything `amfs` can be asked to do.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Store a new memory
    Add {
        /// Text to remember
        content: String,

        /// Owner this memory belongs to
        #[arg(short, long)]
        user_id: Option<String>,
    },

    /// Find memories by meaning, not by keyword
    Search {
        /// What to look for
        query: String,

        /// Restrict the search to a single owner
        #[arg(short, long)]
        user_id: Option<String>,

        /// Maximum number of matches to return
        #[arg(short = 'n', long, default_value_t = 10)]
        limit: usize,
    },

    /// Show a single memory
    Get {
        /// Identifier of the memory
        id: String,
    },

    /// List stored memories
    List {
        /// Restrict the listing to a single owner
        #[arg(short, long)]
        user_id: Option<String>,

        /// Maximum number of memories to list
        #[arg(short = 'n', long, default_value_t = 20)]
        limit: usize,
    },

    /// Replace the content of an existing memory
    Update {
        /// Identifier of the memory
        id: String,

        /// New text to store in its place
        content: String,
    },

    /// Remove a memory
    Delete {
        /// Identifier of the memory
        id: String,
    },
}

/// Anything that can go wrong while running a subcommand.
#[derive(Debug)]
pub enum Error {
    /// The subcommand is part of the planned surface but has no implementation yet.
    NotImplemented(&'static str),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotImplemented(command) => {
                write!(f, "`{command}` is not implemented yet")
            }
        }
    }
}

impl std::error::Error for Error {}

impl Cli {
    /// Dispatches the parsed subcommand.
    pub fn run(self) -> Result<(), Error> {
        match self.command {
            Command::Add { .. } => Err(Error::NotImplemented("add")),
            Command::Search { .. } => Err(Error::NotImplemented("search")),
            Command::Get { .. } => Err(Error::NotImplemented("get")),
            Command::List { .. } => Err(Error::NotImplemented("list")),
            Command::Update { .. } => Err(Error::NotImplemented("update")),
            Command::Delete { .. } => Err(Error::NotImplemented("delete")),
        }
    }
}

/// Version string reported by `amfs --version`, injected by `build.rs`.
fn version() -> &'static str {
    crate::version()
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn every_subcommand_is_unimplemented() {
        let commands = [
            vec!["amfs", "add", "hello"],
            vec!["amfs", "search", "hello"],
            vec!["amfs", "get", "1"],
            vec!["amfs", "list"],
            vec!["amfs", "update", "1", "hello"],
            vec!["amfs", "delete", "1"],
        ];

        for argv in commands {
            let cli = Cli::try_parse_from(&argv).expect("argv should parse");
            assert!(matches!(cli.run(), Err(Error::NotImplemented(_))));
        }
    }
}
