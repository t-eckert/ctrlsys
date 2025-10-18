use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ctrlsys::config::CliConfig;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Terminal,
};
use serde::Deserialize;
use std::io;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct TimerResponse {
    id: Uuid,
    name: String,
    duration_seconds: i32,
    status: String,
    remaining_seconds: Option<i32>,
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

        // Fetch timers from API
        let client = reqwest::Client::new();
        let url = format!("{}/api/v1/timers", config.server_url);

        let timers = match client
            .get(&url)
            .header("Authorization", format!("Bearer {}", config.api_token))
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    response.json::<Vec<TimerResponse>>().await.unwrap_or_default()
                } else {
                    vec![]
                }
            }
            Err(_) => vec![],
        };

        // Filter to only running timers
        let running_timers: Vec<_> = timers
            .into_iter()
            .filter(|t| t.status == "running")
            .collect();

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
            let title = Paragraph::new("Active Timers")
                .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, chunks[0]);

            // Timers table
            if running_timers.is_empty() {
                let no_timers = Paragraph::new("No active timers")
                    .style(Style::default().fg(Color::DarkGray))
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL));
                f.render_widget(no_timers, chunks[1]);
            } else {
                let header = Row::new(vec!["Name", "Duration", "Remaining", "Status"])
                    .style(Style::default().add_modifier(Modifier::BOLD))
                    .bottom_margin(1);

                let rows: Vec<Row> = running_timers
                    .iter()
                    .map(|timer| {
                        let remaining = timer
                            .remaining_seconds
                            .map(|s| format_time(s))
                            .unwrap_or_else(|| "--:--".to_string());

                        Row::new(vec![
                            timer.name.clone(),
                            format_time(timer.duration_seconds),
                            remaining,
                            timer.status.clone(),
                        ])
                        .style(Style::default().fg(Color::Green))
                    })
                    .collect();

                let table = Table::new(
                    rows,
                    [
                        Constraint::Percentage(40),
                        Constraint::Percentage(20),
                        Constraint::Percentage(20),
                        Constraint::Percentage(20),
                    ],
                )
                .header(header)
                .block(Block::default().borders(Borders::ALL).title("Timers"));

                f.render_widget(table, chunks[1]);
            }

            // Help text at bottom
            let help = Paragraph::new("Press 'q' to quit")
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center);
            f.render_widget(help, chunks[2]);
        })?;

        // Sleep a bit before next update
        sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}

fn format_time(seconds: i32) -> String {
    let mins = seconds / 60;
    let secs = seconds % 60;
    format!("{:02}:{:02}", mins, secs)
}
