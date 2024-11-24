use crate::app::InputMode;
use ratatui::style::Color;
use tui_input::Input;
use tui_scrollview::ScrollViewState;
use tui_widget_list::ListState;

use super::{Direction, Grid};

/// Struct that holds the ant simulation data
pub struct AntSim {
    pub ants: Vec<Ant>,        // Vector that holds the ants
    pub grid: Grid,            // Grid of cells
    pub states: Vec<Color>,    // Possible states of the cells
    pub rules: Vec<Direction>, // Rules for the ant
    pub generation: usize,     // Number of generations

    // Edit state

    pub settings_state: ListState,

    pub rules_input: Input,          // Input widget
    pub rules_input_mode: InputMode, // Input mode

    pub scroll_state: ScrollViewState, // State of the edit scroll view
    pub edit_item_selected: usize,     // Index of selected item in edit mode
}

impl Default for AntSim {
    fn default() -> Self {
        let mut list_state = ListState::default();
        list_state.selected = Some(0);
        AntSim {
            ants: vec![Ant::default()],
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
                Color::Indexed(17),
            ],
            rules: vec![Direction::Right, Direction::Left],
            generation: 0,

            settings_state: list_state,

            rules_input: Input::from(String::from("RL")),
            rules_input_mode: InputMode::Normal,

            scroll_state: ScrollViewState::default(),
            edit_item_selected: 0,
        }
    }
}

/// Struct that holds the ant data
pub struct Ant {
    pub x: usize,
    pub y: usize,
    pub color: Color,
    pub direction: Direction,
}

impl Default for Ant {
    /// Constructs a new empty `Ant`
    fn default() -> Self {
        Ant {
            // Set to invalid position to reposition in the center of the screen when the
            // frame is available
            x: usize::MAX,
            y: usize::MAX,
            color: Color::Indexed(16),
            direction: Direction::Up,
        }
    }
}

impl Ant {
    /// Move the ant in the specified direction with grid wrapping
    pub fn change_position(&mut self, direction: Direction, grid: &Grid) {
        match direction {
            Direction::Left => {
                self.x = if self.x > 0 {
                    self.x - 1
                } else {
                    grid.width() - 1
                };
            }
            Direction::Right => {
                self.x = if self.x < (grid.width() - 1) {
                    self.x + 1
                } else {
                    0
                };
            }
            Direction::Up => {
                self.y = if self.y < (grid.height() - 1) {
                    self.y + 1
                } else {
                    0
                };
            }
            Direction::Down => {
                self.y = if self.y > 0 {
                    self.y - 1
                } else {
                    grid.height() - 1
                };
            }
        }
    }
}

impl AntSim {
    /// Parses the ant ruleset from a string
    /// - `L` -> turn left
    /// - `R` -> turn right
    /// - `F` -> continue in the same direction (Forward)
    /// - `B` -> turn opposite (Backward)
    ///
    /// # Example
    /// ```
    /// assert_eq!(parse_ant_ruleset("LRFB"), vec![
    ///    Direction::Left,
    ///    Direction::Right,
    ///    Direction::Up,
    ///    Direction::Down,
    /// ]);
    /// ```
    pub fn parse_ant_ruleset(rules: &str) -> Vec<Direction> {
        let mut ruleset = Vec::new();
        for c in rules.to_uppercase().chars() {
            match c {
                'L' => ruleset.push(Direction::Left),
                'R' => ruleset.push(Direction::Right),
                'F' => ruleset.push(Direction::Up),
                'B' => ruleset.push(Direction::Down),
                _ => {}
            }
        }

        ruleset
    }

    /// Standard Langton's Ant simulation
    pub fn run(&mut self, speed_multiplier: usize) {
        for _ in 0..speed_multiplier {
            for ant in self.ants.iter_mut() {
                Self::ant_turn(ant, &self.grid, &self.states, &self.rules);
                Self::ant_flip(ant, &mut self.grid, &self.states, &self.rules);
                Self::ant_forward(ant, &self.grid);
            }
        }
        self.generation = self.generation.saturating_add(speed_multiplier);
    }

    /// Moves the ant forward based on its direction with grid wrapping
    pub fn ant_forward(ant: &mut Ant, grid: &Grid) {
        ant.change_position(ant.direction, grid);
    }

    /// Turns the ant based on the current cell state and rule
    pub fn ant_turn(ant: &mut Ant, grid: &Grid, states: &[Color], rules: &[Direction]) {
        for (state, rule) in states.iter().zip(rules.iter()) {
            if grid.cells[ant.y][ant.x] == *state {
                ant.direction = ant.direction.turn(rule);
                break;
            }
        }
    }

    /// Flips the current cell state based on the rule
    pub fn ant_flip(ant: &Ant, grid: &mut Grid, states: &[Color], rules: &[Direction]) {
        let rules_len = rules.len();
        let mut states = states[0..rules_len].iter().cycle();

        // Assign the next state to the current cell
        while let Some(state) = states.next() {
            if grid.cells[ant.y][ant.x] == *state {
                grid.cells[ant.y][ant.x] = *states.next().unwrap();
                break;
            }
        }
    }
}

pub enum AntSettings {
    Ruleset,
    Start,
}

impl AntSettings {
    pub const COUNT: usize = 2;
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => AntSettings::Ruleset,
            1 => AntSettings::Start,
            _ => AntSettings::Ruleset,
        }
    }
}

impl std::fmt::Display for AntSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AntSettings::Ruleset => write!(f, "Ruleset"),
            AntSettings::Start => write!(f, "Start"),
        }
    }
}
