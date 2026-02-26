use anyhow::Result;

use crate::{config::ConfigManager, questions::*, ui};

pub fn run() -> Result<()> {
    ui::header(" Dimos — Survey");

    let mut mgr = ConfigManager::load_or_recover()?;

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

    mgr.set::<PersonalityEditor>(editor.to_string());
    mgr.set::<PersonalityIndentation>(indentation.to_string());
    mgr.set::<PersonalityLanguage>(primary_language);
    mgr.set::<PersonalitySchedule>(schedule.to_string());
    mgr.set::<PersonalityDebugStyle>(debug_style.to_string());

    mgr.save()?;
    cliclack::log::success(format!("Saved → {}", mgr.path().display()))?;

    ui::outro(" All done!");
    Ok(())
}
