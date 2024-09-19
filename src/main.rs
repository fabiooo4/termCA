mod app;
mod ui;
use app::CurrentScreen;
use crossterm::event::{self, Event, KeyCode};
use ratatui::DefaultTerminal;
use ratatui::{prelude::Backend, Terminal};
use std::io;
use std::time::Duration;

use crate::app::App;
use crate::ui::ui;

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
        if is_event_available()? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    // Skip events that are not KeyEventKind::Press
                    continue;
                }

                match app.current_screen {
                    CurrentScreen::Ant => match key.code {
                        KeyCode::Char('q') => app.current_screen = CurrentScreen::Exit,
                        KeyCode::Right=> {
                            // Move one terminal cell to the right
                            app.ant.x += 1.;
                        }
                        KeyCode::Up => {
                            // Move half terminal cell up
                            app.ant.y += 1.;
                        }
                        KeyCode::Left => {
                            // Move one terminal cell to the left
                            app.ant.x -= 1.;
                        }
                        KeyCode::Down => {
                            // Move half terminal cell down
                            app.ant.y -= 1.;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        match app.current_screen {
            CurrentScreen::Exit => break Ok(()),
            _ => {}
        }
    }
}

fn is_event_available() -> io::Result<bool> {
    event::poll(Duration::from_millis(100))
}
