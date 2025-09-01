use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::package::Package;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub packages: Vec<Package>,
}

pub fn get_config_path() -> anyhow::Result<PathBuf> {
    Ok(dirs::data_dir().ok_or_else(|| anyhow!("Could not find data directory"))?
        .join("kura")
        .join("config.toml"))
}

pub fn load_config() -> anyhow::Result<Config> {
    let config_path = get_config_path()?;
    if config_path.exists() {
        let content = fs::read_to_string(&config_path).unwrap_or_default();
        Ok(toml::from_str(&content).unwrap_or_default())
    } else {
        Ok(Config::default())
    }
}

pub fn save_config(config: &Config) -> anyhow::Result<()> {
    let config_path = get_config_path()?;
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = toml::to_string_pretty(config)?;
    fs::write(&config_path, content)?;
    Ok(())
}
