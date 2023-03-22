use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version)]
#[command(
    about = "Devly - a simple CLI to help you in the development process",
    long_about = None
)]
#[command(arg_required_else_help(true))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {

    /// Allows the user to commit using the Conventional Commits standard
    /// for specifying the type of commit.
    #[clap(verbatim_doc_comment)]
    Commit {
    },

}