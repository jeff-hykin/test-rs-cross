use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "dimos",
    about = "Dimos — project scaffolding and environment setup",
    version,
    propagate_version = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Set up your development environment (nix, git, uv, git-lfs)
    Init,

    /// Answer personality questions stored in your config
    Survey,

    /// Scaffold a new Python application
    #[command(name = "new-app")]
    NewApp,
}
