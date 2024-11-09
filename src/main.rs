mod app;
mod events;
mod simulations;
mod ui;

use crate::app::App;
use crate::ui::ui;
use app::Screen;
use crossterm::event::{self, Event};
use events::{ant_events, elementary_events, is_event_available, main_events};
use ratatui::DefaultTerminal;
use ui::elementary_ui::elementary_screen;
use std::io::{self};

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
                    app.ant_sim.as_mut().unwrap().run(app.speed_multiplier);
                }
            }
            Screen::Elementary => {
                if app.is_running && !app.help_screen {
                    // Run Elementary CA
                    app.elementary_sim
                        .as_mut()
                        .unwrap()
                        .run(app.speed_multiplier);
                }
            }
            _ => {}
        }

        // Only run when an event is available
        if !is_event_available(app.speed)? {
            continue;
        }

        match event::read()? {
            Event::Resize(new_width, new_height) => {
                match app.current_screen {
                    Screen::Ant => ant_events::resize(new_width, new_height, app),
                    Screen::Elementary => elementary_events::resize(new_width, new_height, app),
                    _ => (),
                }
            }

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
                        Screen::AntEdit(ant_idx) => ant_events::edit_ant(key, app, ant_idx),
                        Screen::Elementary => elementary_events::edit(key, app),
                        _ => {}
                    }
                } else {
                    match app.current_screen {
                        Screen::Main => main_events::main(key, app),
                        Screen::Ant => ant_events::main(key, app),
                        Screen::Elementary => elementary_events::main(key, app),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
