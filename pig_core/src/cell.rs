use crate::{coord::Coord, TileIndex, tile_types::*};

#[derive(Debug)]
pub struct Cell {
    location: Coord,
    tile_idx: TileIndex,
    possible_tiles: Vec<bool>,
    is_collapsed: bool,
}

impl Cell {
    pub fn new(location: Coord, num_possible_tiles: u32) -> Self {
        Self {
            location,
            tile_idx: NONE,
            possible_tiles: vec![true; num_possible_tiles as usize],
            is_collapsed: false
        }
    }
    pub fn with_tile_idx(location: Coord, tile_idx: TileIndex) -> Self {
        Self {
            location,
            tile_idx,
            possible_tiles: Vec::new(),
            is_collapsed: true
        }
    }
    pub fn coord(&self) -> &Coord {
        &self.location
    }
    pub fn tile_index(&self) -> TileIndex {
        self.tile_idx
    }
    pub fn collapse(&mut self) {
        todo!();
    }

}