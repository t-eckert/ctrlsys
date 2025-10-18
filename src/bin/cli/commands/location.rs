use anyhow::Result;
use ctrlsys::config::CliConfig;

use crate::LocationCommands;

pub async fn handle(command: LocationCommands, _config: &CliConfig) -> Result<()> {
    match command {
        LocationCommands::Add { name, tz, lat, lon } => {
            println!("Adding location '{}' with timezone {}", name, tz);
            if let (Some(lat), Some(lon)) = (lat, lon) {
                println!("Coordinates: {}, {}", lat, lon);
            }
            println!("(Not yet implemented)");
        }
        LocationCommands::List => {
            println!("Listing locations");
            println!("(Not yet implemented)");
        }
        LocationCommands::Time { name } => {
            match name {
                Some(n) => println!("Getting time for location: {}", n),
                None => println!("Getting time for all locations"),
            }
            println!("(Not yet implemented)");
        }
    }

    Ok(())
}
