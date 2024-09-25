use crate::simulations::ant::{Ant, AntSim, Direction, Grid};
use ratatui::{style::Color, symbols::Marker, text::Line, widgets::ListState};
use std::time::Duration;

/// All the possible screens in the application
#[derive(Clone, Copy)]
pub enum CurrentScreen {
    Main,
    Ant,
    Exit,
}

pub struct ListItem<'a> {
    pub line: Line<'a>,
    pub screen: CurrentScreen,
}

/// Struct that holds the application data
pub struct App<'a> {
    pub current_screen: CurrentScreen,
    pub help_screen: bool,
    pub is_running: bool,        // Pause/Resume
    pub speed: Duration,         // Delay between each generation
    pub speed_multiplier: usize, // Number of generations per frame
    pub marker: Marker,          // Character to draw the cells
    pub list_items: Vec<ListItem<'a>>,
    pub list_state: ListState, // State of the list of CAs

    /// Ant simulation data (optional because it's only used in the Ant
    /// screen)
    pub ant_sim: Option<AntSim>,
}

impl App<'_> {
    /// Constructs a new `App` with default values
    pub fn new() -> Self {
        App {
            help_screen: false,
            current_screen: CurrentScreen::Main,
            is_running: false,
            speed: Duration::from_millis(80),
            speed_multiplier: 1,
            marker: Marker::HalfBlock,
            list_items: vec![
                ListItem {
                    line: Line::from("Langton's Ant"),
                    screen: CurrentScreen::Ant,
                },
                ListItem {
                    line: Line::from("Todo"),
                    screen: CurrentScreen::Exit,
                }
            ],
            list_state: ListState::default(),

            ant_sim: None,
        }
    }

    /// Stops all simulations
    pub fn stop_all(&mut self) {
        self.ant_sim = None;
        self.is_running = false;
        self.speed = Duration::from_millis(80);
        self.speed_multiplier = 1;
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

    pub fn select_none(&mut self) {
        self.list_state.select(None);
    }

    pub fn select_next(&mut self) {
        self.list_state.select_next();
    }
    pub fn select_previous(&mut self) {
        self.list_state.select_previous();
    }

    pub fn select_first(&mut self) {
        self.list_state.select_first();
    }

    pub fn select_last(&mut self) {
        self.list_state.select_last();
    }

    pub fn change_screen(&mut self) {
        if let Some(i) = self.list_state.selected() {
            self.current_screen = self.list_items[i].screen
        }
    }

}
