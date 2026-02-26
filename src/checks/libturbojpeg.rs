use anyhow::Result;

use crate::config::Config;
use super::{apt_install, brew_install, is_apt_installed, nix_install, pkg_config_exists, Autofix, Check};

fn fix_instructions(_cfg: &Config) -> String {
    "Install libturbojpeg dev headers via your package manager.".to_string()
}

pub mod apt {
    use super::*;
    pub fn check() -> Check {
        Check {
            label: "libturbojpeg0-dev",
            detect,
            fix_instructions: Some(fix_instructions),
            autofix: Some(Autofix {
                prompt: "Install libturbojpeg0-dev via apt?",
                run: autofix,
            }),
        }
    }
    fn detect(_cfg: &Config) -> bool {
        is_apt_installed("libturbojpeg0-dev")
    }
    fn autofix(_cfg: &Config) -> Result<()> {
        apt_install(&["libturbojpeg0-dev"])
    }
}

pub mod brew {
    use super::*;
    pub fn check() -> Check {
        Check {
            label: "libturbojpeg",
            detect,
            fix_instructions: Some(fix_instructions),
            autofix: Some(Autofix {
                prompt: "Install jpeg-turbo via Homebrew?",
                run: autofix,
            }),
        }
    }
    fn detect(_cfg: &Config) -> bool {
        pkg_config_exists("libturbojpeg")
    }
    fn autofix(_cfg: &Config) -> Result<()> {
        brew_install(&["jpeg-turbo"])
    }
}

pub mod nix {
    use super::*;
    pub fn check() -> Check {
        Check {
            label: "libturbojpeg",
            detect,
            fix_instructions: Some(fix_instructions),
            autofix: Some(Autofix {
                prompt: "Install libjpeg-turbo via nix?",
                run: autofix,
            }),
        }
    }
    fn detect(_cfg: &Config) -> bool {
        pkg_config_exists("libturbojpeg")
    }
    fn autofix(_cfg: &Config) -> Result<()> {
        nix_install("nixpkgs#libjpeg-turbo")
    }
}
