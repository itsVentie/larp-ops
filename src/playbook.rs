use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
    pub name: String,
    pub command: String,
    pub module: String,
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Playbook {
    pub name: String,
    pub description: Option<String>,
    pub steps: Vec<Step>,
}

impl Playbook {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read playbook file at {:?}", path.as_ref()))?;

        let playbook: Playbook = serde_yaml::from_str(&content)
            .with_context(|| "Failed to parse YAML playbook schema")?;

        Ok(playbook)
    }
}
