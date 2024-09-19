use ratatui::{style::Color, symbols::Marker};

pub enum CurrentScreen {
    Main,
    Ant,
    Exit,
}

pub enum Direction {
    Left,
    Right,
}

pub enum CellState {
    Alive,
    Dead,
}

pub struct Ant {
    pub x: f64,
    pub y: f64,
    pub color: Color,
    pub direction: Direction,
}

pub struct Grid {
    pub cells: Vec<Vec<CellState>>,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub ant: Ant,
    pub ant_grid: Grid,
    pub marker: Marker,
}

impl App {
    pub fn new() -> Self {
        App {
            current_screen: CurrentScreen::Ant,
            ant: Ant {
                x: 0.0,
                y: 0.0,
                color: Color::Green,
                direction: Direction::Right,
            },
            ant_grid: Grid {
                cells: Vec::new()
            },
            marker: Marker::HalfBlock,
        }
    }
}
