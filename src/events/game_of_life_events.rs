use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::Color;

use crate::
    app::{App, Screen}
;

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
            todo!()
        }
        KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => app.speed_increase(),
        KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => app.speed_decrease(),
        _ => {}
    }
}

pub fn edit(key: KeyEvent, app: &mut App) {
    todo!()
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
