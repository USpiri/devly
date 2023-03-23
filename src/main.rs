mod args;
mod commit_types;

use args::Commands;
use clap::Parser;
use commit_types::load_commit_types;

use crate::args::Cli;

fn main() {
    let cli:Cli = Cli::parse();
    match &cli.command {
        Some(Commands::Commit {}) => {
            let commit_types = load_commit_types();
            match commit_types {
                Ok(commit_types) => for commit_type in commit_types {
                    println!("{commit_type}");
                },
                Err(e) =>  println!("ERROR: {}", e),
            }
        },
        None => ()
    }
}
