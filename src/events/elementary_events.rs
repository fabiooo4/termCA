use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::style::Color;
use tui_input::backend::crossterm::EventHandler;

use crate::app::{App, InputMode, Screen};

pub fn main(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
            app.current_screen = Screen::Main;
            app.stop_all();
        }
        KeyCode::Char(' ') => app.is_running = !app.is_running,
        KeyCode::Char('?') => {
            app.help_screen = !app.help_screen;
        }
        KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('L') => {
            // Run simulation once
            app.elementary_sim
                .as_mut()
                .unwrap()
                .run(app.speed_multiplier);
        }
        KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
            // Increase simulation speed
            app.speed_increase();
        }
        KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
            // Decrease simulation speed
            app.speed_decrease();
        }
        _ => {}
    }
}

pub fn edit(key: KeyEvent, app: &mut App) {
    let sim = app.elementary_sim.as_mut().unwrap();
    match sim.rule_input_mode {
        InputMode::Normal => {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                    app.editing = None;
                    app.elementary_sim = None;
                }

                KeyCode::Char('?') => {
                    app.help_screen = !app.help_screen;
                }

                KeyCode::Enter => {
                    sim.rule_input_mode = InputMode::Editing;
                }

                KeyCode::Char(' ') => {
                    // Change the screen
                    app.editing = None;
                    app.current_screen = Screen::Elementary;
                }

                _ => {}
            }
        }
        InputMode::Editing => match key.code {
            KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => {
                app.elementary_sim.as_mut().unwrap().rule_input_mode = InputMode::Normal;
            }
            _ => {
                let sim = app.elementary_sim.as_mut().unwrap();
                let allowed_chars = "0123456789";

                // Only handle allowed characters
                sim.rule_input.handle_event(&Event::Key(match key.code {
                    KeyCode::Char(c) => {
                        if allowed_chars.contains(c) {
                            KeyEvent::from(KeyCode::Char(c))
                        } else {
                            KeyEvent::from(KeyCode::Null)
                        }
                    }
                    _ => key,
                }));
            }
        },
    }
}
