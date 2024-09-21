use ratatui::style::Color;

use crate::app::App;

pub struct AntSim {
    pub ant: Ant,            // Langton's Ant
    pub rules_input: String, // Ant ruleset
    pub ant_grid: Grid,      // Grid of cells
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

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct Grid {
    pub cells: Vec<Vec<Color>>,
}

impl AntSim {
    pub fn parse_ant_ruleset(app: &mut App) {
        app.ant_sim.rules.clear();
        for c in app.ant_sim.rules_input.chars() {
            match c {
                'L' => app.ant_sim.rules.push(Direction::Left),
                'R' => app.ant_sim.rules.push(Direction::Right),
                'F' => app.ant_sim.rules.push(Direction::Up),
                'B' => app.ant_sim.rules.push(Direction::Down),
                _ => {}
            }
        }
    }

    pub fn run_ant_sim(app: &mut App) {
        Self::ant_forward(&mut app.ant_sim.ant, &app.ant_sim.ant_grid);
        Self::ant_turn(
            &mut app.ant_sim.ant,
            &app.ant_sim.ant_grid,
            &app.ant_sim.states,
            &app.ant_sim.rules,
        );
        Self::ant_flip(
            &app.ant_sim.ant,
            &mut app.ant_sim.ant_grid,
            &app.ant_sim.states,
        );

        app.ant_sim.generation += 1;
    }

    pub fn ant_forward(ant: &mut Ant, grid: &Grid) {
        match ant.direction {
            Direction::Left => {
                if ant.x > 0.0 {
                    ant.x -= 1.0;
                }
            }
            Direction::Right => {
                if ant.x < (grid.cells[0].len() - 1) as f64 {
                    ant.x += 1.0;
                }
            }
            Direction::Up => {
                if ant.y > 0.0 {
                    ant.y -= 1.0;
                }
            }
            Direction::Down => {
                if ant.y < (grid.cells.len() - 1) as f64 {
                    ant.y += 1.0;
                }
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

    pub fn ant_flip(ant: &Ant, grid: &mut Grid, states: &Vec<Color>) {
        let states = states.clone();
        let mut states = states.iter().cycle();

        // Assign the next state to the current cell
        while let Some(state) = states.next() {
            if grid.cells[ant.y as usize][ant.x as usize] == *state {
                grid.cells[ant.y as usize][ant.x as usize] = states.next().unwrap().clone();
                break;
            }
        }
    }
}
