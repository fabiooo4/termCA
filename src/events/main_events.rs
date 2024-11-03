use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, Screen};

pub fn main(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => app.current_screen = Screen::Exit,
        KeyCode::Char('?') => {
            app.help_screen = !app.help_screen;
        }
        KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Left => {
            app.select_left();
        }
        KeyCode::Char('l') | KeyCode::Char('L') | KeyCode::Right => {
            app.select_right();
        }
        KeyCode::Char('j') | KeyCode::Char('J') | KeyCode::Down => {
            if app.list_state.selected().is_none() {
                app.select_first();
            }

            app.select_next();
        }
        KeyCode::Char('k') | KeyCode::Char('K') | KeyCode::Up => {
            if app.list_state.selected().is_none() {
                app.select_first();
            }

            app.select_previous();
        }
        KeyCode::Char('g') | KeyCode::Home => {
            app.select_first();
        }
        KeyCode::Char('G') | KeyCode::End => app.select_last(),
        KeyCode::Enter => {
            // If a simulation is selected from the list,
            // change the screen to that simulation
            if let Some(0) = app.list_state.selected_column() {
                app.change_screen_selected();
            } else {
                // If edit is selected, enter edit mode on the selected simulation
                if let Some(i) = app.list_state.selected() {
                    match app.list_items[i].screen {
                        Screen::Ant => {
                            // Create a default ant simulation to be able to edit it
                            app.start_ant_default();
                            app.editing = Some(app.list_items[i].screen);
                        }
                        _ => {}
                    }
                }
            }
        }
        _ => {}
    }
}
