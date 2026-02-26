use anyhow::Result;

use crate::{
    config::ConfigManager,
    install_sequences::{linux_apt, linux_nix, macos_brew, macos_nix},
    ui,
};

pub fn run() -> Result<()> {
    ui::header(" Dimos — Init");

    let mut mgr = ConfigManager::load_or_recover()?;
    mgr.config.init_completed = true;
    mgr.save()?;
    cliclack::log::success(format!("Config saved → {}", mgr.path().display()))?;

    let sequence_key: &str = cliclack::select("Which install sequence would you like to run?")
        .item("linux_apt", "Linux — apt", "Ubuntu / Debian")
        .item("linux_nix", "Linux — nix", "any Linux with the nix package manager")
        .item("macos_brew", "macOS — Homebrew", "macOS with brew")
        .item("macos_nix", "macOS — nix", "macOS with the nix package manager")
        .item("skip", "Skip for now", "configure your environment manually")
        .interact()?;

    match sequence_key {
        "linux_apt" => linux_apt::sequence().run(&mgr.config)?,
        "linux_nix" => linux_nix::sequence().run(&mgr.config)?,
        "macos_brew" => macos_brew::sequence().run(&mgr.config)?,
        "macos_nix" => macos_nix::sequence().run(&mgr.config)?,
        _ => {
            ui::outro("Skipped. Run `dimos init` again to run an install sequence.");
        }
    }

    Ok(())
}
