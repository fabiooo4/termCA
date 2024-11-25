use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::style::Color;
use tui_input::backend::crossterm::EventHandler;

use crate::{
    app::{App, EditTab, InputMode, Screen},
    simulations::{
        ant::{Ant, AntSettings, AntSim},
        Direction,
    },
};

pub fn main(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
            app.current_screen = Screen::Main;
            app.stop_all();
        }
        KeyCode::Char(' ') => app.is_running = !app.is_running,
        KeyCode::Char('?') => app.help_screen = !app.help_screen,
        // Run the simulation one step at a time
        KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('L') => {
            app.ant_sim.as_mut().unwrap().run(app.speed_multiplier)
        }
        KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => app.speed_increase(),
        KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => app.speed_decrease(),
        _ => {}
    }
}

pub fn edit(key: KeyEvent, app: &mut App) {
    let sim = app.ant_sim.as_mut().unwrap();
    match app.selected_edit_tab.as_ref().unwrap() {
        EditTab::Setting => match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                app.editing = None;
                app.selected_edit_tab = None;
                app.ant_sim = None;
            }

            KeyCode::Char(' ') => {
                // Change the screen
                app.editing = None;
                app.selected_edit_tab = None;
                app.current_screen = Screen::Ant;
            }

            KeyCode::Char('?') => {
                app.help_screen = !app.help_screen;
            }

            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => sim.settings_state.next(),

            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => sim.settings_state.previous(),

            KeyCode::Enter | KeyCode::Char('l') | KeyCode::Char('L') | KeyCode::Right => {
                match AntSettings::from_index(sim.settings_state.selected.unwrap_or(0)) {
                    AntSettings::Ruleset => {
                        sim.rules_input_mode = InputMode::Editing;
                        app.selected_edit_tab.as_mut().unwrap().next();
                    }

                    AntSettings::Ants => {
                        app.selected_edit_tab.as_mut().unwrap().next();
                    }

                    AntSettings::Start => {
                        // Change the screen
                        app.editing = None;
                        app.selected_edit_tab = None;
                        app.current_screen = Screen::Ant;
                    }
                }
            }

            _ => {}
        },

        EditTab::Content => {
            match AntSettings::from_index(sim.settings_state.selected.unwrap_or(0)) {
                AntSettings::Ruleset => {
                    match key.code {
                        KeyCode::Char('?') => {
                            app.help_screen = !app.help_screen;
                        }

                        KeyCode::Esc
                        | KeyCode::Enter
                        | KeyCode::Char('q')
                        | KeyCode::Char('h')
                        | KeyCode::Char('H') => {
                            let sim = app.ant_sim.as_mut().unwrap();

                            sim.rules_input_mode = InputMode::Normal;
                            app.selected_edit_tab.as_mut().unwrap().next();

                            // Parse the user inserted rules
                            sim.rules = AntSim::parse_ant_ruleset(sim.rules_input.value());

                            // Add states for every rule
                            let rules_len = sim.rules.len();
                            let states_len = sim.states.len();
                            if rules_len > states_len {
                                for i in (states_len + 1)..=rules_len {
                                    sim.states.push(Color::Indexed(i as u8));
                                }
                            }
                        }
                        _ => {
                            let sim = app.ant_sim.as_mut().unwrap();
                            let allowed_chars = "rlfbRLFB";

                            // Only handle allowed characters
                            sim.rules_input.handle_event(&Event::Key(match key.code {
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
                    }
                }

                AntSettings::Ants => match key.code {
                    KeyCode::Char('?') => {
                        app.help_screen = !app.help_screen;
                    }

                    KeyCode::Esc
                    | KeyCode::Char('q')
                    | KeyCode::Char('h')
                    | KeyCode::Char('H')
                    | KeyCode::Left => {
                        app.selected_edit_tab.as_mut().unwrap().next();
                    }

                    KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
                        sim.ants_list_state.next()
                    }

                    KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                        sim.ants_list_state.previous()
                    }

                    KeyCode::Backspace => {
                        if sim
                            .ants_list_state
                            .selected
                            .is_some_and(|selected| selected > 0)
                        {
                            let selected = sim.ants_list_state.selected.unwrap();

                            if selected == sim.ants.len() {
                                sim.ants_list_state.previous();
                            }

                            sim.ants.remove(selected.saturating_sub(1));
                        }
                    }

                    KeyCode::Enter
                        if sim
                            .ants_list_state
                            .selected
                            .is_some_and(|selected| selected == 0) =>
                    {
                        sim.ants.push(Ant::default());
                    }

                    KeyCode::Enter => {
                        app.editing = Some(Screen::AntEdit(
                            sim.ants_list_state.selected.unwrap().saturating_sub(1),
                        ));
                    }
                    _ => {}
                },
                _ => (),
            }
        }
    }
}

pub fn edit_ant(key: KeyEvent, app: &mut App, ant_idx: usize) {
    let ant_sim = app.ant_sim.as_mut().unwrap();
    let speed_toggle = 2;

    match key.code {
        KeyCode::Char('?') => app.help_screen = !app.help_screen,

        KeyCode::Enter | KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.editing = Some(Screen::Ant)
        }

        KeyCode::Up | KeyCode::Char('k') => {
            ant_sim.ants[ant_idx].change_position(Direction::Up, &ant_sim.grid)
        }
        KeyCode::Char('K') => {
            for _ in 0..speed_toggle {
                ant_sim.ants[ant_idx].change_position(Direction::Up, &ant_sim.grid)
            }
        }

        KeyCode::Down | KeyCode::Char('j') => {
            ant_sim.ants[ant_idx].change_position(Direction::Down, &ant_sim.grid)
        }
        KeyCode::Char('J') => {
            for _ in 0..speed_toggle {
                ant_sim.ants[ant_idx].change_position(Direction::Down, &ant_sim.grid)
            }
        }

        KeyCode::Right | KeyCode::Char('l') => {
            ant_sim.ants[ant_idx].change_position(Direction::Right, &ant_sim.grid)
        }
        KeyCode::Char('L') => {
            for _ in 0..speed_toggle {
                ant_sim.ants[ant_idx].change_position(Direction::Right, &ant_sim.grid)
            }
        }

        KeyCode::Left | KeyCode::Char('h') => {
            ant_sim.ants[ant_idx].change_position(Direction::Left, &ant_sim.grid)
        }
        KeyCode::Char('H') => {
            for _ in 0..speed_toggle {
                ant_sim.ants[ant_idx].change_position(Direction::Left, &ant_sim.grid)
            }
        }

        KeyCode::Char('r') => {
            ant_sim.ants[ant_idx].direction = ant_sim.ants[ant_idx].direction.turn_right()
        }

        KeyCode::Char('R') => {
            ant_sim.ants[ant_idx].direction = ant_sim.ants[ant_idx].direction.turn_left()
        }
        _ => {}
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
