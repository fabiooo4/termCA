use crate::simulations::ant::{Ant, AntSim, Direction, Grid};
use ratatui::{style::Color, symbols::Marker};
use std::time::Duration;

/// All the possible screens in the application
pub enum CurrentScreen {
    Main,
    Ant,
    Exit,
}


/// Struct that holds the application data
pub struct App {
    pub current_screen: CurrentScreen,
    pub help_screen: bool,
    pub is_running: bool, // Pause/Resume
    pub speed: Duration,  // Delay between each generation
    pub speed_multiplier: usize, // Number of generations per frame 
    pub marker: Marker,   // Character to draw the cells

    /// Ant simulation data (optional because it's only used in the Ant
    /// screen)
    pub ant_sim: Option<AntSim>,
}

impl App {
    /// Constructs a new `App` with default values
    pub fn new() -> Self {
        App {
            help_screen: false,
            current_screen: CurrentScreen::Ant,
            is_running: false,
            speed: Duration::from_millis(80),
            speed_multiplier: 1,
            marker: Marker::HalfBlock,

            ant_sim: None,
        }
    }

    /// Stops all simulations
    pub fn stop_all(&mut self) {
        self.ant_sim = None;
    }

    /// Starts the Langton's Ant simulation with the given `AntSim`
    pub fn start_ant(&mut self, ant_sim: AntSim) {
        self.stop_all();
        self.ant_sim = Some(ant_sim);
    }

    pub fn start_ant_default(&mut self) {
        self.stop_all();
        self.ant_sim = Some(AntSim {
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
            rules: vec![Direction::Right, Direction::Left],
            generation: 0,
        });
    }
}
