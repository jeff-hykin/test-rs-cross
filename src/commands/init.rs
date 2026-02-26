use anyhow::{bail, Result};
use std::{fs, path::PathBuf, process::Command};
use which::which;

use crate::{config, config::Personality, ui};

pub fn run() -> Result<()> {
    ui::header(" Dimos — Environment Setup");

    let personality = ask_personality()?;

    check_nix()?;
    check_nix_flakes()?;
    check_tool("git", "nixpkgs#git")?;
    check_tool("git-lfs", "nixpkgs#git-lfs")?;
    check_uv()?;

    let mut cfg = config::load()?;
    cfg.init_completed = true;
    cfg.personality = personality;
    config::save(&cfg)?;

    cliclack::log::success(format!(
        "Config saved → {}",
        config::config_path().display()
    ))?;

    ui::outro(" All set! Run `dimos new-app` to scaffold a project.");
    Ok(())
}

// ── personality questions ─────────────────────────────────────────────────────

fn ask_personality() -> Result<Personality> {
    cliclack::note(
        "Quick intro",
        "A few questions so Dimos can tailor its suggestions to you.",
    )?;

    let editor: &str = cliclack::select("What's your editor of choice?")
        .item("neovim", "Neovim", "")
        .item("emacs", "Emacs", "")
        .item("vscode", "VS Code", "")
        .item("zed", "Zed", "")
        .item("other", "Other / CLI", "")
        .interact()?;

    let indentation: &str = cliclack::select("Tabs or spaces?")
        .item("spaces", "Spaces", "")
        .item("tabs", "Tabs", "heresy")
        .interact()?;

    let primary_language: String = cliclack::input("Primary programming language?")
        .placeholder("e.g. Python, Rust, TypeScript")
        .interact()?;

    let schedule: &str = cliclack::select("When do you do your best work?")
        .item("morning", "Early bird", "up before the coffee")
        .item("night", "Night owl", "when everyone else is asleep")
        .item("whenever", "Whenever the flow hits", "chaos schedule")
        .interact()?;

    let debug_style: &str = cliclack::select("How do you debug?")
        .item("prints", "Print statements", "the classic")
        .item("debugger", "Proper debugger", "breakpoints and watches")
        .item("rubber_duck", "Rubber duck", "talking it through")
        .item("rewrite", "Rewrite until it works", "burn it down")
        .interact()?;

    Ok(Personality {
        editor: editor.to_string(),
        indentation: indentation.to_string(),
        primary_language,
        schedule: schedule.to_string(),
        debug_style: debug_style.to_string(),
    })
}

// ── nix ──────────────────────────────────────────────────────────────────────

fn check_nix() -> Result<()> {
    let sp = cliclack::spinner();
    sp.start("Checking for nix…");
    let found = which("nix").is_ok();

    if found {
        sp.stop("nix is installed");
        return Ok(());
    }

    sp.error("nix not found");

    let yes = cliclack::confirm("Install nix via the Determinate Systems installer?")
        .initial_value(true)
        .interact()?;

    if !yes {
        bail!("nix is required — install it then re-run `dimos init`.");
    }

    let sp = cliclack::spinner();
    sp.start("Running Determinate Systems installer…");
    let status = Command::new("sh")
        .args([
            "-c",
            "curl --proto '=https' --tlsv1.2 -sSf -L \
             https://install.determinate.systems/nix | sh -s -- install",
        ])
        .status()?;

    if !status.success() {
        sp.error("Installation failed");
        bail!("nix installation failed — install manually then re-run `dimos init`.");
    }

    sp.stop("nix installed");
    cliclack::log::warning("Open a new terminal so nix is on PATH before continuing.")?;
    Ok(())
}

// ── nix flakes ───────────────────────────────────────────────────────────────

fn check_nix_flakes() -> Result<()> {
    let sp = cliclack::spinner();
    sp.start("Checking nix flakes…");
    let enabled = flakes_enabled();

    if enabled {
        sp.stop("nix flakes are enabled");
        return Ok(());
    }

    sp.error("nix flakes are not enabled");

    let yes = cliclack::confirm("Enable nix flakes in ~/.config/nix/nix.conf?")
        .initial_value(true)
        .interact()?;

    if !yes {
        bail!("nix flakes are required — enable them then re-run `dimos init`.");
    }

    enable_flakes()?;
    cliclack::log::success("nix flakes enabled")?;
    cliclack::log::warning(
        "You may need to restart the nix daemon: `sudo systemctl restart nix-daemon`",
    )?;
    Ok(())
}

fn flakes_enabled() -> bool {
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

fn enable_flakes() -> Result<()> {
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
    Ok(())
}

// ── generic tool via `nix profile install` ───────────────────────────────────

fn check_tool(cmd: &str, nix_pkg: &str) -> Result<()> {
    let sp = cliclack::spinner();
    sp.start(format!("Checking for {cmd}…"));
    let found = which(cmd).is_ok();

    if found {
        sp.stop(format!("{cmd} is installed"));
        return Ok(());
    }

    sp.error(format!("{cmd} not found"));

    let yes = cliclack::confirm(format!(
        "Install {cmd} via `nix profile install {nix_pkg}`?"
    ))
    .initial_value(true)
    .interact()?;

    if !yes {
        bail!("{cmd} is required — install it then re-run `dimos init`.");
    }

    let sp = cliclack::spinner();
    sp.start(format!("Running: nix profile install {nix_pkg}"));
    let status = Command::new("nix")
        .args(["profile", "install", nix_pkg])
        .status()?;

    if !status.success() {
        sp.error(format!("Failed to install {cmd}"));
        bail!("Failed to install {cmd} — install it manually then re-run `dimos init`.");
    }

    sp.stop(format!("{cmd} installed"));
    Ok(())
}

// ── uv ───────────────────────────────────────────────────────────────────────

fn check_uv() -> Result<()> {
    let sp = cliclack::spinner();
    sp.start("Checking for uv…");
    let found = which("uv").is_ok();

    if found {
        sp.stop("uv is installed");
        return Ok(());
    }

    sp.error("uv not found");

    let yes = cliclack::confirm("Install uv via the official installer (astral.sh)?")
        .initial_value(true)
        .interact()?;

    if !yes {
        bail!("uv is required — install it then re-run `dimos init`.");
    }

    let sp = cliclack::spinner();
    sp.start("Running: curl -LsSf https://astral.sh/uv/install.sh | sh");
    let status = Command::new("sh")
        .args(["-c", "curl -LsSf https://astral.sh/uv/install.sh | sh"])
        .status()?;

    if !status.success() {
        sp.error("Installation failed");
        bail!("Failed to install uv — install it manually then re-run `dimos init`.");
    }

    sp.stop("uv installed");
    Ok(())
}
