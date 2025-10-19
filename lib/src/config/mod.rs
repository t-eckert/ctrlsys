use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod cli;
pub mod server;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliConfig {
    pub server_url: String,
    pub api_token: String,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            server_url: "http://localhost:3000".to_string(),
            api_token: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub database_url: String,
    pub api_tokens: Vec<String>,
    pub weather_api_key: Option<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            database_url: String::new(),
            api_tokens: vec![],
            weather_api_key: None,
        }
    }
}

/// Get the path to the CLI config file
pub fn cli_config_path() -> anyhow::Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;

    let ctrlsys_dir = config_dir.join("ctrlsys");
    std::fs::create_dir_all(&ctrlsys_dir)?;

    Ok(ctrlsys_dir.join("config.toml"))
}
