use anyhow::Result;
use std::process::Command;
use which::which;

use super::{Autofix, Check};

pub fn check() -> Check {
    Check {
        label: "nix",
        detect,
        fix_instructions: Some(fix_instructions),
        autofix: Some(Autofix {
            prompt: "Install nix via the Determinate Systems installer?",
            run: autofix,
        }),
    }
}

fn detect() -> bool {
    which("nix").is_ok()
}

fn fix_instructions() -> String {
    "Install nix manually: https://nixos.org/download/".to_string()
}

fn autofix() -> Result<()> {
    let status = Command::new("sh")
        .args([
            "-c",
            "curl --proto '=https' --tlsv1.2 -sSf -L \
             https://install.determinate.systems/nix | sh -s -- install",
        ])
        .status()?;

    if !status.success() {
        anyhow::bail!("nix installation failed — install manually then re-run `dimos init`.");
    }

    cliclack::log::warning("Open a new terminal so nix is on PATH before continuing.")?;
    Ok(())
}
