mod app;
mod simulations;
mod ui;

use crate::app::App;
use crate::ui::ui;
use app::CurrentScreen;
use crossterm::event::{self, Event, KeyCode};
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

        // Only run when an event is available
        if !is_event_available(app.speed)? {
            continue;
        }

        match event::read()? {
            Event::Resize(new_width, new_height) => match app.current_screen {
                CurrentScreen::Ant => {
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

                match app.current_screen {
                    CurrentScreen::Main => match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => {
                            app.current_screen = CurrentScreen::Exit
                        }
                        KeyCode::Char('?') => {
                            app.help_screen = !app.help_screen;
                        }
                        KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Left => {
                            if !app.settings_list_state.selected().is_none() {
                                app.sim_select_idx(app.settings_list_state.selected());
                                app.settings_select_none();
                            }
                        }
                        KeyCode::Char('l') | KeyCode::Char('L') | KeyCode::Right => {
                            if !app.sim_list_state.selected().is_none() {
                                app.settings_select_idx(app.sim_list_state.selected());
                                app.sim_select_none();
                            }
                        }
                        KeyCode::Char('j') | KeyCode::Char('J') | KeyCode::Down => {
                            if app.sim_list_state.selected().is_none()
                                && app.settings_list_state.selected().is_none()
                            {
                                app.sim_select_first();
                            }

                            if app.sim_list_state.selected().is_none() {
                                if app.settings_list_state.selected()
                                    == Some(app.simulation_items.len() - 1)
                                {
                                    app.settings_select_next();
                                }
                            } else {
                                app.sim_select_next();
                            }
                        }
                        KeyCode::Char('k') | KeyCode::Char('K') | KeyCode::Up => {
                            if app.sim_list_state.selected().is_none()
                                && app.settings_list_state.selected().is_none()
                            {
                                app.sim_select_first();
                            }

                            if app.sim_list_state.selected().is_none() {
                                app.settings_select_previous();
                            } else {
                                app.sim_select_previous();
                            }
                        }
                        KeyCode::Char('g') | KeyCode::Home => {
                            if app.sim_list_state.selected().is_none()
                                && app.settings_list_state.selected().is_none()
                            {
                                app.sim_select_first();
                            }

                            if app.sim_list_state.selected().is_none() {
                                app.settings_select_first();
                            } else {
                                app.sim_select_first();
                            }
                        }
                        KeyCode::Char('G') | KeyCode::End => {
                            if app.sim_list_state.selected().is_none()
                                && app.settings_list_state.selected().is_none()
                            {
                                app.sim_select_last();
                            }

                            if app.sim_list_state.selected().is_none() {
                                if app.settings_list_state.selected()
                                    == Some(app.simulation_items.len() - 1)
                                {
                                    app.settings_select_last();
                                }
                            } else {
                                app.sim_select_last();
                            }
                        }
                        KeyCode::Enter => {
                            if !app.sim_list_state.selected().is_none() {
                                app.change_screen();
                            }

                            if let Some(i) = app.settings_list_state.selected() {
                                match app.simulation_items[i].screen {
                                    CurrentScreen::Ant => {
                                        let mut ant_sim_options = AntSim::default();
                                        ant_sim_options.ants =
                                            vec![Ant::new(10000, 10000, Direction::Up)];

                                        ant_sim_options.rules =
                                            AntSim::parse_ant_ruleset("RRLLLRLLLRRR");

                                        app.start_ant(ant_sim_options);
                                        app.change_screen();
                                    }
                                    _ => {
                                        app.change_screen();
                                    }
                                }
                            }
                        }
                        _ => {}
                    },
                    CurrentScreen::Ant => match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => {
                            app.current_screen = CurrentScreen::Main;
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
                        KeyCode::Up | KeyCode::Char('j') | KeyCode::Char('J') => {
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
                                    app.speed_multiplier =
                                        app.speed_multiplier.saturating_add(1000);
                                }
                            }
                        }
                        KeyCode::Down | KeyCode::Char('k') | KeyCode::Char('K') => {
                            // Decrease simulation speed
                            if app.speed_multiplier > 1 {
                                if app.speed_multiplier > 1000 {
                                    app.speed_multiplier =
                                        app.speed_multiplier.saturating_sub(1000);
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
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

fn is_event_available(speed: Duration) -> io::Result<bool> {
    event::poll(speed)
}
