// not really a grid. 

use crate::{tile::Tile, Direction, coord::Coord};

pub struct PhantomGrid {
    tiles: Vec<Tile>
}

impl PhantomGrid {
    pub fn new() -> Self {
        Self {
            tiles: Vec::new()
        }
    }

    pub fn tile_at(&self, tile_coord: Coord) -> Option<&Tile> {
        
    }

    pub fn tile_at_mut(&mut self, tile_coord: Coord) -> Option<&mut Tile> {
        
    }

    pub fn adjacent(&self, tile_coord: Coord, dir: Direction) -> Option<&Tile> {
        
    }

    pub fn adjacent_mut(&mut self, tile_coord: Coord, dir: Direction) -> Option<&mut Tile> {
        
    }
}