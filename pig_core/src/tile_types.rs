use crate::TileIndex;

pub const BOTTOM_LEFT_CORNER: TileIndex = 0;
pub const BOTTOM_RIGHT_CORNER: TileIndex = 1;
pub const TOP_LEFT_CORNER: TileIndex = 2;
pub const TOP_RIGHT_CORNER: TileIndex = 3;
pub const LEFT_WALL: TileIndex = 4;
pub const RIGHT_WALL: TileIndex = 5;
pub const TOP_WALL: TileIndex = 6;
pub const BOTTOM_WALL: TileIndex = 7;
pub const EMPTY: TileIndex = 8;
pub const EXIT: TileIndex = 9;
pub const NONE: TileIndex = 10;

pub const ALL_TYPES: [TileIndex; 10] = [
    BOTTOM_LEFT_CORNER,
    BOTTOM_RIGHT_CORNER,
    TOP_LEFT_CORNER,
    TOP_RIGHT_CORNER,
    LEFT_WALL,
    RIGHT_WALL,
    TOP_WALL,
    BOTTOM_WALL,
    EMPTY,
    EXIT,
];