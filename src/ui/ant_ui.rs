use ratatui::Frame;


use rand::Rng;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{
        block::{Position, Title},
        canvas::{Canvas, Points},
        Block, BorderType, Borders, Clear, Paragraph,
    },
};

use crate::{
    app::App,
    simulations::ant::AntSim,
};

use super::centered_rect_length;

pub fn ant_screen(frame: &mut Frame, app: &mut App) {
    // Initialize the ant simulation if it's not already
    if let None = app.ant_sim {
        app.start_ant_default();

        let width = f64::from(frame.area().width - 2);
        let height = f64::from((frame.area().height - 2) * 2);
        let ant_sim = app.ant_sim.as_mut().unwrap();

        // Initialize the grid with the same size as the canvas
        ant_sim.grid.cells = vec![vec![ant_sim.states[0]; width as usize]; height as usize];

        // Change default ruleset
        ant_sim.rules_input = String::from("RRLLLRLLLLLLLLL");
        ant_sim.rules = AntSim::parse_ant_ruleset(&ant_sim.rules_input);

        // Set ant position randomly biased towards the center
        let mut rng = rand::thread_rng();

        for ant in &mut ant_sim.ants {
            ant.x = rng.gen_range((width * 0.4) as usize..(width - width * 0.4) as usize) as usize;
            ant.y =
                rng.gen_range((height * 0.4) as usize..(height - height * 0.4) as usize) as usize;
        }

        // Set ant direction randomly
        for ant in &mut ant_sim.ants {
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

    // From here `app.ant_sim` is `Some`
    let ant_sim = app.ant_sim.as_ref().unwrap();

    /////////////////////////////
    // Border content
    /////////////////////////////

    let top_title = Title::from(Line::from(vec![" Langton's Ant ".yellow()]))
        .position(Position::Top)
        .alignment(Alignment::Center);

    let bottom_left_title = Title::from(Line::from(vec![
        " Generation: ".into(),
        ant_sim.generation.to_string().yellow(),
        " ".into(),
    ]))
    .position(Position::Bottom);

    let key_hints = Title::from(Line::from(vec![" '?' ".yellow(), "Help ".into()]))
        .position(Position::Bottom)
        .alignment(Alignment::Center);

    let bottom_right_title = Title::from(Line::from(vec![
        " Speed: ".into(),
        if app.speed.as_millis() == 0 {
            format!("{}x ", app.speed_multiplier).yellow()
        } else {
            format!("{}ms ", app.speed.as_millis()).yellow()
        },
    ]))
    .position(Position::Bottom)
    .alignment(Alignment::Right);

    /* let top_left_debug = Title::from(Line::from(vec![
        "(".into(),
        ant_sim.ants[0].x.to_string().yellow(),
        ",".into(),
        ant_sim.ants[0].y.to_string().yellow(),
        ")".into(),
        " ".into(),
        ant_sim.ants[0].direction.to_string().yellow(),
        " ".into(),
        Span::styled(
            ant_sim.states[ant_sim.generation % ant_sim.states.len()].to_string(),
            Style::default().fg(ant_sim.states[ant_sim.generation % ant_sim.states.len()]),
        ),
    ])); */

    /////////////////////////////
    // Simulation canvas
    /////////////////////////////

    let ant_canvas = Canvas::default()
        .block(
            Block::default()
                .border_type(BorderType::Double)
                .borders(Borders::ALL)
                // .title(top_left_debug)
                .title(top_title)
                .title(bottom_left_title)
                .title(bottom_right_title)
                .title(key_hints)
                .title_style(Style::default().bold()),
        )
        .marker(app.marker)
        .paint(|ctx| {
            // Draw grid
            for (y, row) in ant_sim.grid.cells.iter().enumerate() {
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
            for ant in ant_sim.ants.iter() {
                ctx.draw(&Points {
                    coords: &[(ant.x as f64, ant.y as f64)],
                    color: ant.color,
                });
            }
        })
        .x_bounds([0., f64::from((frame.area().width - 2) - 1)])
        .y_bounds([0., f64::from(((frame.area().height - 2) * 2) - 1)]);

    frame.render_widget(ant_canvas, frame.area());

    /////////////////////////////
    // Help screen
    /////////////////////////////

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
