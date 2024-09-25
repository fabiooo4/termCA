use std::fmt::{self, Display, Formatter};

use crate::app::App;
use ratatui::style::Color;

/// Struct that holds the ant simulation data
pub struct AntSim {
    pub ants: Vec<Ant>,
    /// Vector that holds the ants
    pub rules_input: String, // Ant ruleset
    pub grid: Grid,            // Grid of cells
    pub states: Vec<Color>,    // Possible states of the cells
    pub rules: Vec<Direction>, // Rules for the ant
    pub generation: usize,     // Number of generations
}

/// Struct that holds the ant data
pub struct Ant {
    pub x: usize,
    pub y: usize,
    pub color: Color,
    pub direction: Direction,
}

impl Ant {
    /// Constructs a new empty `Ant`
    pub fn new() -> Self {
        Ant {
            x: 0,
            y: 0,
            color: Color::Indexed(16),
            direction: Direction::Right,
        }
    }
}

/// Enum that represents the 2D directions
/// ```plain
///     U
///     |
/// L --|-- R
///     |
///     D
/// ```
#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    /// Returns the direction to the left of the current direction
    pub fn turn_left(&self) -> Self {
        match self {
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
        }
    }

    /// Returns the direction to the right of the current direction
    pub fn turn_right(&self) -> Self {
        match self {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        }
    }

    /// Returns the opposite direction of the current direction
    pub fn turn_opposite(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }

    /// Returns the direction relative to the current direction
    /// - `Left` -> turn left
    /// - `Right` -> turn right
    /// - `Up` -> continue in the same direction
    /// - `Down` -> turn opposite
    pub fn turn(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Left => self.turn_left(),
            Direction::Right => self.turn_right(),
            Direction::Up => *self,
            Direction::Down => self.turn_opposite(),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Direction::Left => write!(f, "←"),
            Direction::Right => write!(f, "→"),
            Direction::Up => write!(f, "↑"),
            Direction::Down => write!(f, "↓"),
        }
    }
}

/// Struct that represents a grid of cells
pub struct Grid {
    pub cells: Vec<Vec<Color>>,
}

impl Grid {
    /// Constructs a new empty `Grid`
    pub fn new() -> Self {
        Grid { cells: Vec::new() }
    }

    // Resizes the grid in-place by providing the new dimensions and a new state
    pub fn resize(&mut self, new_width: usize, new_height: usize, new_state: Color) {
        for row in self.cells.iter_mut() {
            row.resize(new_width as usize, new_state);
        }

        self.cells
            .resize(new_height as usize, vec![new_state; new_width as usize]);
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
            for ant in ant_sim.ants.iter_mut() {
                for _ in 0..app.speed_multiplier {
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
                    grid.cells[0].len() - 1
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
                    grid.cells.len() - 1
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
