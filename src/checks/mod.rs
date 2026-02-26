use anyhow::Result;
use std::process::Command;

use crate::config::Config;

pub mod brew;
pub mod curl;
pub mod xcode_clt;
pub mod git;
pub mod git_lfs;
pub mod gxx;
pub mod libturbojpeg;
pub mod nix;
pub mod nix_flakes;
pub mod portaudio;
pub mod pre_commit_tool;
pub mod python_dev;
pub mod uv;

// ── core types ────────────────────────────────────────────────────────────────

pub struct Autofix {
    /// Confirmation prompt shown before attempting the fix.
    pub prompt: &'static str,
    /// Run the automated fix; receives the current config snapshot.
    pub run: fn(&Config) -> Result<()>,
}

pub struct Check {
    /// Short label used in spinner messages, e.g. `"git"` or `"nix flakes"`.
    pub label: &'static str,
    /// Returns `true` if the dependency is already present/satisfied.
    pub detect: fn(&Config) -> bool,
    /// Optional callback returning manual fix instructions shown on failure.
    pub fix_instructions: Option<fn(&Config) -> String>,
    /// Optional automated fix offered to the user when detection fails.
    pub autofix: Option<Autofix>,
}

impl Check {
    /// Run detect → (if failing) offer autofix → show instructions → bail.
    pub fn run(&self, config: &Config) -> Result<()> {
        let sp = cliclack::spinner();
        sp.start(format!("Checking {}…", self.label));

        if (self.detect)(config) {
            sp.stop(format!("{} found", self.label));
            return Ok(());
        }

        sp.error(format!("{} not found", self.label));

        if let Some(ref fix) = self.autofix {
            let yes = cliclack::confirm(fix.prompt)
                .initial_value(true)
                .interact()?;

            if yes {
                let sp2 = cliclack::spinner();
                sp2.start(format!("Running auto-fix for {}…", self.label));
                match (fix.run)(config) {
                    Ok(()) => {
                        sp2.stop(format!("{} ready", self.label));
                        return Ok(());
                    }
                    Err(e) => {
                        sp2.error(format!("Auto-fix failed for {}", self.label));
                        if let Some(instructions) = self.fix_instructions {
                            cliclack::log::info(instructions(config))?;
                        }
                        return Err(e);
                    }
                }
            }
        }

        if let Some(instructions) = self.fix_instructions {
            cliclack::log::info(instructions(config))?;
        }

        anyhow::bail!(
            "{} is required — install it then re-run `dimos init`.",
            self.label
        )
    }
}

// ── shared install helpers ────────────────────────────────────────────────────

pub fn apt_install(packages: &[&str]) -> Result<()> {
    let status = Command::new("sudo")
        .arg("apt-get")
        .arg("install")
        .arg("-y")
        .args(packages)
        .status()?;
    if !status.success() {
        anyhow::bail!("apt-get install {} failed", packages.join(" "));
    }
    Ok(())
}

pub fn brew_install(packages: &[&str]) -> Result<()> {
    let status = Command::new("brew")
        .arg("install")
        .args(packages)
        .status()?;
    if !status.success() {
        anyhow::bail!("brew install {} failed", packages.join(" "));
    }
    Ok(())
}

pub fn nix_install(pkg: &str) -> Result<()> {
    let status = Command::new("nix")
        .args(["profile", "install", pkg])
        .status()?;
    if !status.success() {
        anyhow::bail!("nix profile install {pkg} failed");
    }
    Ok(())
}

/// Check whether an apt package is currently installed.
pub fn is_apt_installed(pkg: &str) -> bool {
    Command::new("dpkg-query")
        .args(["-W", "-f=${Status}", pkg])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).contains("install ok installed"))
        .unwrap_or(false)
}

/// Check whether `pkg-config --exists <lib>` succeeds.
pub fn pkg_config_exists(lib: &str) -> bool {
    Command::new("pkg-config")
        .args(["--exists", lib])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
