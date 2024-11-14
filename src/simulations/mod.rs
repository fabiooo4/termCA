pub mod ant;
pub mod elementary;
pub mod game_of_life;


use ratatui::style::Color;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;


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
#[derive(Clone)]
pub struct Grid {
    pub cells: Vec<Vec<Color>>,
}

impl Grid {
    /// Constructs a new empty `Grid`
    pub fn new() -> Self {
        Grid { cells: Vec::new() }
    }

    /// Resizes the grid in-place by providing the new dimensions and a new state
    pub fn resize(&mut self, new_width: usize, new_height: usize, new_state: Color) {
        for row in self.cells.iter_mut() {
            row.resize(new_width, new_state);
        }

        self.cells
            .resize(new_height, vec![new_state; new_width]);
    }

    /// Returns the width of the grid
    pub fn width(&self) -> usize {
        if self.cells.is_empty() {
            0
        } else {
            self.cells[0].len()
        }
    }

    /// Returns the height of the grid
    pub fn height(&self) -> usize {
        if self.cells.is_empty() {
            0
        } else {
            self.cells.len()
        }
    }
}
