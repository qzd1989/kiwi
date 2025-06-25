use super::App;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub app: ConfigApp,
}

impl Default for Config {
    fn default() -> Self {
        let toml_path = get_config_toml_path();
        let content = fs::read_to_string(&toml_path).unwrap();
        let config: Config = toml::from_str(&content).unwrap();
        config
    }
}

impl Config {
    pub fn save(&self) -> Result<bool> {
        let toml_path = get_config_toml_path();
        let contents = toml::to_string_pretty(&self).map_err(|error| anyhow!(error.to_string()))?;
        fs::write(&toml_path, contents).map_err(|error| anyhow!(error.to_string()))?;
        Ok(true)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigApp {
    pub websocket_port: u16,
}

pub fn get_config_toml_path() -> PathBuf {
    App::get_resource_dir().join("config.toml")
}
