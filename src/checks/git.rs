use anyhow::Result;
use std::process::Command;
use which::which;

use super::{Autofix, Check};

pub fn check() -> Check {
    Check {
        label: "git",
        detect,
        fix_instructions: Some(fix_instructions),
        autofix: Some(Autofix {
            prompt: "Install git via `nix profile install nixpkgs#git`?",
            run: autofix,
        }),
    }
}

fn detect() -> bool {
    which("git").is_ok()
}

fn fix_instructions() -> String {
    "Run: nix profile install nixpkgs#git".to_string()
}

fn autofix() -> Result<()> {
    let status = Command::new("nix")
        .args(["profile", "install", "nixpkgs#git"])
        .status()?;

    if !status.success() {
        anyhow::bail!("Failed to install git — install it manually then re-run `dimos init`.");
    }
    Ok(())
}
