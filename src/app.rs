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
                ants: vec![
                    Ant::new(),
                    Ant::new(),
                ],
                rules_input: String::from("LR"),
                grid: Grid::new(),
                states: vec![Color::Black, Color::Yellow],
                rules: vec![
                    Direction::Left,
                    Direction::Right,
                ],
                generation: 0,
            },
        }
    }
}
