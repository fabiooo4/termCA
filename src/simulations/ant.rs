use crate::app::{App, InputMode};
use ratatui::style::Color;
use tui_input::Input;
use tui_scrollview::ScrollViewState;

use super::{Direction, Grid};

/// Struct that holds the ant simulation data
pub struct AntSim {
    pub ants: Vec<Ant>,        // Vector that holds the ants
    pub grid: Grid,            // Grid of cells
    pub states: Vec<Color>,    // Possible states of the cells
    pub rules: Vec<Direction>, // Rules for the ant
    pub generation: usize,     // Number of generations

    pub rules_input: Input,          // Input widget
    pub rules_input_mode: InputMode, // Input mode

    pub scroll_state: ScrollViewState, // State of the edit scroll view
    pub edit_item_selected: usize,     // Index of selected item in edit mode
}

impl Default for AntSim {
    fn default() -> Self {
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
            ],
            rules: vec![Direction::Right, Direction::Left],
            generation: 0,

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
            // Set position to invalid position to reposition in the center of the screen when the
            // frame is available
            x: usize::MAX,
            y: usize::MAX,
            color: Color::Indexed(16),
            direction: Direction::Up,
        }
    }
}

impl Ant {
    /// Constructs a new `Ant` given the position and direction
    pub fn new(x: usize, y: usize, direction: Direction) -> Self {
        Ant {
            x,
            y,
            color: Color::Indexed(16),
            direction,
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
    pub fn run_ant_sim(app: &mut App) {
        if let Some(ref mut ant_sim) = app.ant_sim {
            for _ in 0..app.speed_multiplier {
                for ant in ant_sim.ants.iter_mut() {
                    Self::ant_turn(ant, &ant_sim.grid, &ant_sim.states, &ant_sim.rules);
                    Self::ant_flip(ant, &mut ant_sim.grid, &ant_sim.states, &ant_sim.rules);
                    Self::ant_forward(ant, &ant_sim.grid);
                }
            }
            ant_sim.generation = ant_sim.generation.saturating_add(1 * app.speed_multiplier);
        }
    }

    /// Moves the ant forward based on its direction with grid wrapping
    pub fn ant_forward(ant: &mut Ant, grid: &Grid) {
        match ant.direction {
            Direction::Left => {
                ant.x = if ant.x > 0 {
                    ant.x - 1
                } else {
                    grid.width() - 1
                };
            }
            Direction::Right => {
                ant.x = if ant.x < (grid.cells[0].len() - 1) {
                    ant.x + 1
                } else {
                    0
                };
            }
            Direction::Up => {
                ant.y = if ant.y < (grid.cells.len() - 1) {
                    ant.y + 1
                } else {
                    0
                };
            }
            Direction::Down => {
                ant.y = if ant.y > 0 {
                    ant.y - 1
                } else {
                    grid.height() - 1
                };
            }
        }
    }

    /// Turns the ant based on the current cell state and rule
    pub fn ant_turn(ant: &mut Ant, grid: &Grid, states: &Vec<Color>, rules: &Vec<Direction>) {
        for (state, rule) in states.iter().zip(rules.iter()) {
            if grid.cells[ant.y][ant.x] == *state {
                ant.direction = ant.direction.turn(rule);
                break;
            }
        }
    }

    /// Flips the current cell state based on the rule
    pub fn ant_flip(ant: &Ant, grid: &mut Grid, states: &Vec<Color>, rules: &Vec<Direction>) {
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
