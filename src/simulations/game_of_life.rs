use ratatui::{style::Color, symbols::Marker};
use tui_widget_list::ListState;

use super::{ant::Ant, Direction, Grid};

pub struct GolSim {
    pub marker: Marker,        // Character to draw the cells
    pub grid: Grid, // Grid of cells
    pub alive_state: Color,
    pub dead_state: Color,
    pub generation: usize, // Number of generations

    pub settings_state: ListState,
    pub edit_cursor: Ant, // Reuse of the ant as a cursor
}

impl Default for GolSim {
    fn default() -> Self {
        let mut list_state = ListState::default();
        list_state.selected = Some(0);
        Self {
            marker: Marker::HalfBlock,
            grid: Grid::new(),
            alive_state: Color::Yellow,
            dead_state: Color::Reset,
            generation: Default::default(),

            settings_state: list_state,
            edit_cursor: Ant {
                x: usize::MAX,
                y: usize::MAX,
                color: Color::Red,
                direction: Direction::Up,
            },
        }
    }
}

impl GolSim {
    pub fn run(&mut self, speed_multiplier: usize) {
        for _ in 0..speed_multiplier {
            let grid = self.grid.clone();
            for y in 0..grid.height() {
                for x in 0..grid.width() {
                    let alive_count = count_alive_neighbours(&grid, x, y, self.alive_state);
                    let cell = &mut self.grid.cells[y][x];

                    // Game of life rules
                    if *cell == self.alive_state {
                        if alive_count < 2 {
                            *cell = self.dead_state;
                        } else if alive_count <= 3 {
                            *cell = self.alive_state;
                        } else if alive_count > 3 {
                            *cell = self.dead_state;
                        }
                    } else if alive_count == 3 {
                        *cell = self.alive_state;
                    }
                }
            }
        }
        self.generation = self.generation.saturating_add(speed_multiplier);
    }

    pub fn toggle_cell(&mut self, x: usize, y: usize) {
        let cell = &mut self.grid.cells[y][x];
        if *cell == self.alive_state {
            *cell = self.dead_state;
        } else {
            *cell = self.alive_state;
        }
    }
}

/// Counts the alive neighbours of a cell based on its position and the alive state
fn count_alive_neighbours(grid: &Grid, x: usize, y: usize, alive_state: Color) -> usize {
    let left = grid.cells[y][match x as i32 - 1 < 0 {
        true => grid.width() - 1,
        false => x - 1,
    }];
    let right = grid.cells[y][match x + 1 >= grid.width() {
        true => 0,
        false => x + 1,
    }];

    let bottom = grid.cells[match y as i32 - 1 < 0 {
        true => grid.height() - 1,
        false => y - 1,
    }][x];
    let top = grid.cells[match y + 1 >= grid.height() {
        true => 0,
        false => y + 1,
    }][x];

    let top_right = grid.cells[match y + 1 >= grid.height() {
        true => 0,
        false => y + 1,
    }][match x + 1 >= grid.width() {
        true => 0,
        false => x + 1,
    }];
    let top_left = grid.cells[match y + 1 >= grid.height() {
        true => 0,
        false => y + 1,
    }][match x as i32 - 1 < 0 {
        true => grid.width() - 1,
        false => x - 1,
    }];

    let bottom_right = grid.cells[match y as i32 - 1 < 0 {
        true => grid.height() - 1,
        false => y - 1,
    }][match x + 1 >= grid.width() {
        true => 0,
        false => x + 1,
    }];
    let bottom_left = grid.cells[match y as i32 - 1 < 0 {
        true => grid.height() - 1,
        false => y - 1,
    }][match x as i32 - 1 < 0 {
        true => grid.width() - 1,
        false => x - 1,
    }];

    let neighbours = [
        top_left,
        top,
        top_right,
        left,
        right,
        bottom_left,
        bottom,
        bottom_right,
    ];

    let mut count = 0;
    for cell in neighbours {
        if cell == alive_state {
            count += 1;
        }
    }

    count
}

pub enum GolSettings {
    EditGrid,
    Start,
}

impl GolSettings {
    pub const COUNT: usize = 2;
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => GolSettings::EditGrid,
            1 => GolSettings::Start,
            _ => GolSettings::EditGrid,
        }
    }
}

impl std::fmt::Display for GolSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GolSettings::EditGrid => write!(f, "Edit grid"),
            GolSettings::Start => write!(f, "Start"),
        }
    }
}
