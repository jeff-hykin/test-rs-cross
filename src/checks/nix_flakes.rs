use anyhow::Result;
use std::{fs, path::PathBuf};

use super::{Autofix, Check};

pub fn check() -> Check {
    Check {
        label: "nix flakes",
        detect,
        fix_instructions: Some(fix_instructions),
        autofix: Some(Autofix {
            prompt: "Enable nix flakes in ~/.config/nix/nix.conf?",
            run: autofix,
        }),
    }
}

fn detect() -> bool {
    let candidates: Vec<PathBuf> = [
        dirs::home_dir().map(|h| h.join(".config/nix/nix.conf")),
        Some(PathBuf::from("/etc/nix/nix.conf")),
    ]
    .into_iter()
    .flatten()
    .collect();

    for path in candidates {
        if let Ok(content) = fs::read_to_string(&path) {
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("experimental-features") && trimmed.contains("flakes") {
                    return true;
                }
            }
        }
    }
    false
}

fn fix_instructions() -> String {
    "Add `experimental-features = nix-command flakes` to ~/.config/nix/nix.conf".to_string()
}

fn autofix() -> Result<()> {
    let conf_dir = dirs::home_dir()
        .expect("no home dir")
        .join(".config/nix");
    fs::create_dir_all(&conf_dir)?;
    let conf_path = conf_dir.join("nix.conf");

    let existing = if conf_path.exists() {
        fs::read_to_string(&conf_path)?
    } else {
        String::new()
    };

    let new_content = if existing
        .lines()
        .any(|l| l.trim().starts_with("experimental-features"))
    {
        existing
            .lines()
            .map(|line| {
                if line.trim().starts_with("experimental-features") && !line.contains("flakes") {
                    format!("{} flakes", line.trim_end())
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
            + "\n"
    } else {
        format!(
            "{}{}experimental-features = nix-command flakes\n",
            existing,
            if existing.ends_with('\n') || existing.is_empty() {
                ""
            } else {
                "\n"
            }
        )
    };

    fs::write(&conf_path, new_content)?;
    cliclack::log::warning(
        "You may need to restart the nix daemon: `sudo systemctl restart nix-daemon`",
    )?;
    Ok(())
}
