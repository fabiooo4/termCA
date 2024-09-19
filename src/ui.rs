use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{
        block::Title,
        canvas::{Canvas, Points},
        Block, BorderType, Borders, Padding,
    },
    Frame,
};

use crate::app::{App, CurrentScreen};

pub fn ui(frame: &mut Frame, app: &App) {
    // Render widgets
    match app.current_screen {
        CurrentScreen::Ant => {
            let ant_canvas = Canvas::default()
                .block(
                    Block::default()
                        .border_type(BorderType::Double)
                        .borders(Borders::ALL)
                        .title(
                            Title::from(format!(
                                " Langton's Ant {} {}",
                                frame.area().width,
                                frame.area().height
                            ))
                            .alignment(Alignment::Center),
                        )
                        .title_style(Style::default().light_red().bold()),
                )
                .marker(app.marker)
                .paint(|ctx| {
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
