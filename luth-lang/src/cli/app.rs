extern crate clap;
use clap::{Parser, Subcommand};

use super::actions::actions::run_action;

#[derive(Subcommand, Debug)]
enum Commands {
    /// Execute a file
    Run { file_execute: String },
}

#[derive(Debug, Parser)]
struct App {
    #[command(subcommand)]
    command: Commands,
}

pub fn init_cli() {
    let cli = App::parse();

    match cli.command {
        Commands::Run { file_execute } => run_action(&file_execute),
    }
}
