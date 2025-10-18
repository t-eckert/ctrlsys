use anyhow::{Context, Result};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ctrlsys::config::CliConfig;
use futures_util::StreamExt;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Terminal,
};
use serde::Deserialize;
use std::io;
use tokio::time::{timeout, Duration};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct TimerResponse {
    id: Uuid,
    name: String,
    duration_seconds: i32,
    status: String,
    remaining_seconds: Option<i32>,
}

pub async fn run(config: &CliConfig, timer_id: Uuid) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app
    let res = run_app(&mut terminal, config, timer_id).await;

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
    timer_id: Uuid,
) -> Result<()> {
    // Connect to WebSocket
    let ws_url = format!(
        "{}/api/v1/timers/{}/ws",
        config.server_url.replace("http", "ws"),
        timer_id
    );

    let (ws_stream, _) = connect_async(&ws_url)
        .await
        .context("Failed to connect to WebSocket")?;

    let (_write, mut read) = ws_stream.split();

    let mut current_timer: Option<TimerResponse> = None;

    loop {
        // Check for keyboard events (non-blocking)
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        // Try to read from WebSocket (with timeout)
        match timeout(Duration::from_millis(100), read.next()).await {
            Ok(Some(Ok(msg))) => {
                if let Message::Text(text) = msg {
                    if let Ok(timer) = serde_json::from_str::<TimerResponse>(&text) {
                        current_timer = Some(timer);
                    }
                }
            }
            Ok(Some(Err(e))) => {
                return Err(anyhow::anyhow!("WebSocket error: {:?}", e));
            }
            Ok(None) => {
                // WebSocket closed
                break;
            }
            Err(_) => {
                // Timeout - no new data, just redraw
            }
        }

        // Draw the UI
        terminal.draw(|f| {
            let size = f.area();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Min(1),
                    ]
                    .as_ref(),
                )
                .split(size);

            // Title
            let title = Paragraph::new("Timer Watch Mode")
                .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, chunks[0]);

            if let Some(ref timer) = current_timer {
                // Timer name and status
                let info = Paragraph::new(Line::from(vec![
                    Span::raw("Name: "),
                    Span::styled(&timer.name, Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("  Status: "),
                    Span::styled(
                        &timer.status,
                        Style::default()
                            .fg(match timer.status.as_str() {
                                "running" => Color::Green,
                                "completed" => Color::Blue,
                                "cancelled" => Color::Red,
                                _ => Color::Gray,
                            })
                            .add_modifier(Modifier::BOLD),
                    ),
                ]))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
                f.render_widget(info, chunks[1]);

                // Progress bar
                if let Some(remaining) = timer.remaining_seconds {
                    let progress = 1.0
                        - (remaining as f64 / timer.duration_seconds.max(1) as f64);
                    let gauge = Gauge::default()
                        .block(Block::default().borders(Borders::ALL).title("Progress"))
                        .gauge_style(Style::default().fg(Color::Green))
                        .ratio(progress.max(0.0).min(1.0))
                        .label(format!(
                            "{} / {} seconds",
                            timer.duration_seconds - remaining,
                            timer.duration_seconds
                        ));
                    f.render_widget(gauge, chunks[2]);
                } else {
                    let gauge = Gauge::default()
                        .block(Block::default().borders(Borders::ALL).title("Progress"))
                        .gauge_style(Style::default().fg(Color::Blue))
                        .ratio(1.0)
                        .label("Completed");
                    f.render_widget(gauge, chunks[2]);
                }

                // Remaining time (large display)
                let remaining_text = if let Some(remaining) = timer.remaining_seconds {
                    format_time(remaining)
                } else {
                    "DONE!".to_string()
                };

                let time_display = Paragraph::new(remaining_text)
                    .style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    )
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Remaining Time"),
                    );
                f.render_widget(time_display, chunks[3]);
            } else {
                let loading = Paragraph::new("Loading timer...")
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL));
                f.render_widget(loading, chunks[1]);
            }

            // Help text at bottom
            let help = Paragraph::new("Press 'q' to quit")
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center);
            let help_area = ratatui::layout::Rect {
                x: size.x,
                y: size.height.saturating_sub(1),
                width: size.width,
                height: 1,
            };
            f.render_widget(help, help_area);
        })?;

        // Check if timer is done
        if let Some(ref timer) = current_timer {
            if timer.status == "completed" || timer.status == "cancelled" {
                // Give user a moment to see the final state
                tokio::time::sleep(Duration::from_secs(2)).await;
                break;
            }
        }
    }

    Ok(())
}

fn format_time(seconds: i32) -> String {
    let mins = seconds / 60;
    let secs = seconds % 60;
    format!("{:02}:{:02}", mins, secs)
}
