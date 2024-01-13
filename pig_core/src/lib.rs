// TYPES
pub type TileIndex = u8;
pub static BOTTOM_LEFT_CORNER: TileIndex = 0;
pub static BOTTOM_RIGHT_CORNER: TileIndex = 1;
pub static TOP_LEFT_CORNER: TileIndex = 2;
pub static TOP_RIGHT_CORNER: TileIndex = 3;
pub static LEFT_WALL: TileIndex = 4;
pub static RIGHT_WALL: TileIndex = 5;
pub static UP_WALL: TileIndex = 6;
pub static DOWN_WALL: TileIndex = 7;
pub static EMPTY: TileIndex = 8;
pub static NONE: TileIndex = 9;

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