use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, Screen};

pub fn main(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => app.current_screen = Screen::Exit,
        KeyCode::Char('?') => app.help_screen = !app.help_screen,
        KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Left => app.select_left(),
        KeyCode::Char('l') | KeyCode::Char('L') | KeyCode::Right => app.select_right(),
        KeyCode::Char('j') | KeyCode::Char('J') | KeyCode::Down => app.select_next(),
        KeyCode::Char('k') | KeyCode::Char('K') | KeyCode::Up => app.select_previous(),
        KeyCode::Char('g') | KeyCode::Home => app.select_first(),
        KeyCode::Char('G') | KeyCode::End => app.select_last(),
        KeyCode::Enter => app.apply_selected(),
        _ => {}
    }
}
