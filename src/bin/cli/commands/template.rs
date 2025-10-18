use anyhow::Result;
use ctrlsys::config::CliConfig;

use crate::TemplateCommands;

pub async fn handle(command: TemplateCommands, _config: &CliConfig) -> Result<()> {
    match command {
        TemplateCommands::Create { name } => {
            println!("Creating template '{}'", name);
            println!("(Not yet implemented)");
        }
        TemplateCommands::List => {
            println!("Listing templates");
            println!("(Not yet implemented)");
        }
        TemplateCommands::Use { name, output } => {
            println!("Using template '{}' to create project at {}", name, output);
            println!("(Not yet implemented)");
        }
    }

    Ok(())
}
