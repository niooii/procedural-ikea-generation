use crate::{coord::Coord, TileIndex, NONE};

pub struct Cell {
    location: Coord,
    tile_idx: TileIndex
}

impl Cell {
    pub fn new(location: Coord) -> Self {
        Self {
            location,
            tile_idx: NONE
        }
    }
    pub fn with_tile_idx(location: Coord, tile_idx: TileIndex) -> Self {
        Self {
            location,
            tile_idx
        }
    }
    pub fn coord(&self) -> &Coord {
        &self.location
    }
    pub fn tile_index(&self) -> TileIndex {
        self.tile_idx
    }
}