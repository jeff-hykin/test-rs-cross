use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub init_completed: bool,
    #[serde(default)]
    pub personality: Personality,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Personality {
    #[serde(default)]
    pub editor: String,
    #[serde(default)]
    pub indentation: String,
    #[serde(default)]
    pub primary_language: String,
    #[serde(default)]
    pub schedule: String,
    #[serde(default)]
    pub debug_style: String,
}

pub fn config_path() -> PathBuf {
    dirs::home_dir()
        .expect("cannot determine home directory")
        .join(".dimos")
        .join("config.yaml")
}

pub fn load() -> Result<Config> {
    let path = config_path();
    if !path.exists() {
        return Ok(Config::default());
    }
    let text = fs::read_to_string(&path)
        .with_context(|| format!("cannot read {}", path.display()))?;
    serde_yaml::from_str(&text).context("cannot parse config")
}

pub fn save(config: &Config) -> Result<()> {
    let path = config_path();
    fs::create_dir_all(path.parent().unwrap())
        .with_context(|| format!("cannot create {}", path.parent().unwrap().display()))?;
    let text = serde_yaml::to_string(config)?;
    fs::write(&path, text)
        .with_context(|| format!("cannot write {}", path.display()))
}
