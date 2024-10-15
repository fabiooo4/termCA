use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::{App, Screen};

use super::centered_rect_length;

pub fn render_edit(frame: &mut Frame, edit_sim: Screen, app: &mut App) {
    match edit_sim {
        Screen::Ant => {
            // let ant_sim = app.ant_sim.as_mut().unwrap();

            let edit_area = centered_rect_length(27, 4, frame.area());
            let edit_block = Block::default()
                .title(" Edit ".yellow().bold())
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default());

            frame.render_widget(Clear, edit_area);
            frame.render_widget(edit_block, edit_area);

            let ruleset_block = Block::default().title("Ruleset").borders(Borders::ALL);
            let ruleset = app.ant_sim.as_ref().unwrap().rules_input.clone();

            let ruleset_paragraph = Paragraph::new(ruleset).block(ruleset_block);
            frame.render_widget(ruleset_paragraph, edit_area);
        }
        _ => {}
    }
}
