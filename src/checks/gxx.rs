use anyhow::Result;
use which::which;

use crate::config::Config;
use super::{apt_install, brew_install, nix_install, Autofix, Check};

fn detect(_cfg: &Config) -> bool {
    which("g++").is_ok() || which("c++").is_ok()
}

fn fix_instructions(_cfg: &Config) -> String {
    "Install g++ (C++ compiler) via your package manager.".to_string()
}

pub mod apt {
    use super::*;
    pub fn check() -> Check {
        Check {
            label: "g++",
            detect,
            fix_instructions: Some(fix_instructions),
            autofix: Some(Autofix {
                prompt: "Install g++ via apt?",
                run: autofix,
            }),
        }
    }
    fn autofix(_cfg: &Config) -> Result<()> {
        apt_install(&["g++"])
    }
}

pub mod brew {
    use super::*;
    pub fn check() -> Check {
        Check {
            label: "g++",
            detect,
            fix_instructions: Some(fix_instructions),
            autofix: Some(Autofix {
                prompt: "Install gcc (includes g++) via Homebrew?",
                run: autofix,
            }),
        }
    }
    fn autofix(_cfg: &Config) -> Result<()> {
        brew_install(&["gcc"])
    }
}

pub mod nix {
    use super::*;
    pub fn check() -> Check {
        Check {
            label: "g++",
            detect,
            fix_instructions: Some(fix_instructions),
            autofix: Some(Autofix {
                prompt: "Install gcc via nix?",
                run: autofix,
            }),
        }
    }
    fn autofix(_cfg: &Config) -> Result<()> {
        nix_install("nixpkgs#gcc")
    }
}
