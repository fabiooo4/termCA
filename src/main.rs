mod app;
mod simulations;
mod ui;
mod events;

use crate::app::App;
use crate::ui::ui;
use app::Screen;
use crossterm::event::{self, Event, KeyCode};
use events::{ant_events, main_events};
use ratatui::style::Color;
use ratatui::DefaultTerminal;
use simulations::ant::{Ant, AntSim, Direction};
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
        // Always running
        match app.current_screen {
            Screen::Exit => break Ok(()),
            Screen::Ant => {
                if app.is_running && !app.help_screen {
                    // Run Langton's Ant
                    AntSim::run_ant_sim(app);
                }
            }
            _ => {}
        }

        // Only run when an event is available
        if !is_event_available(app.speed)? {
            continue;
        }

        match event::read()? {
            Event::Resize(new_width, new_height) => match app.current_screen {
                Screen::Ant => ant_events::resize(new_width, new_height, app),
                _ => {}
            },

            Event::Key(key) => {
                if key.kind != event::KeyEventKind::Press {
                    // Skip events that are not KeyEventKind::Press
                    continue;
                }

                if app.help_screen {
                    // Prevent any action when the help screen is displayed
                    app.help_screen = false;
                    continue;
                }

                if let Some(edit_sim) = app.editing {
                    match edit_sim {
                        Screen::Ant => ant_events::edit(key, app),
                        _ => {}
                    }
                } else {
                    match app.current_screen {
                        Screen::Main => main_events::main(key, app),
                        Screen::Ant => ant_events::main(key, app),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

fn is_event_available(speed: Duration) -> io::Result<bool> {
    event::poll(speed)
}
