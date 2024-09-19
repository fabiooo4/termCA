use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout}, widgets::{
        block::Title, canvas::{Canvas, Points, Rectangle}, Block, Paragraph
    }, Frame
};

use crate::app::{App, CurrentScreen};

pub fn ui(frame: &mut Frame, app: &App) {
    // Render widgets
    match app.current_screen {
        CurrentScreen::Ant => {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(0),
                    ]
                )
                .split(frame.area());

            let ant_canvas = Canvas::default()
                .block(Block::bordered().title(Title::from(format!("{}", frame.area().width)).alignment(Alignment::Center)))
                .marker(app.marker)
                .paint(|ctx| {
                    ctx.draw(&Points {
                        coords: &[(app.ant.x, app.ant.y)],
                        color: app.ant.color,
                    });
                })
                .x_bounds([0., (frame.area().width as f64)])
                .y_bounds([0., (frame.area().height as f64)*2.]);

            frame.render_widget(ant_canvas, frame.area());
        }
        _ => {}
    }
}
