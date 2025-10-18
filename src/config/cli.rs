use super::{CliConfig, cli_config_path};
use anyhow::Result;
use std::fs;

impl CliConfig {
    /// Load config from the default location
    pub fn load() -> Result<Self> {
        let path = cli_config_path()?;

        if !path.exists() {
            // Create default config file
            let default_config = Self::default();
            default_config.save()?;
            return Ok(default_config);
        }

        let contents = fs::read_to_string(&path)?;
        let config: CliConfig = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Save config to the default location
    pub fn save(&self) -> Result<()> {
        let path = cli_config_path()?;
        let contents = toml::to_string_pretty(self)?;
        fs::write(&path, contents)?;
        Ok(())
    }
}
