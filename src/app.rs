use crate::simulations::ant::AntSim;
use ratatui::{
    symbols::Marker,
    widgets::{ListItem, ListState, ScrollbarState},
};
use std::time::Duration;

/// All the possible screens in the application
#[derive(Clone, Copy)]
pub enum Screen {
    Main,
    Ant,
    AntEdit(usize), // Screen for editing the ants' position and direction
    Exit,
}

pub enum InputMode {
    Normal,
    Editing,
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
    pub edit_items: Vec<ListItem<'a>>,
    pub sim_list_state: ListState,  // State of the list of CAs
    pub edit_list_state: ListState, // State of the list of edits
    pub scroll_state: ScrollbarState,

    /// Ant simulation data (optional because it's only used in the Ant
    /// screen)
    pub ant_sim: Option<AntSim>,
}

impl App<'_> {
    /// Constructs a new `App` with default values
    pub fn new() -> Self {
        let simulations_list = vec![
            SimulationItem {
                item: ListItem::new("Langton's Ant\n "),
                screen: Screen::Ant,
            },
            SimulationItem {
                item: ListItem::new("Exit\n "),
                screen: Screen::Exit,
            },
        ];

        let mut edit_list = vec![ListItem::new("Edit\n "); simulations_list.len() - 1];
        edit_list.push(ListItem::new(" \n "));

        App {
            help_screen: false,
            current_screen: Screen::Main,
            editing: None,
            is_running: false,
            speed: Duration::from_millis(80),
            speed_multiplier: 1,
            marker: Marker::HalfBlock,
            simulation_items: simulations_list,
            edit_items: edit_list,
            sim_list_state: ListState::default().with_selected(Some(0)),
            edit_list_state: ListState::default(),
            scroll_state: ScrollbarState::default(),

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

    // List handling
    pub fn select_first(&mut self) {
        if self.sim_list_state.selected().is_some() {
            self.sim_list_state.select_first();
        } else {
            self.edit_list_state.select_first();
        }
    }

    pub fn select_last(&mut self) {
        self.sim_list_state.select_last();
    }

    pub fn select_next(&mut self) {
        if self.sim_list_state.selected().is_some()
            && self.sim_list_state.selected() != Some(self.simulation_items.len() - 1)
        {
            self.sim_list_state.select_next();
        }

        if self.edit_list_state.selected().is_some()
            && self.edit_list_state.selected() != Some(self.edit_items.len() - 2)
        {
            self.edit_list_state.select_next();
        } else if self.edit_list_state.selected().is_some()
            && self.edit_list_state.selected() != Some(self.edit_items.len() - 1)
        {
            self.sim_list_state.select_last();
        }
    }

    pub fn select_previous(&mut self) {
        if self.sim_list_state.selected().is_some() && self.sim_list_state.selected() != Some(0) {
            self.sim_list_state.select_previous();
        }

        if self.edit_list_state.selected().is_some() && self.edit_list_state.selected() != Some(0) {
            self.edit_list_state.select_previous();
        }
    }

    pub fn select_left(&mut self) {
        if self.edit_list_state.selected().is_some() {
            self.sim_list_state.select(self.edit_list_state.selected());
            self.edit_list_state.select(None);
            self.sync_lists();
        }
    }

    pub fn select_right(&mut self) {
        if self.sim_list_state.selected().is_some()
            && self.sim_list_state.selected() != Some(self.simulation_items.len() - 1)
        {
            self.edit_list_state.select(self.sim_list_state.selected());
            self.sim_list_state.select(None);
            self.sync_lists();
        }
    }

    pub fn sync_lists(&mut self) {
        if self.sim_list_state.selected().is_some() {
            self.edit_list_state = ListState::default().with_offset(self.sim_list_state.offset());
        }

        if self.edit_list_state.selected().is_some() {
            self.sim_list_state = ListState::default().with_offset(self.edit_list_state.offset());
        }
        self.scroll_state = self.scroll_state.position(self.sim_list_state.offset());
    }
}
