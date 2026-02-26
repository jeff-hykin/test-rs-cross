use anyhow::Result;
use std::process::Command;
use which::which;

use crate::config::Config;
use super::{Autofix, Check};

/// Check for Homebrew itself (macOS sequences need this first).
pub fn check() -> Check {
    Check {
        label: "Homebrew",
        detect,
        fix_instructions: Some(fix_instructions),
        autofix: Some(Autofix {
            prompt: "Install Homebrew via the official installer?",
            run: autofix,
        }),
    }
}

fn detect(_cfg: &Config) -> bool {
    which("brew").is_ok()
}

fn fix_instructions(_cfg: &Config) -> String {
    "Install Homebrew: https://brew.sh".to_string()
}

fn autofix(_cfg: &Config) -> Result<()> {
    let status = Command::new("sh")
        .args([
            "-c",
            r#"/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)""#,
        ])
        .status()?;

    if !status.success() {
        anyhow::bail!("Homebrew installation failed — see https://brew.sh for manual instructions.");
    }

    cliclack::log::warning(
        "Homebrew installed. You may need to add it to your PATH — \
         follow the instructions printed above.",
    )?;
    Ok(())
}
