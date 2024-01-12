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
pub mod tile;
pub mod statics; 
pub mod coord;
pub mod adjacency_rule;