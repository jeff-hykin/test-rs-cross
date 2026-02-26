use anyhow::Result;

use crate::{checks, config, config::Personality, ui};

pub fn run() -> Result<()> {
    ui::header(" Dimos — Environment Setup");

    let personality = ask_personality()?;

    checks::nix().run()?;
    checks::nix_flakes().run()?;
    checks::git().run()?;
    checks::git_lfs().run()?;
    checks::uv().run()?;

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
