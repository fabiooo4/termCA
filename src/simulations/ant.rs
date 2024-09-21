use ratatui::style::Color;

use crate::app::App;

pub struct AntSim {
    pub ants: Vec<Ant>,            // Langton's Ant
    pub rules_input: String, // Ant ruleset
    pub grid: Grid,      // Grid of cells
    pub states: Vec<Color>,
    pub rules: Vec<Direction>,
    pub generation: u64, // Number of generations
}

pub struct Ant {
    pub x: f64,
    pub y: f64,
    pub color: Color,
    pub direction: Direction,
}

impl Ant {
    pub fn new() -> Self {
        Ant {
            x: 0.0,
            y: 0.0,
            color: Color::Black,
            direction: Direction::Right,
        }
    }
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct Grid {
    pub cells: Vec<Vec<Color>>,
}

impl Grid {
    pub fn new() -> Self {
        Grid { cells: Vec::new() }
    }
}

impl AntSim {
    pub fn parse_ant_ruleset(rules: &str) -> Vec<Direction> {
        let mut ruleset = Vec::new();
        for c in rules.chars() {
            match c {
                'L' => ruleset.push(Direction::Left),
                'R' => ruleset.push(Direction::Right),
                'F' => ruleset.push(Direction::Down),
                'B' => ruleset.push(Direction::Up),
                _ => {}
            }
        }

        println!("{:?}", ruleset);

        ruleset
    }

    pub fn run_ant_sim(app: &mut App) {
        for ant in app.ant_sim.ants.iter_mut() {
            Self::ant_turn(
                ant,
                &app.ant_sim.grid,
                &app.ant_sim.states,
                &app.ant_sim.rules,
            );
            Self::ant_forward(ant, &app.ant_sim.grid);
            Self::ant_flip(
                ant,
                &mut app.ant_sim.grid,
                &app.ant_sim.states,
                &app.ant_sim.rules,
            );
        }

        app.ant_sim.generation += 1;
    }

    pub fn ant_forward(ant: &mut Ant, grid: &Grid) {
        match ant.direction {
            // Wrap around the grid
            Direction::Left => {
                ant.x = if ant.x > 0.0 {
                    ant.x - 1.0
                } else {
                    grid.cells[0].len() as f64 - 1.0
                };
            }
            Direction::Right => {
                ant.x = if ant.x < (grid.cells[0].len() - 1) as f64 {
                    ant.x + 1.0
                } else {
                    0.0
                };
            }
            Direction::Up => {
                ant.y = if ant.y > 0.0 {
                    ant.y - 1.0
                } else {
                    grid.cells.len() as f64 - 1.0
                };
            }
            Direction::Down => {
                ant.y = if ant.y < (grid.cells.len() - 1) as f64 {
                    ant.y + 1.0
                } else {
                    0.0
                };
            }
        }
    }

    pub fn ant_turn(ant: &mut Ant, grid: &Grid, states: &Vec<Color>, rules: &Vec<Direction>) {
        for (state, rule) in states.iter().zip(rules.iter()) {
            if grid.cells[ant.y as usize][ant.x as usize] == *state {
                match rule {
                    Direction::Left => {
                        ant.direction = match ant.direction {
                            Direction::Left => Direction::Down,
                            Direction::Right => Direction::Up,
                            Direction::Up => Direction::Left,
                            Direction::Down => Direction::Right,
                        };
                    }
                    Direction::Right => {
                        ant.direction = match ant.direction {
                            Direction::Left => Direction::Up,
                            Direction::Right => Direction::Down,
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                        };
                    }
                    Direction::Down => {
                        ant.direction = match ant.direction {
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

    pub fn ant_flip(ant: &Ant, grid: &mut Grid, states: &Vec<Color>, rules: &Vec<Direction>) {
        let states = states.clone();
        let rules_len = rules.len();
        let mut states = states[0..rules_len].iter().cycle();

        // Assign the next state to the current cell
        while let Some(state) = states.next() {
            if grid.cells[ant.y as usize][ant.x as usize] == *state {
                grid.cells[ant.y as usize][ant.x as usize] = states.next().unwrap().clone();
                break;
            }
        }
    }
}
