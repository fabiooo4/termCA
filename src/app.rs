use crate::simulations::ant::AntSim;
use ratatui::{
    symbols::Marker,
    widgets::{ListItem, ListState},
};
use std::time::Duration;

/// All the possible screens in the application
#[derive(Clone, Copy)]
pub enum Screen {
    Main,
    Ant,
    Exit,
}

pub enum InputMode {
    Normal,
    Editing
}

pub struct SimulationItem<'a> {
    pub item: ListItem<'a>,
    pub screen: Screen,
}

/// Struct that holds the application data
pub struct App<'a> {
    pub current_screen: Screen,
    pub editing: Option<Screen>,
    pub help_screen: bool,
    pub is_running: bool,        // Pause/Resume
    pub speed: Duration,         // Delay between each generation
    pub speed_multiplier: usize, // Number of generations per frame
    pub marker: Marker,          // Character to draw the cells
    pub simulation_items: Vec<SimulationItem<'a>>,
    pub sim_list_state: ListState, // State of the list of CAs
    pub edit_list_state: ListState,

    /// Ant simulation data (optional because it's only used in the Ant
    /// screen)
    pub ant_sim: Option<AntSim>,
}

impl App<'_> {
    /// Constructs a new `App` with default values
    pub fn new() -> Self {
        App {
            help_screen: false,
            current_screen: Screen::Main,
            editing: None,
            is_running: false,
            speed: Duration::from_millis(80),
            speed_multiplier: 1,
            marker: Marker::HalfBlock,
            simulation_items: vec![
                SimulationItem {
                    item: ListItem::from(vec!["Langton's Ant".into(), "".into()]),
                    screen: Screen::Ant,
                },
                SimulationItem {
                    item: ListItem::from(vec!["Exit".into()]),
                    screen: Screen::Exit,
                },
            ],
            sim_list_state: ListState::default().with_selected(Some(0)),
            edit_list_state: ListState::default(),

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

    /// Starts the Langton's Ant simulation with default values
    pub fn start_ant_default(&mut self) {
        self.stop_all();
        self.ant_sim = Some(AntSim::default());
    }

    pub fn change_screen_selected(&mut self) {
        if let Some(i) = self.sim_list_state.selected() {
            self.current_screen = self.simulation_items[i].screen
        }

        if let Some(i) = self.edit_list_state.selected() {
            self.current_screen = self.simulation_items[i].screen
        }
    }
}
