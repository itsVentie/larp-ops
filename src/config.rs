use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolConfig {
    pub path: String,
    #[serde(default = "default_timeout")]
    pub timeout: u64,
}

fn default_timeout() -> u64 {
    300
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub tools: HashMap<String, ToolConfig>,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        if !config_path.exists() {
            Self::create_default_config(&config_path)?;
        }

        let config_str = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file at {:?}", config_path))?;

        let config: AppConfig = serde_yaml::from_str(&config_str)
            .with_context(|| format!("Failed to parse YAML config at {:?}", config_path))?;

        Ok(config)
    }

    fn get_config_path() -> Result<PathBuf> {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "larp") {
            let config_dir = proj_dirs.config_dir();
            fs::create_dir_all(config_dir)?;
            Ok(config_dir.join("config.yaml"))
        } else {
            anyhow::bail!("Could not determine OS config directory");
        }
    }

    fn create_default_config(path: &PathBuf) -> Result<()> {
        let default_content = r#"# LarpOps Configuration
tools:
  evtx_dump:
    path: "evtx_dump.exe"
    timeout: 300
  scanner:
    path: "nmap.exe"
    timeout: 600
"#;
        fs::write(path, default_content)?;
        Ok(())
    }
}
