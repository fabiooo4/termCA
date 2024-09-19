use ratatui::{
    layout::Alignment,
    style::{Style, Stylize},
    text::Line,
    widgets::{
        block::{Position, Title},
        canvas::{Canvas, Points},
        Block, BorderType, Borders,
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

            let top_title = Title::from(Line::from(vec![" Langton's Ant ".into()]))
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
        }
        _ => {}
    }
}
