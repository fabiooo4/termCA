use ratatui::style::Color;

use super::Grid;

pub struct ElementarySim {
    pub grid: Grid,              // Grid of cells
    pub current_line: Vec<bool>, // Grid of cells
    pub neighbours: usize,
    pub alive_state: Color,
    pub dead_state: Color,
    pub generation: usize, // Number of generations
    pub rule: u8,          // Rules for the ant
}

impl Default for ElementarySim {
    fn default() -> Self {
        Self {
            grid: Grid::new(),
            current_line: Vec::new(),
            neighbours: 3,
            alive_state: Color::White,
            dead_state: Color::Reset,
            generation: 0,
            rule: 22,
        }
    }
}

impl ElementarySim {
    pub fn run(&mut self, speed_multiplier: usize) {
        for _ in 0..speed_multiplier {
            if self.generation == 0 {
                self.grid.cells[0] = self
                    .current_line
                    .iter()
                    .map(|&b| if b { self.alive_state } else { self.dead_state })
                    .collect();
            } else {
                // Scroll the grid upwards
                self.grid.cells.pop();
                self.grid
                    .cells
                    .insert(0, vec![self.dead_state; self.grid.width()]);

                // Iterate over every window of neighbours
                for (center_idx, neighbours) in
                    self.current_line.windows(self.neighbours).enumerate()
                {
                    // Get the index of the rule corresponding to the slice of bools
                    let rule_idx = bin_to_idx(neighbours);

                    let center_idx = center_idx + self.neighbours / 2;

                    // Get the nth bit of the rule
                    let rule = get_bit(self.rule as u32, rule_idx);

                    match rule {
                        true => self.grid.cells[0][center_idx] = self.alive_state,
                        false => self.grid.cells[0][center_idx] = self.dead_state,
                    }
                }
            }

            // Update the line with the next generation
            for (i, c) in self.grid.cells[0].iter().enumerate() {
                self.current_line[i] = *c == self.alive_state
            }

            self.generation = self.generation.saturating_add(1);
        }
    }
}

/// Converts a slice of booleans into integer
/// # Example
/// ```rust
/// let bits: [bool; 3] = [true, false, true];
/// assert_eq!(bin_to_idx(&bits), 5);
/// ```
pub fn bin_to_idx(slice: &[bool]) -> usize {
    let mut rule_idx: usize = 0;
    slice.iter().rev().enumerate().for_each(|(i, b)| {
        rule_idx += (*b as usize) << (i * *b as usize);
    });
    rule_idx
}

/// Gets the nth bit of a positive integer
pub fn get_bit(num: u32, idx: usize) -> bool {
    ((num >> idx) & 1) != 0
}

#[cfg(test)]
#[test]
fn idx_seven() {
    let bits: [bool; 3] = [true, true, true];
    assert_eq!(bin_to_idx(&bits), 7);

    // Check if the third bit of 7 is 1
    assert!(get_bit(7_u32, 2));
}

#[test]
fn idx_zero() {
    let bits: [bool; 3] = [false, false, false];
    assert_eq!(bin_to_idx(&bits), 0);

    assert!(!get_bit(0_u32, 2));
}

#[test]
fn idx_one() {
    let bits: [bool; 3] = [false, false, true];
    assert_eq!(bin_to_idx(&bits), 1);

    assert!(get_bit(1_u32, 0));
}

#[test]
fn idx_four() {
    let bits: [bool; 3] = [true, false, false];
    assert_eq!(bin_to_idx(&bits), 4);

    assert!(get_bit(4_u32, 2));
}

#[test]
fn idx_two() {
    let bits: [bool; 3] = [false, true, false];
    assert_eq!(bin_to_idx(&bits), 2);

    assert!(get_bit(2_u32, 1));
}

#[test]
fn idx_five() {
    let bits: [bool; 3] = [true, false, true];
    assert_eq!(bin_to_idx(&bits), 5);

    assert!(!get_bit(5_u32, 1));
}
