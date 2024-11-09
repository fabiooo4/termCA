use crate::simulations::{ant::AntSim, elementary::ElementarySim};
use ratatui::{
    symbols::Marker,
    widgets::{ScrollbarState, TableState},
};
use std::time::Duration;

/// All the possible screens in the application
#[derive(Clone, Copy)]
pub enum Screen {
    Main,
    Exit,
    Ant,
    AntEdit(usize), // Screen for editing the ants' position and direction
    Elementary,
}

pub enum InputMode {
    Normal,
    Editing,
}

pub struct SimulationItem {
    pub label: String,
    pub screen: Screen,
}

/// Struct that holds the application data
pub struct App {
    pub current_screen: Screen,
    pub editing: Option<Screen>,
    pub help_screen: bool,
    pub is_running: bool,        // Pause/Resume
    pub speed: Duration,         // Delay between each generation
    pub speed_multiplier: usize, // Number of generations per frame
    pub marker: Marker,          // Character to draw the cells
    pub list_items: Vec<SimulationItem>,
    pub list_state: TableState, // State of the list
    pub scroll_state: ScrollbarState,

    /// Ant simulation data (optional because it's only used in the Ant
    /// screen)
    pub ant_sim: Option<AntSim>,
    pub elementary_sim: Option<ElementarySim>,
}

impl App {
    /// Constructs a new `App` with default values
    pub fn new() -> Self {
        let simulations_list = vec![
            SimulationItem {
                label: String::from("Langton's Ant\n "),
                screen: Screen::Ant,
            },
            SimulationItem {
                label: String::from("Elementary CA\n "),
                screen: Screen::Elementary,
            },
            SimulationItem {
                label: String::from("Exit\n "),
                screen: Screen::Exit,
            },
        ];

        App {
            help_screen: false,
            current_screen: Screen::Main,
            editing: None,
            is_running: false,
            speed: Duration::from_millis(80),
            speed_multiplier: 1,
            marker: Marker::HalfBlock,
            list_items: simulations_list,
            list_state: TableState::new().with_selected_cell((0, 0)),
            scroll_state: ScrollbarState::default(),

            ant_sim: None,
            elementary_sim: None,
        }
    }

    /// Increases simulation speed
    pub fn speed_increase(&mut self) {
        if self.speed > Duration::from_millis(100) {
            self.speed = self.speed.saturating_sub(Duration::from_millis(100));
        } else if self.speed > Duration::from_millis(10) {
            self.speed = self.speed.saturating_sub(Duration::from_millis(10));
        } else if self.speed > Duration::from_millis(0) {
            self.speed = self.speed.saturating_sub(Duration::from_millis(1));
        } else if self.speed_multiplier < 10 {
            self.speed_multiplier = self.speed_multiplier.saturating_add(1);
        } else if self.speed_multiplier < 100 {
            self.speed_multiplier = self.speed_multiplier.saturating_add(10);
        } else if self.speed_multiplier < 1000 {
            self.speed_multiplier = self.speed_multiplier.saturating_add(100);
        } else {
            self.speed_multiplier = self.speed_multiplier.saturating_add(1000);
        }
    }

    /// Decreases simulation speed
    pub fn speed_decrease(&mut self) {
        if self.speed_multiplier > 1 {
            if self.speed_multiplier > 1000 {
                self.speed_multiplier = self.speed_multiplier.saturating_sub(1000);
            } else if self.speed_multiplier > 100 {
                self.speed_multiplier = self.speed_multiplier.saturating_sub(100);
            } else if self.speed_multiplier > 10 {
                self.speed_multiplier = self.speed_multiplier.saturating_sub(10);
            } else {
                self.speed_multiplier = self.speed_multiplier.saturating_sub(1);
            }
        } else if self.speed < Duration::from_millis(10) {
            self.speed = self.speed.saturating_add(Duration::from_millis(1));
        } else if self.speed < Duration::from_millis(100) {
            self.speed = self.speed.saturating_add(Duration::from_millis(10));
        } else {
            self.speed = self.speed.saturating_add(Duration::from_millis(100));
        }
    }

    /// Applies the currently selected item of the list:
    /// - If a simulation is selected the screen changes to that simulation
    /// - If edit is selected enters edit mode on the selected simulation
    pub fn apply_selected(&mut self) {
        // If a simulation is selected from the list,
        // change the screen to that simulation
        if let Some(0) = self.list_state.selected_column() {
            if let Some(i) = self.list_state.selected() {
                self.current_screen = self.list_items[i].screen
            }
        } else {
            // If edit is selected, enter edit mode on the selected simulation
            if let Some(i) = self.list_state.selected() {
                match self.list_items[i].screen {
                    Screen::Ant => {
                        // Create a default ant simulation to be able to edit it
                        self.start_ant_default();
                        self.editing = Some(self.list_items[i].screen);
                    }
                    Screen::Elementary => {
                        self.start_elementary_default();
                        self.editing = Some(self.list_items[i].screen);
                    }
                    _ => {}
                }
            }
        }
    }

    /// Stops all simulations
    pub fn stop_all(&mut self) {
        self.ant_sim = None;
        self.elementary_sim = None;
        self.is_running = false;
        self.speed = Duration::from_millis(80);
        self.speed_multiplier = 1;
    }

    /// Starts the Langton's Ant simulation with default values
    pub fn start_ant_default(&mut self) {
        self.stop_all();
        self.ant_sim = Some(AntSim::default());
    }

    /// Starts the Elementary CA simulation with default values
    pub fn start_elementary_default(&mut self) {
        self.stop_all();
        self.elementary_sim = Some(ElementarySim::default());
    }

    // List handling
    pub fn select_first(&mut self) {
        if let Some(selected_column) = self.list_state.selected_column() {
            self.list_state.select_cell(Some((0, selected_column)));
        } else {
            self.list_state.select_first();
        }

        self.scroll_state = self.scroll_state.position(0);
    }

    pub fn select_last(&mut self) {
        self.list_state.select_first_column();
        self.list_state.select_last();

        self.scroll_state = self.scroll_state.position(100000);
    }

    pub fn select_next(&mut self) {
        if self.list_state.selected_column() == Some(1)
            && self.list_state.selected() == Some(self.list_items.len() - 2)
        {
            self.select_last()
        } else if self.list_state.selected().is_some()
            && self.list_state.selected() != Some(self.list_items.len() - 1)
        {
            self.list_state.select_next();
        }

        self.scroll_state = self.scroll_state.position(self.list_state.offset());
    }

    pub fn select_previous(&mut self) {
        if self.list_state.selected().is_some() && self.list_state.selected() != Some(0) {
            self.list_state.select_previous();
        }

        self.scroll_state = self.scroll_state.position(self.list_state.offset());
    }

    pub fn select_left(&mut self) {
        if self.list_state.selected().is_some() {
            self.list_state.select_previous_column();
        }
    }

    pub fn select_right(&mut self) {
        if self.list_state.selected().is_some()
            && self.list_state.selected() != Some(self.list_items.len() - 1)
        {
            self.list_state.select_next_column();
        }
    }
}
