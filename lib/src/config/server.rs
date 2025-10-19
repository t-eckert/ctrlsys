use super::ServerConfig;
use anyhow::Result;
use std::env;

impl ServerConfig {
    /// Load config from environment variables
    pub fn load() -> Result<Self> {
        let port = env::var("CTRLSYS_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000);

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| anyhow::anyhow!("DATABASE_URL environment variable not set"))?;

        let api_tokens_str = env::var("CTRLSYS_API_TOKENS")
            .unwrap_or_else(|_| String::new());

        let api_tokens = api_tokens_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let weather_api_key = env::var("OPENWEATHER_API_KEY").ok();

        Ok(Self {
            port,
            database_url,
            api_tokens,
            weather_api_key,
        })
    }
}
