use crate::app::InputMode;
use ratatui::{
    style::Color,
    symbols::Marker, widgets::ScrollbarState,
};
use tui_input::Input;
use tui_widget_list::ListState;

use super::{Direction, Grid};

/// Struct that holds the ant simulation data
pub struct AntSim {
    pub marker: Marker,        // Character to draw the cells
    pub ants: Vec<Ant>,        // Vector that holds the ants
    pub grid: Grid,            // Grid of cells
    pub states: Vec<Color>,    // Possible states of the cells
    pub rules: Vec<Direction>, // Rules for the ant
    pub generation: usize,     // Number of generations

    // Edit state
    pub settings_state: ListState,
    pub preset_state: ListState,
    pub ants_list_state: ListState,

    pub preset_scroll_state: ScrollbarState,

    pub rules_input: Input,          // Input widget
    pub rules_input_mode: InputMode, // Input mode
}

impl Default for AntSim {
    fn default() -> Self {
        let mut list_state = ListState::default();
        list_state.selected = Some(0);
        AntSim {
            ants: vec![Ant::default()],
            grid: Grid::new(),
            states: vec![
                Color::Reset,
                Color::Indexed(3),
                Color::Indexed(1),
                Color::Indexed(2),
                Color::Indexed(4),
                Color::Indexed(5),
                Color::Indexed(6),
                Color::Indexed(9),
                Color::Indexed(10),
                Color::Indexed(11),
                Color::Indexed(12),
                Color::Indexed(13),
                Color::Indexed(14),
                Color::Indexed(7),
                Color::Indexed(8),
                Color::Indexed(15),
                Color::Indexed(17),
            ],
            rules: vec![Direction::Right, Direction::Left],
            generation: 0,
            marker: Marker::HalfBlock,

            settings_state: list_state.clone(),
            preset_state: list_state.clone(),
            ants_list_state: list_state,

            preset_scroll_state: ScrollbarState::default(),

            rules_input: Input::from(String::from("RL")),
            rules_input_mode: InputMode::Normal,
        }
    }
}

/// Struct that holds the ant data
#[derive(Clone, Copy)]
pub struct Ant {
    pub x: usize,
    pub y: usize,
    pub color: Color,
    pub direction: Direction,
}

impl Default for Ant {
    /// Constructs a new empty `Ant`
    fn default() -> Self {
        Ant {
            // Set to invalid position to reposition in the center of the screen when the
            // frame is available
            x: usize::MAX,
            y: usize::MAX,
            color: Color::Indexed(16),
            direction: Direction::Up,
        }
    }
}

impl Ant {
    /// Move the ant in the specified direction with grid wrapping
    pub fn change_position(&mut self, direction: Direction, grid: &Grid) {
        match direction {
            Direction::Left => {
                self.x = if self.x > 0 {
                    self.x - 1
                } else {
                    grid.width() - 1
                };
            }
            Direction::Right => {
                self.x = if self.x < (grid.width() - 1) {
                    self.x + 1
                } else {
                    0
                };
            }
            Direction::Up => {
                self.y = if self.y < (grid.height() - 1) {
                    self.y + 1
                } else {
                    0
                };
            }
            Direction::Down => {
                self.y = if self.y > 0 {
                    self.y - 1
                } else {
                    grid.height() - 1
                };
            }
        }
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
    pub fn run(&mut self, speed_multiplier: usize) {
        for _ in 0..speed_multiplier {
            for ant in self.ants.iter_mut() {
                Self::ant_turn(ant, &self.grid, &self.states, &self.rules);
                Self::ant_flip(ant, &mut self.grid, &self.states, &self.rules);
                Self::ant_forward(ant, &self.grid);
            }
        }
        self.generation = self.generation.saturating_add(speed_multiplier);
    }

    /// Moves the ant forward based on its direction with grid wrapping
    pub fn ant_forward(ant: &mut Ant, grid: &Grid) {
        ant.change_position(ant.direction, grid);
    }

    /// Turns the ant based on the current cell state and rule
    pub fn ant_turn(ant: &mut Ant, grid: &Grid, states: &[Color], rules: &[Direction]) {
        for (state, rule) in states.iter().zip(rules.iter()) {
            if grid.cells[ant.y][ant.x] == *state {
                ant.direction = ant.direction.turn(rule);
                break;
            }
        }
    }

    /// Flips the current cell state based on the rule
    pub fn ant_flip(ant: &Ant, grid: &mut Grid, states: &[Color], rules: &[Direction]) {
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

pub enum AntSettings {
    Presets,
    Ruleset,
    Ants,
    Start,
}

impl AntSettings {
    pub const COUNT: usize = 4;
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => AntSettings::Presets,
            1 => AntSettings::Ruleset,
            2 => AntSettings::Ants,
            3 => AntSettings::Start,
            _ => AntSettings::Ruleset,
        }
    }
}

impl std::fmt::Display for AntSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AntSettings::Presets => write!(f, "Presets"),
            AntSettings::Ruleset => write!(f, "Ruleset"),
            AntSettings::Ants => write!(f, "Ants"),
            AntSettings::Start => write!(f, "Start"),
        }
    }
}

pub enum AntPresets {
    Default,

    FractalLabyrinth,
    CosmicBloom,
    Starburst,
    InfinitePrism,
    IntersectingTriangles,
    SquareEscape,
    SoaringSerpent,
    ConvergingChaos,
    DiamondDance,
    ZigzaggingZones,
    JaggedJourney,
    TiltedTiles,
    GalaxyNeedle,
    Highway,
    PatternedHighway,
    SquaresHighway,
    TriangleHighway,
    TilesHighway,
    StripedHighway,
    BoxHighway,
    ChaosHighway,
    OrderedHighways,
    MosaicMaze,
    TessellationTrail,
    Spiral,
    ChaoticSpiral,
    RandomSpirals,
    SpiralHighway,
    SymmetricBloom,
    ExpandingTriangle,
    SwirlingBlades,
    Island,
    Cross,
    InfiniteSaw,
    DomainExpansion,
    TriangleDunes,
    Butterfly,
    Moth,
    Mountains,
    Laser,
    Stairs,
    Pyramid,
    DoubleCircle,
    TriangleFractal,
    TriangleFractal2,
    Spikes,
    Sword,
    Key,
    SpiralingBlocks,
    ChaosTriangles,
    Shuriken,
    ExpandingBoxes,
    PyramidBox,
    RandomSquarePaths,
    BouncingPath,
    FillingBoxes,
    RandomAreas,
    InfiniteTriangle,
    TriangleSpiral,
    ExplodingStar,
    PyramidExplosion,
    Spider,
    Towers,
    DitheringFill,
    SwirlingPath,
    SpiralFilling,
    Thorns,
    Nest,
    ZigzagPath,
    ChaosPrison,
    ExpandingCage,
    Caterpillar,
}

impl AntPresets {
    pub const COUNT: usize = 73;
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => AntPresets::Default,
            1 => AntPresets::FractalLabyrinth,
            2 => AntPresets::CosmicBloom,
            3 => AntPresets::Starburst,
            4 => AntPresets::InfinitePrism,
            5 => AntPresets::IntersectingTriangles,
            6 => AntPresets::SquareEscape,
            7 => AntPresets::SoaringSerpent,
            8 => AntPresets::ConvergingChaos,
            9 => AntPresets::DiamondDance,
            10 => AntPresets::ZigzaggingZones,
            11 => AntPresets::JaggedJourney,
            12 => AntPresets::TiltedTiles,
            13 => AntPresets::GalaxyNeedle,
            14 => AntPresets::Highway,
            15 => AntPresets::PatternedHighway,
            16 => AntPresets::SquaresHighway,
            17 => AntPresets::TriangleHighway,
            18 => AntPresets::TilesHighway,
            19 => AntPresets::StripedHighway,
            20 => AntPresets::BoxHighway,
            21 => AntPresets::ChaosHighway,
            22 => AntPresets::OrderedHighways,
            23 => AntPresets::MosaicMaze,
            24 => AntPresets::TessellationTrail,
            25 => AntPresets::Spiral,
            26 => AntPresets::ChaoticSpiral,
            27 => AntPresets::RandomSpirals,
            28 => AntPresets::SpiralHighway,
            29 => AntPresets::SymmetricBloom,
            30 => AntPresets::ExpandingTriangle,
            31 => AntPresets::SwirlingBlades,
            32 => AntPresets::Island,
            33 => AntPresets::Cross,
            34 => AntPresets::InfiniteSaw,
            35 => AntPresets::DomainExpansion,
            36 => AntPresets::TriangleDunes,
            37 => AntPresets::Butterfly,
            38 => AntPresets::Moth,
            39 => AntPresets::Mountains,
            40 => AntPresets::Laser,
            41 => AntPresets::Stairs,
            42 => AntPresets::Pyramid,
            43 => AntPresets::DoubleCircle,
            44 => AntPresets::TriangleFractal,
            45 => AntPresets::TriangleFractal2,
            46 => AntPresets::Spikes,
            47 => AntPresets::Sword,
            48 => AntPresets::Key,
            49 => AntPresets::SpiralingBlocks,
            50 => AntPresets::ChaosTriangles,
            51 => AntPresets::Shuriken,
            52 => AntPresets::ExpandingBoxes,
            53 => AntPresets::PyramidBox,
            54 => AntPresets::RandomSquarePaths,
            55 => AntPresets::BouncingPath,
            56 => AntPresets::FillingBoxes,
            57 => AntPresets::RandomAreas,
            58 => AntPresets::InfiniteTriangle,
            59 => AntPresets::TriangleSpiral,
            60 => AntPresets::ExplodingStar,
            61 => AntPresets::PyramidExplosion,
            62 => AntPresets::Spider,
            63 => AntPresets::Towers,
            64 => AntPresets::DitheringFill,
            65 => AntPresets::SwirlingPath,
            66 => AntPresets::SpiralFilling,
            67 => AntPresets::Thorns,
            68 => AntPresets::Nest,
            69 => AntPresets::ZigzagPath,
            70 => AntPresets::ChaosPrison,
            71 => AntPresets::ExpandingCage,
            72 => AntPresets::Caterpillar,
            _ => AntPresets::Default,
        }
    }
}

impl std::fmt::Display for AntPresets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AntPresets::Default => write!(f, "Default"),
            AntPresets::FractalLabyrinth => write!(f, "Fractal Labyrinth"),
            AntPresets::CosmicBloom => write!(f, "Cosmic Bloom"),
            AntPresets::Starburst => write!(f, "Starburst"),
            AntPresets::InfinitePrism => write!(f, "Infinite Prism"),
            AntPresets::IntersectingTriangles => write!(f, "Intersecting Triangles"),
            AntPresets::SquareEscape => write!(f, "Square Escape"),
            AntPresets::SoaringSerpent => write!(f, "Soaring Serpent"),
            AntPresets::ConvergingChaos => write!(f, "Converging Chaos"),
            AntPresets::DiamondDance => write!(f, "Diamond Dance"),
            AntPresets::ZigzaggingZones => write!(f, "Zigzagging Zones"),
            AntPresets::JaggedJourney => write!(f, "Jagged Journey"),
            AntPresets::TiltedTiles => write!(f, "Tilted Tiles"),
            AntPresets::GalaxyNeedle => write!(f, "Galaxy Needle"),
            AntPresets::Highway => write!(f, "Highway"),
            AntPresets::PatternedHighway => write!(f, "Patterned Highway"),
            AntPresets::SquaresHighway => write!(f, "Squares Highway"),
            AntPresets::TriangleHighway => write!(f, "Triangle Highway"),
            AntPresets::TilesHighway => write!(f, "Tiles Highway"),
            AntPresets::StripedHighway => write!(f, "Striped Highway"),
            AntPresets::BoxHighway => write!(f, "Box Highway"),
            AntPresets::ChaosHighway => write!(f, "Chaos Highway"),
            AntPresets::OrderedHighways => write!(f, "Ordered Highways"),
            AntPresets::MosaicMaze => write!(f, "Mosaic Maze"),
            AntPresets::TessellationTrail => write!(f, "Tessellation Trail"),
            AntPresets::Spiral => write!(f, "Spiral"),
            AntPresets::ChaoticSpiral => write!(f, "Chaotic Spiral"),
            AntPresets::RandomSpirals => write!(f, "Random Spirals"),
            AntPresets::SpiralHighway => write!(f, "Spiral Highway"),
            AntPresets::SymmetricBloom => write!(f, "Symmetric Bloom"),
            AntPresets::ExpandingTriangle => write!(f, "Expanding Triangle"),
            AntPresets::SwirlingBlades => write!(f, "Swirling Blades"),
            AntPresets::Island => write!(f, "Island"),
            AntPresets::Cross => write!(f, "Cross"),
            AntPresets::InfiniteSaw => write!(f, "Infinite Saw"),
            AntPresets::DomainExpansion => write!(f, "Domain Expansion"),
            AntPresets::TriangleDunes => write!(f, "Triangle Dunes"),
            AntPresets::Butterfly => write!(f, "Butterfly"),
            AntPresets::Moth => write!(f, "Moth"),
            AntPresets::Mountains => write!(f, "Mountains"),
            AntPresets::Laser => write!(f, "Laser"),
            AntPresets::Stairs => write!(f, "Stairs"),
            AntPresets::Pyramid => write!(f, "Pyramid"),
            AntPresets::DoubleCircle => write!(f, "Double Circle"),
            AntPresets::TriangleFractal => write!(f, "Triangle Fractal"),
            AntPresets::TriangleFractal2 => write!(f, "Triangle Fractal 2"),
            AntPresets::Spikes => write!(f, "Spikes"),
            AntPresets::Sword => write!(f, "Sword"),
            AntPresets::Key => write!(f, "Key"),
            AntPresets::SpiralingBlocks => write!(f, "Spiraling Blocks"),
            AntPresets::ChaosTriangles => write!(f, "Chaos Triangles"),
            AntPresets::Shuriken => write!(f, "Shuriken"),
            AntPresets::ExpandingBoxes => write!(f, "Expanding Boxes"),
            AntPresets::PyramidBox => write!(f, "Pyramid in a Box"),
            AntPresets::RandomSquarePaths => write!(f, "Random Square Paths"),
            AntPresets::BouncingPath => write!(f, "Bouncing Path"),
            AntPresets::FillingBoxes => write!(f, "Filling Boxes"),
            AntPresets::RandomAreas => write!(f, "Random Areas"),
            AntPresets::InfiniteTriangle => write!(f, "Infinite Triangle"),
            AntPresets::TriangleSpiral => write!(f, "Triangle Spiral"),
            AntPresets::ExplodingStar => write!(f, "Exploding Star"),
            AntPresets::PyramidExplosion => write!(f, "Pyramid Explosion"),
            AntPresets::Spider => write!(f, "Spider"),
            AntPresets::Towers => write!(f, "Towers"),
            AntPresets::DitheringFill => write!(f, "Dithering Fill"),
            AntPresets::SwirlingPath => write!(f, "Swirling Path"),
            AntPresets::SpiralFilling => write!(f, "Spiral Filling"),
            AntPresets::Thorns => write!(f, "Thorns"),
            AntPresets::Nest => write!(f, "Nest"),
            AntPresets::ZigzagPath => write!(f, "Zigzag Path"),
            AntPresets::ChaosPrison => write!(f, "Chaos Prison"),
            AntPresets::ExpandingCage => write!(f, "Expanding Cage"),
            AntPresets::Caterpillar => write!(f, "Caterpillar"),
        }
    }
}

impl AntSim {
    pub fn get_preset(preset: AntPresets) -> Self {
        let ruleset = match preset {
            AntPresets::Default => "RL",
            AntPresets::FractalLabyrinth => "LRRRRLLLRLLLLLLL",
            AntPresets::CosmicBloom => "LRRLLLLRLLRRLLLL",
            AntPresets::Starburst => "LRLLLRRRRRLRLL",
            AntPresets::InfinitePrism => "LLLRLLRRRLLLLL",
            AntPresets::IntersectingTriangles => "LLRLLLRRRRRLLRLL",
            AntPresets::SquareEscape => "LRRRLLLLLLRRLL",
            AntPresets::SoaringSerpent => "LLLLLLLRRLLRLLL",
            AntPresets::ConvergingChaos => "LLLLRLLLRRLRLLL",
            AntPresets::DiamondDance => "LRRLLLRRRLLLL",
            AntPresets::ZigzaggingZones => "LRRLLLRRRLLLLL",
            AntPresets::JaggedJourney => "LRRLLLRRRLLLLLL",
            AntPresets::TiltedTiles => "LRRLLLRRRLLLLLLL",
            AntPresets::GalaxyNeedle => "LRRLRLLLLLRRLL",
            AntPresets::Highway => "LLRLRLLLRLLLLL",
            AntPresets::PatternedHighway => "RRLRLLRRRRLL",
            AntPresets::SquaresHighway => "LRRRRRLRRRRLLL",
            AntPresets::TriangleHighway => "LLRLRRLRLLRRLLL",
            AntPresets::TilesHighway => "LRRRRRLRRRRLLLL",
            AntPresets::StripedHighway => "LLRLRRLRLRRLLLLL",
            AntPresets::BoxHighway => "RRRLRRLLRLRRRLL",
            AntPresets::ChaosHighway => "LLRLRRLLRLLLLLL",
            AntPresets::OrderedHighways => "RRLLRLLLLLLLRRL",
            AntPresets::MosaicMaze => "LRRRRRLRRRRLLLLL",
            AntPresets::TessellationTrail => "LRRRRRLRRRRLLLLLL",
            AntPresets::Spiral => "RLLLLRRRLLL",
            AntPresets::ChaoticSpiral => "RRLLLLRLRLRRLLL",
            AntPresets::RandomSpirals => "RLLLLLRRRLLLLRRL",
            AntPresets::SpiralHighway => "LRLRRLLLLLLLRL",
            AntPresets::SymmetricBloom => "LLRRLLRRRRLLRRLL",
            AntPresets::ExpandingTriangle => "LLRRRLRRRLLLLRRL",
            AntPresets::SwirlingBlades => "LLRRRLLLRRRLLLLL",
            AntPresets::Island => "LLRLRRLRLRRRRRRL",
            AntPresets::Cross => "LRLRLLRLLLRRLLLL",
            AntPresets::InfiniteSaw => "LLRRRLRRRLRLLLL",
            AntPresets::DomainExpansion => "LRRLLLRLRRRRRRL",
            AntPresets::TriangleDunes => "LLRLRRLLLLLRLRL",
            AntPresets::Butterfly => "LLLRRRRRRLLL",
            AntPresets::Moth => "LRRLLRRL",
            AntPresets::Mountains => "RRLRLLLLRLLLLLLL",
            AntPresets::Laser => "RRLLLRRLRRRRLLLL",
            AntPresets::Stairs => "RRLRLLLLRLLRLLL",
            AntPresets::Pyramid => "RRRLRRRRLLRRRRLL",
            AntPresets::DoubleCircle => "RRRRRRRRLLLLLLLL",
            AntPresets::TriangleFractal => "LLLLLRLLLRRLLLLL",
            AntPresets::TriangleFractal2 => "LRRRLLLRLLLLLLL",
            AntPresets::Spikes => "LLRLRLRRRRRRL",
            AntPresets::Sword => "LLRLRLLLLRLRLRRL",
            AntPresets::Key => "RRLRLLLRLLRRRRLL",
            AntPresets::SpiralingBlocks => "LRRLLLRLRLRRLLL",
            AntPresets::ChaosTriangles => "LRRRRLRRLLRRRRRL",
            AntPresets::Shuriken => "LLLLRRRRRLLRRRRL",
            AntPresets::ExpandingBoxes => "RRLRLLLRRRRRLL",
            AntPresets::PyramidBox => "RRLLLLRRLRLLLLLL",
            AntPresets::RandomSquarePaths => "RLLLLLLLRRL",
            AntPresets::BouncingPath => "RRLLLLRRLRLRLLL",
            AntPresets::FillingBoxes => "LRRLLRRLLLRRRLLL",
            AntPresets::RandomAreas => "RRLLLLRLLLLRRLLL",
            AntPresets::InfiniteTriangle => "RRLLLRRRRRLRLLL",
            AntPresets::TriangleSpiral => "RRLLLLRRRLLLLLLL",
            AntPresets::ExplodingStar => "RRRRLRRRLLRRRLL",
            AntPresets::PyramidExplosion => "LLRLRRRLRRLLLL",
            AntPresets::Spider => "LRRLLLLRLLRLL",
            AntPresets::Towers => "RLLLRRRLLLL",
            AntPresets::DitheringFill => "RLLLRLRRRRRLRL",
            AntPresets::SwirlingPath => "RLLLLLLLLLRRL",
            AntPresets::SpiralFilling => "RLLLRLLLLRRLLLLL",
            AntPresets::Thorns => "LLLLLRRLLLLRLLL",
            AntPresets::Nest => "LRRLLLLRRLLRLLLL",
            AntPresets::ZigzagPath => "LRRLLLLRRRLLLRRL",
            AntPresets::ChaosPrison => "LRRLLLRRRRRRRL",
            AntPresets::ExpandingCage => "LRRRLLLLLLRRLLL",
            AntPresets::Caterpillar => "LRRRRLLLRRRLRL",
        };

        AntSim {
            rules: AntSim::parse_ant_ruleset(ruleset),
            rules_input: Input::default().with_value(ruleset.to_string()),
            ..Default::default()
        }
    }
}
