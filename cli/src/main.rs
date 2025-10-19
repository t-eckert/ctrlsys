use clap::{Parser, Subcommand};
use lib::config::CliConfig;

mod client;
mod commands;
mod tui;

#[derive(Parser)]
#[command(name = "cs")]
#[command(about = "ctrlsys - Your homelab swiss-army-knife tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Timer management
    Timer {
        #[command(subcommand)]
        command: TimerCommands,
    },
    /// Location and timezone management
    Location {
        #[command(subcommand)]
        command: LocationCommands,
    },
    /// Weather information
    Weather {
        #[command(subcommand)]
        command: WeatherCommands,
    },
    /// Task management
    Task {
        #[command(subcommand)]
        command: TaskCommands,
    },
    /// Project template management
    Template {
        #[command(subcommand)]
        command: TemplateCommands,
    },
    /// Database management
    Db {
        #[command(subcommand)]
        command: DatabaseCommands,
    },
    /// Configuration management
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
}

#[derive(Subcommand)]
enum TimerCommands {
    /// Create a new timer
    Create {
        /// Timer name
        name: String,
        /// Duration in seconds
        duration: i32,
    },
    /// List all timers
    List,
    /// Watch a timer (blocking, with TUI)
    Watch {
        /// Timer ID
        id: String,
    },
    /// Watch all active timers (blocking, with TUI)
    WatchAll,
}

#[derive(Subcommand)]
enum LocationCommands {
    /// Add a new location (auto-geocodes if tz not provided)
    Add {
        /// Location name
        name: String,
        /// Timezone (e.g., America/New_York) - auto-detected if not provided
        #[arg(short, long)]
        tz: Option<String>,
        /// Latitude - auto-detected if not provided
        #[arg(long)]
        lat: Option<f32>,
        /// Longitude - auto-detected if not provided
        #[arg(long)]
        lon: Option<f32>,
    },
    /// List all locations
    List,
    /// Get current time at location(s)
    Time {
        /// Location name (optional, shows all if not specified)
        name: Option<String>,
    },
    /// Watch all locations with live clocks (TUI)
    WatchAll,
}

#[derive(Subcommand)]
enum WeatherCommands {
    /// Get weather for a location
    Get {
        /// Location name (optional, shows all if not specified)
        name: Option<String>,
    },
    /// Watch weather for all locations (TUI)
    WatchAll,
}

#[derive(Subcommand)]
enum TaskCommands {
    /// Create a new task
    Create {
        /// Task title
        title: String,
        /// Task description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// List all tasks
    List,
    /// Start a timer on a task
    Start {
        /// Task ID
        id: String,
    },
    /// Complete a task
    Complete {
        /// Task ID
        id: String,
    },
}

#[derive(Subcommand)]
enum TemplateCommands {
    /// Create a new template
    Create {
        /// Template name
        name: String,
    },
    /// List all templates
    List,
    /// Use a template to create a new project
    Use {
        /// Template name
        name: String,
        /// Output path
        output: String,
    },
}

#[derive(Subcommand)]
enum DatabaseCommands {
    /// Create a new database
    Create {
        /// Database name
        name: String,
    },
    /// List all managed databases
    List,
    /// Drop a database
    Drop {
        /// Database name
        name: String,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Show current configuration
    Show,
    /// Set server URL
    SetServer {
        /// Server URL
        url: String,
    },
    /// Set API token
    SetToken {
        /// API token
        token: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Load configuration
    let mut config = CliConfig::load()?;

    match cli.command {
        Commands::Config { command } => commands::config::handle(command, &mut config).await?,
        Commands::Timer { command } => commands::timer::handle(command, &config).await?,
        Commands::Location { command } => commands::location::handle(command, &config).await?,
        Commands::Weather { command } => commands::weather::handle(command, &config).await?,
        Commands::Task { command } => commands::task::handle(command, &config).await?,
        Commands::Template { command } => commands::template::handle(command, &config).await?,
        Commands::Db { command } => commands::database::handle(command, &config).await?,
    }

    Ok(())
}
