use crate::{TileIndex, coord::Coord, statics, cell::Cell, EMPTY};
use statics::GRID;

pub struct TileInfo {
    tile_idx: TileIndex,
    world_space_position: Coord 
}



pub fn pig_generate(radius: u32, starting_pos: Coord) -> Vec<TileInfo> {
    let ret = Vec::new();

    let mut grid = GRID.lock().unwrap();

    if grid.cell_at(&starting_pos).is_none() {
        grid.add_tile(Cell::with_tile_idx(starting_pos, EMPTY)).unwrap();
    }

    let current_tile = grid.cell_at(&starting_pos);

    'propogate: loop {

    }

    ret
}