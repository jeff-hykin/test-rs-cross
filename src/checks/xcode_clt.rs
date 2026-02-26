use anyhow::Result;
use std::process::Command;

use crate::config::Config;
use super::{Autofix, Check};

/// Xcode Command Line Tools — required by Homebrew on macOS.
pub fn check() -> Check {
    Check {
        label: "Xcode Command Line Tools",
        detect,
        fix_instructions: Some(fix_instructions),
        autofix: Some(Autofix {
            prompt: "Trigger the Xcode Command Line Tools installer?",
            run: autofix,
        }),
    }
}

fn detect(_cfg: &Config) -> bool {
    Command::new("xcode-select")
        .arg("-p")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn fix_instructions(_cfg: &Config) -> String {
    "Run `xcode-select --install` and complete the dialog, then re-run `dimos init`.".to_string()
}

fn autofix(_cfg: &Config) -> Result<()> {
    let status = Command::new("xcode-select")
        .arg("--install")
        .status()?;

    // exit code 1 with "already installed" message is a success case
    if status.success() {
        cliclack::log::warning(
            "A dialog has opened to install Xcode Command Line Tools.\n\
             Complete the installation, then re-run `dimos init`.",
        )?;
        anyhow::bail!("Re-run `dimos init` after the Xcode CLT installation finishes.");
    }

    // xcode-select --install exits non-zero if already installed
    if detect(&Config::default()) {
        return Ok(());
    }

    anyhow::bail!(
        "Could not install Xcode Command Line Tools automatically. \
         Run `xcode-select --install` manually."
    )
}
