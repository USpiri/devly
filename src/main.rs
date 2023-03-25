mod args;
mod commit;
mod git;
mod utils;
mod commit_types;
mod restore;

use args::{Commands, GitSubcommands};
use clap::Parser;
use commit::commit;
use restore::restore;

use crate::args::Cli;

fn main() {
    let cli:Cli = Cli::parse();
    match &cli.command {
        Some(Commands::Commit {}) => {
            commit();
        },
        Some(Commands::Git(commit)) => {
            match &commit.command {
                Some(GitSubcommands::Restore {}) => {
                    restore();
                }
                None => todo!(),
            }
        },
        None => ()
    }
}
