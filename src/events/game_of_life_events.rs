use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::Color;

use crate::{
    app::{App, EditTab, Screen},
    simulations::{game_of_life::GolSettings, Direction},
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
            app.gol_sim.as_mut().unwrap().run(app.speed_multiplier);
        }
        KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => app.speed_increase(),
        KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => app.speed_decrease(),
        _ => {}
    }
}

pub fn edit(key: KeyEvent, app: &mut App) {
    let sim = app.gol_sim.as_mut().unwrap();
    if app.selected_edit_tab.as_ref().unwrap() == &EditTab::Setting {
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                app.editing = None;
                app.selected_edit_tab = None;
                app.ant_sim = None;
            }

            KeyCode::Char(' ') => {
                // Change the screen
                app.editing = None;
                app.selected_edit_tab = None;
                app.current_screen = Screen::GameOfLife;
            }

            KeyCode::Char('?') => {
                app.help_screen = !app.help_screen;
            }

            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => sim.settings_state.next(),

            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => sim.settings_state.previous(),

            KeyCode::Enter | KeyCode::Char('l') | KeyCode::Char('L') | KeyCode::Right => {
                match GolSettings::from_index(sim.settings_state.selected.unwrap_or(0)) {
                    GolSettings::EditGrid => {
                        app.editing = Some(Screen::GolEdit);
                    }

                    GolSettings::Start => {
                        // Change the screen
                        app.editing = None;
                        app.selected_edit_tab = None;
                        app.current_screen = Screen::GameOfLife;
                    }
                }
            }

            _ => {}
        }
    }
}

pub fn edit_gol(key: KeyEvent, app: &mut App) {
    let sim = app.gol_sim.as_mut().unwrap();
    let speed_toggle = 2;

    match key.code {
        KeyCode::Char('?') => app.help_screen = !app.help_screen,
        KeyCode::Enter => {
            app.current_screen = Screen::GameOfLife;
            app.editing = None;
            app.is_running = !app.is_running;
        }

        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.editing = Some(Screen::GameOfLife)
        }

        KeyCode::Up | KeyCode::Char('k') => {
            sim.edit_cursor.change_position(Direction::Up, &sim.grid)
        }
        KeyCode::Char('K') => {
            for _ in 0..speed_toggle {
                sim.edit_cursor.change_position(Direction::Up, &sim.grid)
            }
        }

        KeyCode::Down | KeyCode::Char('j') => {
            sim.edit_cursor.change_position(Direction::Down, &sim.grid)
        }
        KeyCode::Char('J') => {
            for _ in 0..speed_toggle {
                sim.edit_cursor.change_position(Direction::Down, &sim.grid)
            }
        }

        KeyCode::Right | KeyCode::Char('l') => {
            sim.edit_cursor.change_position(Direction::Right, &sim.grid)
        }
        KeyCode::Char('L') => {
            for _ in 0..speed_toggle {
                sim.edit_cursor.change_position(Direction::Right, &sim.grid)
            }
        }

        KeyCode::Left | KeyCode::Char('h') => {
            sim.edit_cursor.change_position(Direction::Left, &sim.grid)
        }
        KeyCode::Char('H') => {
            for _ in 0..speed_toggle {
                sim.edit_cursor.change_position(Direction::Left, &sim.grid)
            }
        }

        KeyCode::Char(' ') => {
            sim.toggle_cell(sim.edit_cursor.x, sim.edit_cursor.y);
        }
        _ => {}
    }
}

pub fn resize(new_width: u16, new_height: u16, app: &mut App) {
    let new_width: usize = new_width as usize - 2;
    let new_height: usize = (new_height as usize - 2) * 2;

    // Resize the grid
    app.gol_sim
        .as_mut()
        .unwrap()
        .grid
        .resize(new_width, new_height, Color::Reset);
}
