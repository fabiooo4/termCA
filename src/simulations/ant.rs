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
        Self::ant_forward(app);
        Self::ant_turn(app);
        Self::ant_flip(app);
        app.ant_sim.generation += 1;
    }

    pub fn ant_forward(app: &mut App) {
        match app.ant_sim.ant.direction {
            Direction::Left => {
                if app.ant_sim.ant.x > 0.0 {
                    app.ant_sim.ant.x -= 1.0;
                }
            }
            Direction::Right => {
                if app.ant_sim.ant.x < (app.ant_sim.ant_grid.cells[0].len() - 1) as f64 {
                    app.ant_sim.ant.x += 1.0;
                }
            }
            Direction::Up => {
                if app.ant_sim.ant.y > 0.0 {
                    app.ant_sim.ant.y -= 1.0;
                }
            }
            Direction::Down => {
                if app.ant_sim.ant.y < (app.ant_sim.ant_grid.cells.len() - 1) as f64 {
                    app.ant_sim.ant.y += 1.0;
                }
            }
        }
    }

    pub fn ant_turn(app: &mut App) {
        for (state, rule) in app.ant_sim.states.iter().zip(app.ant_sim.rules.iter()) {
            if app.ant_sim.ant_grid.cells[app.ant_sim.ant.y as usize][app.ant_sim.ant.x as usize]
                == *state
            {
                match rule {
                    Direction::Left => {
                        app.ant_sim.ant.direction = match app.ant_sim.ant.direction {
                            Direction::Left => Direction::Down,
                            Direction::Right => Direction::Up,
                            Direction::Up => Direction::Left,
                            Direction::Down => Direction::Right,
                        };
                    }
                    Direction::Right => {
                        app.ant_sim.ant.direction = match app.ant_sim.ant.direction {
                            Direction::Left => Direction::Up,
                            Direction::Right => Direction::Down,
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                        };
                    }
                    Direction::Down => {
                        app.ant_sim.ant.direction = match app.ant_sim.ant.direction {
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

    pub fn ant_flip(app: &mut App) {
        let states = app.ant_sim.states.clone();
        let mut states = states.iter().cycle();

        // Assign the next state to the current cell
        while let Some(state) = states.next() {
            if app.ant_sim.ant_grid.cells[app.ant_sim.ant.y as usize][app.ant_sim.ant.x as usize]
                == *state
            {
                app.ant_sim.ant_grid.cells[app.ant_sim.ant.y as usize]
                    [app.ant_sim.ant.x as usize] = states.next().unwrap().clone();
                break;
            }
        }
    }
}
