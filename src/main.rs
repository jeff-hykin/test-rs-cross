use anyhow::Result;
use clap::Parser;

mod checks;
mod cli;
mod config;
mod install_sequences;
mod questions;
mod subcommands;
mod ui;

fn main() -> Result<()> {
    let args = cli::Cli::parse();
    match args.command {
        cli::Commands::Init => subcommands::init::run(),
        cli::Commands::Survey => subcommands::survey::run(),
        cli::Commands::NewApp => subcommands::new_app::run(),
    }
}
