use anyhow::Result;
use std::process::Command;

use crate::config::Config;
use super::{apt_install, brew_install, is_apt_installed, nix_install, Autofix, Check};

fn fix_instructions(_cfg: &Config) -> String {
    "Install Python development headers via your package manager.".to_string()
}

pub mod apt {
    use super::*;
    pub fn check() -> Check {
        Check {
            label: "python3-dev",
            detect,
            fix_instructions: Some(fix_instructions),
            autofix: Some(Autofix {
                prompt: "Install python3-dev via apt?",
                run: autofix,
            }),
        }
    }
    fn detect(_cfg: &Config) -> bool {
        is_apt_installed("python3-dev")
    }
    fn autofix(_cfg: &Config) -> Result<()> {
        apt_install(&["python3-dev"])
    }
}

pub mod brew {
    use super::*;
    pub fn check() -> Check {
        Check {
            label: "python3 (with headers)",
            detect,
            fix_instructions: Some(fix_instructions),
            autofix: Some(Autofix {
                prompt: "Install python3 via Homebrew (includes headers)?",
                run: autofix,
            }),
        }
    }
    fn detect(_cfg: &Config) -> bool {
        // On brew, python3 includes headers; check for python3-config
        Command::new("python3-config")
            .arg("--prefix")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
    fn autofix(_cfg: &Config) -> Result<()> {
        brew_install(&["python3"])
    }
}

pub mod nix {
    use super::*;
    pub fn check() -> Check {
        Check {
            label: "python3 (with headers)",
            detect,
            fix_instructions: Some(fix_instructions),
            autofix: Some(Autofix {
                prompt: "Install python3 via nix?",
                run: autofix,
            }),
        }
    }
    fn detect(_cfg: &Config) -> bool {
        Command::new("python3-config")
            .arg("--prefix")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
    fn autofix(_cfg: &Config) -> Result<()> {
        nix_install("nixpkgs#python3")
    }
}
