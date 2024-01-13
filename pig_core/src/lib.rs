// TYPES
pub type TileIndex = u8;

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
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