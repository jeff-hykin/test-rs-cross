use anyhow::Result;
use which::which;

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

    let is_linux = std::env::consts::OS == "linux";
    let is_macos = std::env::consts::OS == "macos";
    let has_apt = which("apt").is_ok();

    let mut select =
        cliclack::select("Which install sequence would you like to run?");

    if is_linux && has_apt {
        select = select.item("linux_apt", "Linux — apt", "Ubuntu / Debian");
    }
    if is_linux {
        select = select.item("linux_nix", "Linux — nix", "any Linux with nix");
    }
    if is_macos {
        select = select.item("macos_brew", "macOS — Homebrew", "macOS with brew");
        select = select.item("macos_nix", "macOS — nix", "macOS with nix");
    }
    select = select.item("skip", "Skip for now", "configure your environment manually");

    let sequence_key: &str = select.interact()?;

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
