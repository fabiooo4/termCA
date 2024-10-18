use std::time::Duration;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::style::Color;
use tui_input::{backend::crossterm::EventHandler, InputRequest};

use crate::{
    app::{App, InputMode, Screen},
    simulations::ant::{Ant, AntSim},
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
            AntSim::run_ant_sim(app);
        }
        KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
            // Increase simulation speed
            if app.speed > Duration::from_millis(100) {
                app.speed = app.speed.saturating_sub(Duration::from_millis(100));
            } else if app.speed > Duration::from_millis(10) {
                app.speed = app.speed.saturating_sub(Duration::from_millis(10));
            } else if app.speed > Duration::from_millis(0) {
                app.speed = app.speed.saturating_sub(Duration::from_millis(1));
            } else {
                if app.speed_multiplier < 10 {
                    app.speed_multiplier = app.speed_multiplier.saturating_add(1);
                } else if app.speed_multiplier < 100 {
                    app.speed_multiplier = app.speed_multiplier.saturating_add(10);
                } else if app.speed_multiplier < 1000 {
                    app.speed_multiplier = app.speed_multiplier.saturating_add(100);
                } else {
                    app.speed_multiplier = app.speed_multiplier.saturating_add(1000);
                }
            }
        }
        KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
            // Decrease simulation speed
            if app.speed_multiplier > 1 {
                if app.speed_multiplier > 1000 {
                    app.speed_multiplier = app.speed_multiplier.saturating_sub(1000);
                } else if app.speed_multiplier > 100 {
                    app.speed_multiplier = app.speed_multiplier.saturating_sub(100);
                } else if app.speed_multiplier > 10 {
                    app.speed_multiplier = app.speed_multiplier.saturating_sub(10);
                } else {
                    app.speed_multiplier = app.speed_multiplier.saturating_sub(1);
                }
            } else if app.speed < Duration::from_millis(10) {
                app.speed = app.speed.saturating_add(Duration::from_millis(1));
            } else if app.speed < Duration::from_millis(100) {
                app.speed = app.speed.saturating_add(Duration::from_millis(10));
            } else {
                app.speed = app.speed.saturating_add(Duration::from_millis(100));
            }
        }
        _ => {}
    }
}

pub fn edit(key: KeyEvent, app: &mut App) {
    let ant_sim = app.ant_sim.as_mut().unwrap();
    let scroll_factor =
        if ant_sim.edit_item_selected > 0 && ant_sim.edit_item_selected <= ant_sim.ants.len() + 2 {
            5
        } else if ant_sim.edit_item_selected == 0 {
            0
        } else {
            0
        };
    match ant_sim.rules_input_mode {
        InputMode::Normal => {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                    app.editing = None;
                    app.ant_sim = None;
                }

                KeyCode::Char('?') => {
                    app.help_screen = !app.help_screen;
                }

                KeyCode::Backspace => {
                    if ant_sim.edit_item_selected > 0
                        && ant_sim.edit_item_selected < ant_sim.ants.len() + 1
                    {
                        ant_sim
                            .ants
                            .remove(ant_sim.edit_item_selected.saturating_sub(1));
                    }
                }

                KeyCode::Enter => {
                    if ant_sim.edit_item_selected == 0 {
                        // Ruleset Input

                        ant_sim.rules_input_mode = InputMode::Editing;
                    } else if ant_sim.edit_item_selected == ant_sim.ants.len() + 1 {
                        // Add ant button

                        ant_sim.ants.push(Ant::default());

                        // Scroll down
                        for _ in 0..5 {
                            ant_sim.scroll_state.scroll_down();
                        }

                        if ant_sim.edit_item_selected < 1 + ant_sim.ants.len() {
                            ant_sim.edit_item_selected =
                                ant_sim.edit_item_selected.saturating_add(1);
                        }
                    } else if ant_sim.edit_item_selected == ant_sim.ants.len() + 2 {
                        // Confirm button

                        // Parse the user inserted rules
                        ant_sim.rules = AntSim::parse_ant_ruleset(&ant_sim.rules_input.value());

                        // Add states for every rule
                        let rules_len = ant_sim.rules.len();
                        let states_len = ant_sim.states.len();
                        if rules_len > states_len {
                            for i in (states_len + 1)..=rules_len {
                                ant_sim.states.push(Color::Indexed(i as u8));
                            }
                        }

                        // Change the screen
                        app.editing = None;
                        app.current_screen = Screen::Ant;
                    }
                }

                KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
                    for _ in 0..scroll_factor {
                        ant_sim.scroll_state.scroll_down();
                    }
                    if ant_sim.edit_item_selected < 2 + ant_sim.ants.len() {
                        ant_sim.edit_item_selected = ant_sim.edit_item_selected.saturating_add(1);
                    }
                }

                KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                        for _ in 0..scroll_factor {
                            ant_sim.scroll_state.scroll_up();
                        }
                    ant_sim.edit_item_selected = ant_sim.edit_item_selected.saturating_sub(1);
                }

                _ => {}
            }
        }
        InputMode::Editing => match key.code {
            KeyCode::Esc => {
                app.ant_sim.as_mut().unwrap().rules_input_mode = InputMode::Normal;
            }
            KeyCode::Enter => {
                app.ant_sim.as_mut().unwrap().rules_input_mode = InputMode::Normal;
            }
            _ => {
                let ant_sim = app.ant_sim.as_mut().unwrap();
                let allowed_chars = "rlfbRLFB";

                // Only handle allowed characters
                ant_sim
                    .rules_input
                    .handle_event(&Event::Key(match key.code {
                        KeyCode::Char(c) => {
                            if allowed_chars.contains(c) {
                                KeyEvent::from(KeyCode::Char(c.to_ascii_uppercase()))
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

pub fn resize(new_width: u16, new_height: u16, app: &mut App) {
    let new_width: usize = new_width as usize - 2;
    let new_height: usize = (new_height as usize - 2) * 2;

    // Reposition the ant in the view if it is out of bounds
    for ant in app.ant_sim.as_mut().unwrap().ants.iter_mut() {
        if ant.x >= new_width {
            ant.x = new_width - 1;
        }

        if ant.y >= new_height {
            ant.y = new_height - 1;
        }
    }

    // Resize the grid
    app.ant_sim
        .as_mut()
        .unwrap()
        .grid
        .resize(new_width, new_height, Color::Reset);
}
