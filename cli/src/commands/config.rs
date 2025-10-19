use anyhow::Result;
use lib::config::CliConfig;

use crate::ConfigCommands;

pub async fn handle(command: ConfigCommands, config: &mut CliConfig) -> Result<()> {
    match command {
        ConfigCommands::Show => {
            println!("Server URL: {}", config.server_url);
            println!("API Token: {}", if config.api_token.is_empty() { "(not set)" } else { "********" });
        }
        ConfigCommands::SetServer { url } => {
            config.server_url = url.clone();
            config.save()?;
            println!("Server URL set to: {}", url);
        }
        ConfigCommands::SetToken { token } => {
            config.api_token = token;
            config.save()?;
            println!("API token updated");
        }
    }

    Ok(())
}
