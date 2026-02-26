use anyhow::Result;
use which::which;

use crate::config::Config;
use super::{apt_install, brew_install, nix_install, Autofix, Check};

fn detect(_cfg: &Config) -> bool {
    which("curl").is_ok()
}

fn fix_instructions(_cfg: &Config) -> String {
    "Install curl via your package manager (apt/brew/nix).".to_string()
}

pub mod apt {
    use super::*;
    pub fn check() -> Check {
        Check {
            label: "curl",
            detect,
            fix_instructions: Some(fix_instructions),
            autofix: Some(Autofix {
                prompt: "Install curl via apt?",
                run: autofix,
            }),
        }
    }
    fn autofix(_cfg: &Config) -> Result<()> {
        apt_install(&["curl"])
    }
}

pub mod brew {
    use super::*;
    pub fn check() -> Check {
        Check {
            label: "curl",
            detect,
            fix_instructions: Some(fix_instructions),
            autofix: Some(Autofix {
                prompt: "Install curl via Homebrew?",
                run: autofix,
            }),
        }
    }
    fn autofix(_cfg: &Config) -> Result<()> {
        brew_install(&["curl"])
    }
}

pub mod nix {
    use super::*;
    pub fn check() -> Check {
        Check {
            label: "curl",
            detect,
            fix_instructions: Some(fix_instructions),
            autofix: Some(Autofix {
                prompt: "Install curl via nix?",
                run: autofix,
            }),
        }
    }
    fn autofix(_cfg: &Config) -> Result<()> {
        nix_install("nixpkgs#curl")
    }
}
