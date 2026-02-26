use anyhow::Result;
use which::which;

use crate::config::Config;
use super::{apt_install, brew_install, nix_install, Autofix, Check};

fn detect(_cfg: &Config) -> bool {
    which("git-lfs").is_ok()
}

fn fix_instructions(_cfg: &Config) -> String {
    "Install git-lfs via your package manager (apt/brew/nix).".to_string()
}

pub mod apt {
    use super::*;
    pub fn check() -> Check {
        Check {
            label: "git-lfs",
            detect,
            fix_instructions: Some(fix_instructions),
            autofix: Some(Autofix {
                prompt: "Install git-lfs via apt?",
                run: autofix,
            }),
        }
    }
    fn autofix(_cfg: &Config) -> Result<()> {
        apt_install(&["git-lfs"])
    }
}

pub mod brew {
    use super::*;
    pub fn check() -> Check {
        Check {
            label: "git-lfs",
            detect,
            fix_instructions: Some(fix_instructions),
            autofix: Some(Autofix {
                prompt: "Install git-lfs via Homebrew?",
                run: autofix,
            }),
        }
    }
    fn autofix(_cfg: &Config) -> Result<()> {
        brew_install(&["git-lfs"])
    }
}

pub mod nix {
    use super::*;
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
    fn autofix(_cfg: &Config) -> Result<()> {
        nix_install("nixpkgs#git-lfs")
    }
}
