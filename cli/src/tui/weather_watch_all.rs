use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use lib::config::CliConfig;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Terminal,
};
use serde::Deserialize;
use std::io;
use tokio::time::{sleep, Duration};

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    location_name: String,
    temperature_celsius: f32,
    temperature_fahrenheit: f32,
    feels_like_celsius: f32,
    humidity: u8,
    description: String,
    wind_speed_ms: f32,
    wind_speed_mph: f32,
}

pub async fn run(config: &CliConfig) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app
    let res = run_app(&mut terminal, config).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    config: &CliConfig,
) -> Result<()> {
    loop {
        // Check for keyboard events (non-blocking)
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        // Fetch weather from API
        let client = reqwest::Client::new();
        let url = format!("{}/api/v1/weather/locations", config.server_url);

        let weather_list = match client
            .get(&url)
            .header("Authorization", format!("Bearer {}", config.api_token))
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    response
                        .json::<Vec<WeatherResponse>>()
                        .await
                        .unwrap_or_default()
                } else {
                    vec![]
                }
            }
            Err(_) => vec![],
        };

        // Draw the UI
        terminal.draw(|f| {
            let size = f.area();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(1),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .split(size);

            // Title
            let title = Paragraph::new("Weather Dashboard")
                .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, chunks[0]);

            // Weather table
            if weather_list.is_empty() {
                let no_weather = Paragraph::new("No weather data available.\nMake sure locations have latitude and longitude set.")
                    .style(Style::default().fg(Color::DarkGray))
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL));
                f.render_widget(no_weather, chunks[1]);
            } else {
                let header = Row::new(vec![
                    "Location",
                    "Temp",
                    "Feels Like",
                    "Conditions",
                    "Humidity",
                    "Wind",
                ])
                .style(Style::default().add_modifier(Modifier::BOLD))
                .bottom_margin(1);

                let rows: Vec<Row> = weather_list
                    .iter()
                    .map(|weather| {
                        Row::new(vec![
                            weather.location_name.clone(),
                            format!("{:.1}C/{:.1}F",
                                weather.temperature_celsius,
                                weather.temperature_fahrenheit),
                            format!("{:.1}C", weather.feels_like_celsius),
                            weather.description.clone(),
                            format!("{}%", weather.humidity),
                            format!("{:.1}mph", weather.wind_speed_mph),
                        ])
                        .style(Style::default().fg(Color::Green))
                    })
                    .collect();

                let table = Table::new(
                    rows,
                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(15),
                        Constraint::Percentage(12),
                        Constraint::Percentage(25),
                        Constraint::Percentage(13),
                        Constraint::Percentage(15),
                    ],
                )
                .header(header)
                .block(Block::default().borders(Borders::ALL).title("Weather"));

                f.render_widget(table, chunks[1]);
            }

            // Help text at bottom
            let help = Paragraph::new("Press 'q' to quit | Updates every 30 seconds")
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center);
            f.render_widget(help, chunks[2]);
        })?;

        // Sleep before next update (30 seconds to avoid API rate limits)
        sleep(Duration::from_secs(30)).await;
    }

    Ok(())
}
