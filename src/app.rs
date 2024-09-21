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
            is_running: false,
            speed: Duration::from_millis(80),
            marker: Marker::HalfBlock,
            ant_sim: AntSim {
                ants: vec![Ant::new()],
                rules_input: String::from("RL"),
                grid: Grid::new(),
                states: vec![
                    Color::Reset,
                    Color::Indexed(3),
                    Color::Indexed(1),
                    Color::Indexed(2),
                    Color::Indexed(4),
                    Color::Indexed(5),
                    Color::Indexed(6),
                    Color::Indexed(9),
                    Color::Indexed(10),
                    Color::Indexed(11),
                    Color::Indexed(12),
                    Color::Indexed(13),
                    Color::Indexed(14),
                    Color::Indexed(7),
                    Color::Indexed(8),
                    Color::Indexed(15),
                ],
                rules: Vec::new(),
                generation: 0,
            },
        }
    }
}
