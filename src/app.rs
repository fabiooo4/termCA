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

pub struct Ant {
    pub x: f64,
    pub y: f64,
    pub color: Color,
    pub direction: Direction,
}

pub struct Grid {
    pub cells: Vec<Vec<Color>>,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub help_screen: bool,
    pub ant: Ant,            // Langton's Ant
    pub rules_input: String, // Ant ruleset
    pub ant_grid: Grid,      // Grid of cells
    pub states: Vec<Color>,
    pub rules: Vec<Direction>,
    pub generation: u64,  // Number of generations
    pub is_running: bool, // Pause/Resume
    pub speed: Duration,  // Delay between each generation
    pub marker: Marker,
}

impl App {
    pub fn new() -> Self {
        App {
            help_screen: false,
            current_screen: CurrentScreen::Ant,
            ant: Ant {
                x: 0.0,
                y: 0.0,
                color: Color::Yellow,
                direction: Direction::Right,
            },
            rules_input: String::from("LR"),
            ant_grid: Grid { cells: Vec::new() },
            states: vec![Color::Black, Color::Red, Color::Yellow, Color::Green],
            rules: vec![Direction::Left, Direction::Right, Direction::Left, Direction::Up],
            generation: 0,
            is_running: true,
            speed: Duration::from_millis(80),
            marker: Marker::HalfBlock,
        }
    }

    pub fn parse_ant_ruleset(&mut self) {
        self.rules.clear();
        for c in self.rules_input.chars() {
            match c {
                'L' => self.rules.push(Direction::Left),
                'R' => self.rules.push(Direction::Right),
                'F' => self.rules.push(Direction::Up),
                'B' => self.rules.push(Direction::Down),
                _ => {}
            }
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
        for (state, rule) in self.states.iter().zip(self.rules.iter()) {
            if self.ant_grid.cells[self.ant.y as usize][self.ant.x as usize] == *state {
                match rule {
                    Direction::Left => {
                        self.ant.direction = match self.ant.direction {
                            Direction::Left => Direction::Down,
                            Direction::Right => Direction::Up,
                            Direction::Up => Direction::Left,
                            Direction::Down => Direction::Right,
                        };
                    }
                    Direction::Right => {
                        self.ant.direction = match self.ant.direction {
                            Direction::Left => Direction::Up,
                            Direction::Right => Direction::Down,
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                        };
                    }
                    Direction::Down => {
                        self.ant.direction = match self.ant.direction {
                            Direction::Left => Direction::Right,
                            Direction::Right => Direction::Left,
                            Direction::Up => Direction::Down,
                            Direction::Down => Direction::Up,
                        };
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn ant_flip(&mut self) {
        let states = self.states.clone();
        let mut states = states.iter().cycle();

        // Assign the next state to the current cell
        while let Some(state) = states.next() {
            if self.ant_grid.cells[self.ant.y as usize][self.ant.x as usize] == *state {
                self.ant_grid.cells[self.ant.y as usize][self.ant.x as usize] = states.next().unwrap().clone();
                break;
            }
        }
    }
}
