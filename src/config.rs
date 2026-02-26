use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, io::IsTerminal, path::PathBuf};

use crate::questions::QuestionKey;

// ── data model ────────────────────────────────────────────────────────────────

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub init_completed: bool,
    /// Flat key→value store for all question answers, keyed by QuestionKey::ADDRESS.
    #[serde(default)]
    pub answers: HashMap<String, String>,
}

// ── manager ───────────────────────────────────────────────────────────────────

pub struct ConfigManager {
    path: PathBuf,
    pub config: Config,
}

impl ConfigManager {
    /// Load the config file, recovering gracefully if it is corrupt.
    pub fn load_or_recover() -> Result<Self> {
        let path = config_path();
        if !path.exists() {
            return Ok(Self {
                path,
                config: Config::default(),
            });
        }

        let text = fs::read_to_string(&path)
            .with_context(|| format!("cannot read {}", path.display()))?;

        match serde_yaml::from_str::<Config>(&text) {
            Ok(config) => Ok(Self { path, config }),
            Err(e) => Self::recover(path, &text, e),
        }
    }

    fn recover(path: PathBuf, corrupt_text: &str, err: serde_yaml::Error) -> Result<Self> {
        let backup = path.with_extension("yaml.corrupt.bak");

        let reset = if std::io::stdin().is_terminal() {
            cliclack::confirm(format!(
                "Config is corrupt ({err}).\nReset to defaults? (corrupt file → {})",
                backup.display()
            ))
            .initial_value(true)
            .interact()?
        } else {
            // Non-interactive: auto-recover silently.
            true
        };

        if !reset {
            anyhow::bail!(
                "Config is corrupt. Fix or delete {} and re-run.",
                path.display()
            );
        }

        if let Some(parent) = backup.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&backup, corrupt_text)
            .with_context(|| format!("cannot write backup to {}", backup.display()))?;

        cliclack::log::warning(format!(
            "Corrupt config backed up to {}. Starting fresh.",
            backup.display()
        ))?;

        Ok(Self {
            path,
            config: Config::default(),
        })
    }

    pub fn save(&self) -> Result<()> {
        if let Some(dir) = self.path.parent() {
            fs::create_dir_all(dir)
                .with_context(|| format!("cannot create {}", dir.display()))?;
        }
        let text = serde_yaml::to_string(&self.config)?;
        fs::write(&self.path, text)
            .with_context(|| format!("cannot write {}", self.path.display()))
    }

    /// Read a question answer by its compile-time key.
    pub fn get<Q: QuestionKey>(&self) -> Option<&str> {
        self.config.answers.get(Q::ADDRESS).map(String::as_str)
    }

    /// Store a question answer by its compile-time key.
    pub fn set<Q: QuestionKey>(&mut self, value: String) {
        self.config.answers.insert(Q::ADDRESS.to_string(), value);
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

pub fn config_path() -> PathBuf {
    dirs::home_dir()
        .expect("cannot determine home directory")
        .join(".dimos")
        .join("config.yaml")
}

/// Convenience loader for subcommands that only need a read-only snapshot.
pub fn load() -> Result<Config> {
    Ok(ConfigManager::load_or_recover()?.config)
}
