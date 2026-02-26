use anyhow::Result;
use std::process::Command;
use which::which;

use crate::config::Config;
use super::{apt_install, brew_install, nix_install, Autofix, Check};

fn detect(_cfg: &Config) -> bool {
    which("pre-commit").is_ok()
}

fn fix_instructions(_cfg: &Config) -> String {
    "Install pre-commit: https://pre-commit.com/#install".to_string()
}

pub mod apt {
    use super::*;
    pub fn check() -> Check {
        Check {
            label: "pre-commit",
            detect,
            fix_instructions: Some(fix_instructions),
            autofix: Some(Autofix {
                prompt: "Install pre-commit via apt?",
                run: autofix,
            }),
        }
    }
    fn autofix(_cfg: &Config) -> Result<()> {
        apt_install(&["pre-commit"])
    }
}

pub mod brew {
    use super::*;
    pub fn check() -> Check {
        Check {
            label: "pre-commit",
            detect,
            fix_instructions: Some(fix_instructions),
            autofix: Some(Autofix {
                prompt: "Install pre-commit via Homebrew?",
                run: autofix,
            }),
        }
    }
    fn autofix(_cfg: &Config) -> Result<()> {
        brew_install(&["pre-commit"])
    }
}

pub mod nix {
    use super::*;
    pub fn check() -> Check {
        Check {
            label: "pre-commit",
            detect,
            fix_instructions: Some(fix_instructions),
            autofix: Some(Autofix {
                prompt: "Install pre-commit via nix?",
                run: autofix,
            }),
        }
    }
    fn autofix(_cfg: &Config) -> Result<()> {
        nix_install("nixpkgs#pre-commit")
    }
}

pub mod pip {
    use super::*;
    pub fn check() -> Check {
        Check {
            label: "pre-commit",
            detect,
            fix_instructions: Some(fix_instructions),
            autofix: Some(Autofix {
                prompt: "Install pre-commit via pip (uv tool install)?",
                run: autofix,
            }),
        }
    }
    fn autofix(_cfg: &Config) -> Result<()> {
        let status = Command::new("uv")
            .args(["tool", "install", "pre-commit"])
            .status()?;
        if !status.success() {
            anyhow::bail!("uv tool install pre-commit failed");
        }
        Ok(())
    }
}
