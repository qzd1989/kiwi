use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::io::Read;

use crate::{extensions::EmbeddedFileExt as _, types::Asset};
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct PyProject {
    #[serde(rename = "build-system")]
    build_system: BuildSystem,

    pub project: Project,

    tool: Tool,
}
impl Default for PyProject {
    fn default() -> Self {
        let toml = Asset::get("python/packages/kiwi/pyproject.toml")
            .expect("get kiwi pyproject.toml failed.");
        let mut toml_content = String::new();
        toml.to_cursor()
            .read_to_string(&mut toml_content)
            .expect("read kiwi project toml failed");
        let pyproject = PyProject::load_from_toml_content(toml_content)
            .expect("load kiwi project toml failed.");
        pyproject
    }
}

impl PyProject {
    pub fn load_from_toml_content(toml_content: String) -> Result<PyProject> {
        let config: PyProject =
            toml::from_str(&toml_content).map_err(|error| anyhow!(error.to_string()))?;
        Ok(config)
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct BuildSystem {
    requires: Vec<String>,
    #[serde(rename = "build-backend")]
    build_backend: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Project {
    name: String,
    pub version: String,
    description: String,
    authors: Vec<Author>,
    readme: String,
    #[serde(rename = "requires-python")]
    requires_python: String,
    dependencies: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Author {
    name: String,
    email: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Tool {
    setuptools: ToolSetuptools,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ToolSetuptools {
    packages: Vec<String>,
}
