use rand::{prelude::Distribution, Rng};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{
        block::{Position, Title},
        canvas::{Canvas, Points},
        Block, BorderType, Borders, Clear, Paragraph,
    },
    Frame,
};

use crate::{app::{App, CurrentScreen}, simulations::ant::AntSim};

pub fn ui(frame: &mut Frame, app: &mut App) {
    // Render widgets
    match app.current_screen {
        CurrentScreen::Ant => {
            if app.ant_sim.grid.cells.is_empty() {
                // Initialize grid
                app.ant_sim.grid.cells = vec![
                    vec![app.ant_sim.states[0]; frame.area().width.into()];
                    (frame.area().height * 2).into()
                ];

                // Initialize ruleset
                app.ant_sim.rules = AntSim::parse_ant_ruleset(&app.ant_sim.rules_input);

                // Set ant position randomly biased towards the center
                let mut rng = rand::thread_rng();
                let width = f64::from(frame.area().width - 2);
                let height = f64::from((frame.area().height - 2) * 2);

                for ant in &mut app.ant_sim.ants {
                    ant.x = rng.gen_range((width * 0.4) as u64..(width - width * 0.4) as u64) as f64;
                    ant.y = rng.gen_range((height * 0.4) as u64..(height - height * 0.4) as u64) as f64;
                }

                // Set ant direction randomly
                for ant in &mut app.ant_sim.ants {
                    let direction = rng.gen_range(0..4);
                    ant.direction = match direction {
                        0 => crate::simulations::ant::Direction::Left,
                        1 => crate::simulations::ant::Direction::Right,
                        2 => crate::simulations::ant::Direction::Up,
                        3 => crate::simulations::ant::Direction::Down,
                        _ => crate::simulations::ant::Direction::Right,
                    };
                }
            }

            let top_title = Title::from(Line::from(vec![" Langton's Ant ".yellow()]))
                .position(Position::Top)
                .alignment(Alignment::Center);

            let bottom_left_title = Title::from(Line::from(vec![
                " Generation: ".into(),
                app.ant_sim.generation.to_string().yellow(),
                " ".into(),
            ]))
            .position(Position::Bottom);

            let key_hints = Title::from(Line::from(vec![" '?' ".yellow(), "Help ".into()]))
                .position(Position::Bottom)
                .alignment(Alignment::Center);

            let bottom_right_title = Title::from(Line::from(vec![
                " Speed: ".into(),
                app.speed.as_millis().to_string().yellow(),
                " ms ".into(),
            ]))
            .position(Position::Bottom)
            .alignment(Alignment::Right);

            let ant_canvas = Canvas::default()
                .block(
                    Block::default()
                        .border_type(BorderType::Double)
                        .borders(Borders::ALL)
                        .title(top_title)
                        .title(bottom_left_title)
                        .title(bottom_right_title)
                        .title(key_hints)
                        .title_style(Style::default().bold()),
                )
                .marker(app.marker)
                .paint(|ctx| {
                    // Draw grid
                    for (y, row) in app.ant_sim.grid.cells.iter().enumerate() {
                        for (x, cell) in row.iter().enumerate() {
                            match *cell {
                                // Skip drawing black cells
                                Color::Black => {}
                                _ => {
                                    ctx.draw(&Points {
                                        coords: &[(x as f64, y as f64)],
                                        color: *cell,
                                    });
                                }
                            }
                        }
                    }

                    // Draw ant
                    for ant in app.ant_sim.ants.iter() {
                        ctx.draw(&Points {
                            coords: &[(ant.x, ant.y)],
                            color: ant.color,
                        });
                    }
                })
                .x_bounds([0., f64::from((frame.area().width - 2) - 1)])
                .y_bounds([0., f64::from(((frame.area().height - 2) * 2) - 1)]);

            frame.render_widget(ant_canvas, frame.area());

            let keys = vec![
                Line::from("Q ".yellow()),
                Line::from("? ".yellow()),
                Line::from("Space ".yellow()),
                Line::from("L / → ".yellow()),
                Line::from("J / ↓ ".yellow()),
                Line::from("K / ↑ ".yellow()),
            ];

            let labels = vec![
                Line::from("Quit"),
                Line::from("Help"),
                Line::from("Start/Pause"),
                Line::from("Next Generation"),
                Line::from("Speed Down"),
                Line::from("Speed Up"),
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

            if app.help_screen {
                frame.render_widget(Clear, help_area);
                frame.render_widget(help_block, help_area);
                frame.render_widget(help_keys, help_center[0]);
                frame.render_widget(help_labels, help_center[1]);
            }
        }
        _ => {}
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect_percent(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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

fn centered_rect_length(cols: u16, rows: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((r.height - rows) / 2),
            Constraint::Length(rows),
            Constraint::Length((r.height - rows) / 2),
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
