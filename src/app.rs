use std::time::Duration;

use ratatui::{style::Color, symbols::Marker};

pub enum CurrentScreen {
    Main,
    Ant,
    Exit,
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub enum CellState {
    Alive,
    Dead,
}

impl Clone for CellState {
    fn clone(&self) -> Self {
        match self {
            CellState::Alive => CellState::Alive,
            CellState::Dead => CellState::Dead,
        }
    }
}

pub struct Ant {
    pub x: f64,
    pub y: f64,
    pub color: Color,
    pub direction: Direction,
}

pub struct Grid {
    pub alive_color: Color,
    pub cells: Vec<Vec<CellState>>,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub ant: Ant, // Langton's Ant
    pub ant_grid: Grid, // Grid of cells
    pub generation: u64, // Number of generations
    pub is_running: bool, // Pause/Resume
    pub speed: Duration, // Delay between each generation
    pub marker: Marker,
}

impl App {
    pub fn new() -> Self {
        App {
            current_screen: CurrentScreen::Ant,
            ant: Ant {
                x: 0.0,
                y: 0.0,
                color: Color::Yellow,
                direction: Direction::Right,
            },
            ant_grid: Grid {
                alive_color: Color::White,
                cells: Vec::new(),
            },
            generation: 0,
            is_running: true,
            speed: Duration::from_millis(80),
            marker: Marker::HalfBlock,
        }
    }

    pub fn run_ant(&mut self) {
        Self::ant_forward(self);
        Self::ant_turn(self);
        Self::ant_flip(self);
        self.generation += 1;
    }

    pub fn ant_forward(&mut self) {
        match self.ant.direction {
            Direction::Left => {
                if self.ant.x > 0.0 {
                    self.ant.x -= 1.0;
                }
            }
            Direction::Right => {
                if self.ant.x < (self.ant_grid.cells[0].len() - 1) as f64 {
                    self.ant.x += 1.0;
                }
            }
            Direction::Up => {
                if self.ant.y > 0.0 {
                    self.ant.y -= 1.0;
                }
            }
            Direction::Down => {
                if self.ant.y < (self.ant_grid.cells.len() - 1) as f64 {
                    self.ant.y += 1.0;
                }
            }
        }
    }

    pub fn ant_turn(&mut self) {
        match self.ant_grid.cells[self.ant.y as usize][self.ant.x as usize] {
            CellState::Alive => {
                // Turn right
                self.ant.direction = match self.ant.direction {
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                };
            }
            CellState::Dead => {
                // Turn left
                self.ant.direction = match self.ant.direction {
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                };
            }
        }
    }

    pub fn ant_flip(&mut self) {
        self.ant_grid.cells[self.ant.y as usize][self.ant.x as usize] =
            match self.ant_grid.cells[self.ant.y as usize][self.ant.x as usize] {
                CellState::Alive => CellState::Dead,
                CellState::Dead => CellState::Alive,
            };
    }
}
