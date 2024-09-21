use std::time::Duration;

use ratatui::{style::Color, symbols::Marker};

use crate::simulations::ant::{Ant, AntSim, Direction, Grid};

pub enum CurrentScreen {
    Main,
    Ant,
    Exit,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub help_screen: bool,
    pub is_running: bool, // Pause/Resume
    pub speed: Duration,  // Delay between each generation
    pub marker: Marker,

    pub ant_sim: AntSim,
}

impl App {
    pub fn new() -> Self {
        App {
            help_screen: false,
            current_screen: CurrentScreen::Ant,
            is_running: true,
            speed: Duration::from_millis(80),
            marker: Marker::HalfBlock,
            ant_sim: AntSim {
                ant: Ant {
                    x: 0.0,
                    y: 0.0,
                    color: Color::Black,
                    direction: Direction::Right,
                },
                rules_input: String::from("LR"),
                ant_grid: Grid { cells: Vec::new() },
                states: vec![Color::Black, Color::Red, Color::Yellow, Color::Green],
                rules: vec![
                    Direction::Left,
                    Direction::Right,
                    Direction::Left,
                    Direction::Up,
                ],
                generation: 0,
            },
        }
    }
}
