pub mod ant_ui;
pub mod main_ui;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
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

pub fn render_help(frame: &mut Frame, entries: Vec<(Line, Line)>) {
    let (keys, labels): (Vec<Line>, Vec<Line>) = entries.into_iter().unzip();

    let longest_key = keys.iter().map(|k| k.to_string().len()).max().unwrap_or(1);

    let longest_label = labels
        .iter()
        .map(|l| l.to_string().len())
        .max()
        .unwrap_or(1);

    let help_area = centered_rect_length(
        (longest_key + longest_label + 3) as u16,
        (keys.len() + 4) as u16,
        frame.area(),
    );
    let help_block = Block::default()
        .title(" Help ".yellow().bold())
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default());

    let help_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(keys.len() as u16),
            Constraint::Length(2),
        ])
        .split(help_area);

    let help_center = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(help_layout[1]);

    let help_keys = Paragraph::new(keys).alignment(Alignment::Right);
    let help_labels = Paragraph::new(labels).alignment(Alignment::Left);

    frame.render_widget(Clear, help_area);
    frame.render_widget(help_block, help_area);
    frame.render_widget(help_keys, help_center[0]);
    frame.render_widget(help_labels, help_center[1]);
}
