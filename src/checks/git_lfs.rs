use anyhow::Result;
use std::process::Command;
use which::which;

use super::{Autofix, Check};

pub fn check() -> Check {
    Check {
        label: "git-lfs",
        detect,
        fix_instructions: Some(fix_instructions),
        autofix: Some(Autofix {
            prompt: "Install git-lfs via `nix profile install nixpkgs#git-lfs`?",
            run: autofix,
        }),
    }
}

fn detect() -> bool {
    which("git-lfs").is_ok()
}

fn fix_instructions() -> String {
    "Run: nix profile install nixpkgs#git-lfs".to_string()
}

fn autofix() -> Result<()> {
    let status = Command::new("nix")
        .args(["profile", "install", "nixpkgs#git-lfs"])
        .status()?;

    if !status.success() {
        anyhow::bail!(
            "Failed to install git-lfs — install it manually then re-run `dimos init`."
        );
    }
    Ok(())
}
