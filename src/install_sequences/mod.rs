use anyhow::Result;
use std::{io::IsTerminal, process::Command};

use crate::{checks::Check, config::Config, ui};

pub mod linux_apt;
pub mod linux_nix;
pub mod macos_brew;
pub mod macos_nix;

// ── types ─────────────────────────────────────────────────────────────────────

/// A step that always runs (no detection), used for preamble tasks like
/// `apt-get update`.
pub struct Step {
    pub label: &'static str,
    pub run: fn(&Config) -> Result<()>,
}

pub struct InstallSequence {
    pub name: &'static str,
    pub label: &'static str,
    /// Steps that run unconditionally before the per-package checks.
    pub preamble: Vec<Step>,
    /// Individual package checks, each run in order.
    pub checks: Vec<Check>,
}

// ── runner ────────────────────────────────────────────────────────────────────

impl InstallSequence {
    pub fn run(&self, config: &Config) -> Result<()> {
        ui::header(format!(" Dimos — {}", self.label));

        // Preamble steps
        for step in &self.preamble {
            let sp = cliclack::spinner();
            sp.start(step.label);
            if let Err(e) = (step.run)(config) {
                sp.error(format!("{}: {e}", step.label));
                if std::io::stdin().is_terminal() {
                    let cont = cliclack::confirm("Preamble step failed. Continue anyway?")
                        .initial_value(false)
                        .interact()?;
                    if !cont {
                        anyhow::bail!("Aborted at preamble step '{}'.", step.label);
                    }
                } else {
                    return Err(e);
                }
            } else {
                sp.stop(format!("{} done", step.label));
            }
        }

        // Per-package checks
        for check in &self.checks {
            if let Err(e) = check.run(config) {
                if std::io::stdin().is_terminal() {
                    let cont =
                        cliclack::confirm(format!("'{}' failed ({e}). Continue?", check.label))
                            .initial_value(false)
                            .interact()?;
                    if !cont {
                        return Err(e);
                    }
                } else {
                    return Err(e);
                }
            }
        }

        cliclack::log::success(format!("{} — all packages ready", self.label))?;

        // Spawn a subshell so env changes (PATH additions, etc.) take effect.
        if std::io::stdin().is_terminal() {
            let shell = std::env::var("SHELL").unwrap_or_else(|_| "sh".to_string());
            ui::outro(format!(
                "Starting {shell} to activate env changes (type `exit` to return to parent shell)"
            ));
            Command::new(&shell).status()?;
        } else {
            ui::outro(format!("{} complete.", self.label));
        }

        Ok(())
    }
}
