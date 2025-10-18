use anyhow::Result;
use ctrlsys::config::CliConfig;

use crate::TimerCommands;

pub async fn handle(command: TimerCommands, _config: &CliConfig) -> Result<()> {
    match command {
        TimerCommands::Create { name, duration } => {
            println!("Creating timer '{}' with duration {} seconds", name, duration);
            println!("(Not yet implemented)");
        }
        TimerCommands::List => {
            println!("Listing timers");
            println!("(Not yet implemented)");
        }
        TimerCommands::Watch { id } => {
            println!("Watching timer {}", id);
            println!("(Not yet implemented)");
        }
    }

    Ok(())
}
