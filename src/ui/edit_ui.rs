use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout}, style::{Style, Stylize}, text::Line, widgets::{Block, BorderType, Borders, Clear, Paragraph}, Frame
};

use crate::app::{App, Screen};

use super::centered_rect_length;

pub fn render_edit(frame: &mut Frame, edit_sim: Screen, app: &mut App) {
    match edit_sim {
        Screen::Ant => {
            let keys = vec![
                Line::from("Q ".yellow()),
                Line::from("? ".yellow()),
                Line::from("Enter ".yellow()),
                Line::from("K / ↑ ".yellow()),
                Line::from("J / ↓ ".yellow()),
                Line::from("L / → ".yellow()),
                Line::from("H / ← ".yellow()),
                Line::from("g ".yellow()),
                Line::from("G ".yellow()),
            ];

            let labels = vec![
                Line::from("Quit"),
                Line::from("Help"),
                Line::from("Select"),
                Line::from("Scroll Up"),
                Line::from("Scroll Down"),
                Line::from("Scroll Right"),
                Line::from("Scroll Left"),
                Line::from("Scroll to Bottom"),
                Line::from("Scroll to Top"),
            ];

            let help_area = centered_rect_length(27, (keys.len() + 4) as u16, frame.area());
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
        _ => {}
    }
}
