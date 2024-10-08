pub mod ant_ui;
pub mod main_ui;
pub mod edit_ui;
pub mod help_ui;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::app::{App, Screen};

pub fn ui(frame: &mut Frame, app: &mut App) {
    // Render widgets
    match app.current_screen {
        Screen::Main => main_ui::main_screen(frame, app),
        Screen::Ant => ant_ui::ant_screen(frame, app),

        _ => {}
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect_percent(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

pub fn centered_rect_length(cols: u16, rows: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            {
                if r.height > rows {
                    Constraint::Length((r.height - rows) / 2)
                } else {
                    Constraint::Min(1)
                }
            },
            Constraint::Length(rows),
            {
                if r.height > rows {
                    Constraint::Length((r.height - rows) / 2)
                } else {
                    Constraint::Min(1)
                }
            },
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((r.width - cols) / 2),
            Constraint::Length(cols),
            Constraint::Length((r.width - cols) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
