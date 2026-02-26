use anyhow::Result;

mod git;
mod git_lfs;
mod nix;
mod nix_flakes;
mod uv;

pub use git::check as git;
pub use git_lfs::check as git_lfs;
pub use nix::check as nix;
pub use nix_flakes::check as nix_flakes;
pub use uv::check as uv;

/// Called when a check fails and the user declined or no autofix is available.
/// Returns a human-readable string with manual fix instructions.
pub type FixInstructions = fn() -> String;

pub struct Autofix {
    /// Confirmation prompt shown before attempting the fix.
    pub prompt: &'static str,
    /// Run the automated fix. Returns `Ok(())` on success.
    pub run: fn() -> Result<()>,
}

pub struct Check {
    /// Short label used in spinner messages, e.g. `"nix"` or `"nix flakes"`.
    pub label: &'static str,
    /// Returns `true` if the dependency is already present/satisfied.
    pub detect: fn() -> bool,
    /// Optional callback that returns manual fix instructions shown on failure.
    pub fix_instructions: Option<FixInstructions>,
    /// Optional automated fix offered to the user when detection fails.
    pub autofix: Option<Autofix>,
}

impl Check {
    /// Run the check: detect → (if failing) offer autofix → show instructions → bail.
    pub fn run(&self) -> Result<()> {
        let sp = cliclack::spinner();
        sp.start(format!("Checking {}…", self.label));

        if (self.detect)() {
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
                match (fix.run)() {
                    Ok(()) => {
                        sp2.stop(format!("{} ready", self.label));
                        return Ok(());
                    }
                    Err(e) => {
                        sp2.error(format!("Auto-fix failed for {}", self.label));
                        if let Some(instructions) = self.fix_instructions {
                            cliclack::log::info(instructions())?;
                        }
                        return Err(e);
                    }
                }
            }
        }

        if let Some(instructions) = self.fix_instructions {
            cliclack::log::info(instructions())?;
        }

        anyhow::bail!(
            "{} is required — install it then re-run `dimos init`.",
            self.label
        )
    }
}
