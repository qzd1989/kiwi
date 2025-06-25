use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub project: ConfigProject,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            project: ConfigProject::default(),
        }
    }
}

impl Config {
    pub fn new(
        name: impl AsRef<str>,
        language: impl AsRef<str>,
        edit_command: impl AsRef<str>,
        kiwi_version: impl AsRef<str>,
    ) -> Self {
        let mut config = Self::default();
        config.project.name = name.as_ref().to_string();
        config.project.language = language.as_ref().to_string();
        config.project.edit_command = edit_command.as_ref().to_string();
        config.project.kiwi_version = kiwi_version.as_ref().to_string();
        config
    }

    pub fn new_from_toml(project_path: impl AsRef<Path>) -> Result<Self> {
        let path = project_path.as_ref().join("config.toml");
        let content = fs::read_to_string(&path).map_err(|error| anyhow!(error.to_string()))?;
        let config: Config =
            toml::from_str(&content).map_err(|error| anyhow!(error.to_string()))?;

        Ok(Self {
            project: config.project,
        })
    }

    pub fn save(&self, project_path: impl AsRef<Path>) -> Result<()> {
        let path = project_path.as_ref().join("config.toml");
        let contents = toml::to_string_pretty(&self).map_err(|error| anyhow!(error.to_string()))?;
        fs::write(&path, contents).map_err(|error| anyhow!(error.to_string()))?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigProject {
    pub name: String,
    pub language: String,
    pub edit_command: String,
    pub kiwi_version: String,
}

impl Default for ConfigProject {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            language: "".to_string(),
            edit_command: "".to_string(),
            kiwi_version: "".to_string(),
        }
    }
}
