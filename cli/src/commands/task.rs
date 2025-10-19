use anyhow::Result;
use lib::config::CliConfig;

use crate::TaskCommands;

pub async fn handle(command: TaskCommands, _config: &CliConfig) -> Result<()> {
    match command {
        TaskCommands::Create { title, description } => {
            println!("Creating task '{}'", title);
            if let Some(desc) = description {
                println!("Description: {}", desc);
            }
            println!("(Not yet implemented)");
        }
        TaskCommands::List => {
            println!("Listing tasks");
            println!("(Not yet implemented)");
        }
        TaskCommands::Start { id } => {
            println!("Starting timer on task {}", id);
            println!("(Not yet implemented)");
        }
        TaskCommands::Complete { id } => {
            println!("Completing task {}", id);
            println!("(Not yet implemented)");
        }
    }

    Ok(())
}
