use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use super::centered_rect_length;

pub fn render_help(frame: &mut Frame, entries: Vec<(Line, Line)>) {
    let (keys, labels): (Vec<Line>, Vec<Line>) = entries.into_iter().unzip();

    let longest_key = keys.iter()
        .map(|k| k.to_string().len())
        .max()
        .unwrap_or(1);

    let longest_label = labels.iter()
        .map(|l| l.to_string().len())
        .max()
        .unwrap_or(1);

    let help_area = centered_rect_length((longest_key + longest_label + 3) as u16, (keys.len() + 4) as u16, frame.area());
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
