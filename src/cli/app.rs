extern crate clap;
use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
enum Commands {
    /// Execute a file
    Run { file_execute: String },
}

#[derive(Debug, Parser)]
struct App {
    file_execute: String,

    #[command(subcommand)]
    command: Commands,
}

pub fn init_cli() {
    let cli = App::parse();

    match cli.command {
        Commands::Run { file_execute: _ } => {
            // Run Action
        }
    }
}
