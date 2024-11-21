use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::style::Color;
use tui_input::backend::crossterm::EventHandler;

use crate::{
    app::{App, EditTab, InputMode, Screen},
    simulations::elementary::ElementarySettings,
};

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
    match app.selected_edit_tab.as_ref().unwrap() {
        EditTab::Setting => match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                app.editing = None;
                app.selected_edit_tab = None;
                app.elementary_sim = None;
            }

            KeyCode::Char(' ') => {
                // Change the screen
                app.editing = None;
                app.selected_edit_tab = None;
                app.current_screen = Screen::Elementary;
            }

            KeyCode::Char('?') => {
                app.help_screen = !app.help_screen;
            }

            KeyCode::Char('j') => sim.settings_state.next(),

            KeyCode::Char('k') => sim.settings_state.previous(),

            KeyCode::Enter => {
                match ElementarySettings::from_index(sim.settings_state.selected.unwrap_or(0)) {
                    ElementarySettings::Rule => {
                        sim.rule_input_mode = InputMode::Editing;
                        app.selected_edit_tab.as_mut().unwrap().next();
                    }
                    _ => {}
                }
            }

            _ => {}
        },

        EditTab::Content => {
            match ElementarySettings::from_index(sim.settings_state.selected.unwrap_or(0)) {
                ElementarySettings::Rule => {
                    match key.code {
                        KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => {
                            app.elementary_sim.as_mut().unwrap().rule_input_mode =
                                InputMode::Normal;
                            app.selected_edit_tab.as_mut().unwrap().next();
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

                            sim.parse_input();
                        }
                    }
                }
            }
        }
    }

    /* match sim.rule_input_mode {
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
                    app.selected_edit_tab.as_mut().unwrap().next();
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
                app.selected_edit_tab.as_mut().unwrap().next();
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

                sim.parse_input();
            }
        },
    } */
}

pub fn resize(new_width: u16, new_height: u16, app: &mut App) {
    let new_width: usize = new_width as usize;
    let new_height: usize = (new_height as usize - 2) * 2;

    let sim = app.elementary_sim.as_mut().unwrap();

    // Resize the grid
    sim.grid.resize(new_width, new_height, Color::Reset);

    // Resize the line
    sim.current_line.resize(new_width, false);
}
