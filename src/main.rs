mod app;
mod simulations;
mod ui;

use crate::app::App;
use crate::ui::ui;
use app::CurrentScreen;
use crossterm::event::{self, Event, KeyCode};
use ratatui::DefaultTerminal;
use simulations::ant::AntSim;
use std::io::{self};
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App::new();
    let app_result = run_app(&mut terminal, &mut app);

    ratatui::restore();

    app_result
}

fn run_app(terminal: &mut DefaultTerminal, app: &mut App) -> io::Result<()> {
    loop {
        // Render
        terminal.draw(|f| ui(f, app))?;

        // Event handling
        match app.current_screen {
            CurrentScreen::Exit => break Ok(()),
            CurrentScreen::Ant => {
                if app.is_running && !app.help_screen {
                    // Run Langton's Ant
                    AntSim::run_ant_sim(app);
                }
            }
            _ => {}
        }

        // Only run when a key is pressed
        if !is_event_available(app.speed)? {
            continue;
        }

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }

            if app.help_screen {
                // Prevent any action when the help screen is displayed
                app.help_screen = false;
            } else {
                match app.current_screen {
                    CurrentScreen::Ant => match key.code {
                        KeyCode::Char('q') => app.current_screen = CurrentScreen::Exit,
                        KeyCode::Char(' ') => app.is_running = !app.is_running,
                        KeyCode::Char('?') => {
                            app.help_screen = !app.help_screen;
                        }
                        KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('L') => {
                            AntSim::run_ant_sim(app);
                        }
                        KeyCode::Up | KeyCode::Char('j') | KeyCode::Char('J') => {
                            app.speed = app.speed.saturating_sub(Duration::from_millis(10));
                        }
                        KeyCode::Down | KeyCode::Char('k') | KeyCode::Char('K') => {
                            app.speed = app.speed.saturating_add(Duration::from_millis(10));
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
}

fn is_event_available(speed: Duration) -> io::Result<bool> {
    event::poll(speed)
}
