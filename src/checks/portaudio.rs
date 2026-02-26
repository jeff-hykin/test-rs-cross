use anyhow::Result;

use crate::config::Config;
use super::{apt_install, brew_install, is_apt_installed, nix_install, pkg_config_exists, Autofix, Check};

fn fix_instructions(_cfg: &Config) -> String {
    "Install portaudio dev headers via your package manager.".to_string()
}

pub mod apt {
    use super::*;
    pub fn check() -> Check {
        Check {
            label: "portaudio19-dev",
            detect,
            fix_instructions: Some(fix_instructions),
            autofix: Some(Autofix {
                prompt: "Install portaudio19-dev via apt?",
                run: autofix,
            }),
        }
    }
    fn detect(_cfg: &Config) -> bool {
        is_apt_installed("portaudio19-dev")
    }
    fn autofix(_cfg: &Config) -> Result<()> {
        apt_install(&["portaudio19-dev"])
    }
}

pub mod brew {
    use super::*;
    pub fn check() -> Check {
        Check {
            label: "portaudio",
            detect,
            fix_instructions: Some(fix_instructions),
            autofix: Some(Autofix {
                prompt: "Install portaudio via Homebrew?",
                run: autofix,
            }),
        }
    }
    fn detect(_cfg: &Config) -> bool {
        pkg_config_exists("portaudio-2.0")
    }
    fn autofix(_cfg: &Config) -> Result<()> {
        brew_install(&["portaudio"])
    }
}

pub mod nix {
    use super::*;
    pub fn check() -> Check {
        Check {
            label: "portaudio",
            detect,
            fix_instructions: Some(fix_instructions),
            autofix: Some(Autofix {
                prompt: "Install portaudio via nix?",
                run: autofix,
            }),
        }
    }
    fn detect(_cfg: &Config) -> bool {
        pkg_config_exists("portaudio-2.0")
    }
    fn autofix(_cfg: &Config) -> Result<()> {
        nix_install("nixpkgs#portaudio")
    }
}
