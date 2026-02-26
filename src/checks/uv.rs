use anyhow::Result;
use std::process::Command;
use which::which;

use crate::config::Config;
use super::{Autofix, Check};

/// The official uv installer works the same on all platforms.
pub fn check() -> Check {
    Check {
        label: "uv",
        detect,
        fix_instructions: Some(fix_instructions),
        autofix: Some(Autofix {
            prompt: "Install uv via the official installer (astral.sh)?",
            run: autofix,
        }),
    }
}

fn detect(_cfg: &Config) -> bool {
    which("uv").is_ok()
}

fn fix_instructions(_cfg: &Config) -> String {
    "Install uv manually: https://docs.astral.sh/uv/getting-started/installation/".to_string()
}

fn autofix(_cfg: &Config) -> Result<()> {
    let status = Command::new("sh")
        .args(["-c", "curl -LsSf https://astral.sh/uv/install.sh | sh"])
        .status()?;

    if !status.success() {
        anyhow::bail!("Failed to install uv — install it manually then re-run `dimos init`.");
    }
    Ok(())
}
