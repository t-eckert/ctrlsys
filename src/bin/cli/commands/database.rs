use anyhow::Result;
use ctrlsys::config::CliConfig;

use crate::DatabaseCommands;

pub async fn handle(command: DatabaseCommands, _config: &CliConfig) -> Result<()> {
    match command {
        DatabaseCommands::Create { name } => {
            println!("Creating database '{}'", name);
            println!("(Not yet implemented)");
        }
        DatabaseCommands::List => {
            println!("Listing databases");
            println!("(Not yet implemented)");
        }
        DatabaseCommands::Drop { name } => {
            println!("Dropping database '{}'", name);
            println!("(Not yet implemented)");
        }
    }

    Ok(())
}
