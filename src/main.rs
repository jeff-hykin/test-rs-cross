use anyhow::Result;
use clap::Parser;

mod cli;
mod commands;
mod config;
mod ui;

fn main() -> Result<()> {
    let args = cli::Cli::parse();
    match args.command {
        cli::Commands::Init => commands::init::run(),
        cli::Commands::NewApp => commands::new_app::run(),
    }
}
