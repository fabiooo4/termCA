
use crossterm::event::{KeyCode, KeyEvent};

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
        KeyCode::Char('?') => {
            app.help_screen = !app.help_screen;
        }
        KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('L') => {
            // Run simulation once
            app.elementary_sim.as_mut().unwrap().run(app.speed_multiplier);
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
