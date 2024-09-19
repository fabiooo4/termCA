use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, ToText},
    widgets::{
        block::{Position, Title},
        canvas::{Canvas, Points},
        Block, BorderType, Borders, Clear, Paragraph,
    },
    Frame,
};

use crate::app::{App, CellState, CurrentScreen};

pub fn ui(frame: &mut Frame, app: &mut App) {
    // Render widgets
    match app.current_screen {
        CurrentScreen::Ant => {
            if app.ant_grid.cells.is_empty() {
                app.ant_grid.cells = vec![
                    vec![CellState::Dead; frame.area().width.into()];
                    (frame.area().height * 2).into()
                ];

                // Set ant position to the middle of the grid
                app.ant.x = f64::from((frame.area().width - 3) / 2);
                app.ant.y = f64::from(frame.area().height - 2);
            }

            let top_title = Title::from(Line::from(vec![" Langton's Ant ".yellow()]))
                .position(Position::Top)
                .alignment(Alignment::Center);

            let bottom_left_title = Title::from(Line::from(vec![
                " Generation: ".into(),
                app.generation.to_string().yellow(),
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
                    for (y, row) in app.ant_grid.cells.iter().enumerate() {
                        for (x, cell) in row.iter().enumerate() {
                            match cell {
                                crate::app::CellState::Alive => {
                                    ctx.draw(&Points {
                                        coords: &[(x as f64, y as f64)],
                                        color: app.ant_grid.alive_color,
                                    });
                                }
                                crate::app::CellState::Dead => {}
                            };
                        }
                    }

                    // Draw ant
                    ctx.draw(&Points {
                        coords: &[(app.ant.x, app.ant.y)],
                        color: app.ant.color,
                    });
                })
                .x_bounds([0., f64::from((frame.area().width - 2) - 1)])
                .y_bounds([0., f64::from(((frame.area().height - 2) * 2) - 1)]);

            frame.render_widget(ant_canvas, frame.area());

            let keys = vec![
                Line::from("Q ".yellow()),
                Line::from("Space ".yellow()),
                Line::from("? ".yellow()),
                Line::from("L / → ".yellow()),
                Line::from("H / ← ".yellow()),
                Line::from("J / ↓ ".yellow()),
                Line::from("K / ↑ ".yellow()),
            ];

            let labels = vec![
                Line::from("Quit"),
                Line::from("Start/Pause"),
                Line::from("Help"),
                Line::from("Next Generation"),
                Line::from("Previous Generation"),
                Line::from("Speed Down"),
                Line::from("Speed Up"),
            ];

            let help_area = centered_rect_length(31, (keys.len() + 4) as u16, frame.area());
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
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
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
