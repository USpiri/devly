mod args;
mod commit;
mod git;
mod utils;
mod commit_types;

use args::Commands;
use clap::Parser;
use commit::commit;

use crate::args::Cli;

fn main() {
    let cli:Cli = Cli::parse();
    match &cli.command {
        Some(Commands::Commit {}) => {
            commit();
        },
        None => ()
    }
}
