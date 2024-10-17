use crossterm::event::{KeyCode, KeyEvent};

use crate::{app::{App, Screen}, simulations::ant::AntSim};

pub fn main(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => app.current_screen = Screen::Exit,
        KeyCode::Char('?') => {
            app.help_screen = !app.help_screen;
        }
        KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Left => {
            if !app.edit_list_state.selected().is_none() {
                app.sim_select_idx(app.edit_list_state.selected());
                app.edit_select_none();
            }
        }
        KeyCode::Char('l') | KeyCode::Char('L') | KeyCode::Right => {
            if !app.sim_list_state.selected().is_none() {
                app.edit_select_idx(app.sim_list_state.selected());
                app.sim_select_none();
            }
        }
        KeyCode::Char('j') | KeyCode::Char('J') | KeyCode::Down => {
            if app.sim_list_state.selected().is_none()
                && app.edit_list_state.selected().is_none()
            {
                app.sim_select_first();
            }

            if app.sim_list_state.selected().is_none() {
                if app.edit_list_state.selected() == Some(app.simulation_items.len() - 1) {
                    app.edit_select_next();
                }
            } else {
                app.sim_select_next();
            }
        }
        KeyCode::Char('k') | KeyCode::Char('K') | KeyCode::Up => {
            if app.sim_list_state.selected().is_none()
                && app.edit_list_state.selected().is_none()
            {
                app.sim_select_first();
            }

            if app.sim_list_state.selected().is_none() {
                app.edit_select_previous();
            } else {
                app.sim_select_previous();
            }
        }
        KeyCode::Char('g') | KeyCode::Home => {
            if app.sim_list_state.selected().is_none()
                && app.edit_list_state.selected().is_none()
            {
                app.sim_select_first();
            }

            if app.sim_list_state.selected().is_none() {
                app.edit_select_first();
            } else {
                app.sim_select_first();
            }
        }
        KeyCode::Char('G') | KeyCode::End => {
            if app.sim_list_state.selected().is_none()
                && app.edit_list_state.selected().is_none()
            {
                app.sim_select_last();
            }

            if app.sim_list_state.selected().is_none() {
                if app.edit_list_state.selected() == Some(app.simulation_items.len() - 1) {
                    app.edit_select_last();
                }
            } else {
                app.sim_select_last();
            }
        }
        KeyCode::Enter => {
            // If a simulation is selected from the list,
            // change the screen to that simulation
            if !app.sim_list_state.selected().is_none() {
                app.change_screen_selected();
            }

            // If edit is selected, enter edit mode on the selected simulation
            if let Some(i) = app.edit_list_state.selected() {
                match app.simulation_items[i].screen {
                    Screen::Ant => {
                        // Create a default ant simulation to be able to edit it
                        app.start_ant_default();
                        app.editing = Some(app.simulation_items[i].screen);
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}
