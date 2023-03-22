use args::Commands;
use clap::Parser;

use crate::args::Cli;

mod args;

fn main() {
    let cli:Cli = Cli::parse();
    match &cli.command {
        Some(Commands::Commit {}) => {
            println!("Devly commit command");
        },
        None => ()
    }
}
