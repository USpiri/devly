mod args;
mod commit;
mod commit_types;
mod config;
mod git;
mod restore;
mod utils;

use args::{Commands, GitSubcommands};
use clap::Parser;
use commit::commit;
use config::{load_config, Config};
use restore::restore;

use crate::args::Cli;

fn main() {
    let cli: Cli = Cli::parse();
    let config:Config = match load_config(){
        Ok(config) => config,
        Err(e) => {
            eprint!("{e}");
            std::process::exit(1);
        },
    };
    match &cli.command {
        Some(Commands::Commit {}) => {
            commit(config.use_emoji);
        }
        Some(Commands::Git(commit)) => match &commit.command {
            Some(GitSubcommands::Restore {}) => {
                restore();
            }
            None => todo!(),
        },
        None => (),
    }
}
