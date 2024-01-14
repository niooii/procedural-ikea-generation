// TYPES
pub type TileIndex = u8;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl Direction {
    pub fn negate(dir: Direction) -> Direction {
        match dir {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
        }
    }
}

pub mod core;
pub mod phantom_grid;
pub mod cell;
pub mod statics; 
pub mod coord;
pub mod adjacency_rules;
pub mod error;
pub mod tile_weights;
pub mod tile_types;