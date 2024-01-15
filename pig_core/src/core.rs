use std::collections::HashSet;

use crate::{TileIndex, coord::Coord, statics, cell::Cell, tile_types::*, adjacency_rules::AdjacencyRules, tile_weights::TileWeights, Direction, error::PigError};
use serde::Serialize;
use statics::GRID;

use rand_chacha::ChaCha20Rng;
use rand_chacha::rand_core::SeedableRng;
use rand::Rng;

#[derive(Serialize)]
pub struct TileInfo {
    tile_idx: TileIndex,
    world_space_position: Coord 
}

#[derive(Serialize)]
pub struct TileInfoWrapper {
    data: Vec<TileInfo>
}

pub fn clear_grid() {
    let mut grid = GRID.lock().unwrap();

    grid.clear();
}

pub fn pig_generate_internal(iters: u32, starting_pos: Coord, adjacency_rules: &AdjacencyRules, tile_weights: &TileWeights, seed: u64, starting_search_radius: u32) -> Result<TileInfoWrapper, PigError> {
    let mut seeded_rng = ChaCha20Rng::seed_from_u64(seed as u64);

    let mut ret = Vec::new();

    let mut grid = GRID.lock().unwrap();

    // if cell doesn't exist, make it exist. 
    if grid.cell_at(&starting_pos).is_none() {
        grid.add_cell(Cell::with_tile_idx(starting_pos, EMPTY))?;
    }

    let mut SEARCH_RADIUS: u32 = starting_search_radius;

    let mut new_additions = Vec::<TileInfo>::new();

    // sometimes the initialized tiles overlap, but it should be following the same adjacency rules anyways so we
    // dont have to worry about this.
    let mut already_initialized: HashSet<Coord> = HashSet::new();

    for _ in 0..iters {
        SEARCH_RADIUS += 1;
        
        // for any existing cells within a radius of the player, that aren't surrounded by tiles.
        for cell in grid.radius_iter(SEARCH_RADIUS, starting_pos)
        .filter(|c| {
            if let Some(cell) = c {
                !grid.is_surrounded_by_cells(cell.coord())
            } else {
                false
            }
        })
        .map(|c| c.unwrap())
        {
            // process the empty cells surrounding it.
            for dir in grid.surrounding_nonexisting_cells(cell.coord()) {
                let original_coord = cell.coord();
                let target = original_coord.translated(dir);

                if already_initialized.contains(&target) {
                    continue;
                }

                // initial allowed tile states
                let mut allowed_tiles: HashSet<TileIndex> = [
                    BOTTOM_LEFT_CORNER,
                    BOTTOM_RIGHT_CORNER,
                    TOP_LEFT_CORNER,
                    TOP_RIGHT_CORNER,
                    LEFT_WALL,
                    RIGHT_WALL,
                    TOP_WALL,
                    BOTTOM_WALL,
                    EMPTY,
                    // NONE always allowed
                    // NONE,
                ].into();
                // filter the possible allowed tiles.
                // get each cell surrounding the cell to be collapsed. 
                // loop through the cells that exist, and check the adjacency rules for allowed indices.
                for (surrounding_target, in_dir) in grid.surrounding_existing_cells(&target) {
                    
                    let allowed_indices = adjacency_rules.allowed_tile_indices(surrounding_target.tile_index(), Direction::negate(in_dir));

                    allowed_tiles = allowed_tiles.intersection(&allowed_indices).copied().collect();
                }

                let chosen_tile = tile_weights.from_allowed_indices(allowed_tiles, &mut seeded_rng);
                
                // println!("finished cell {:?} in direction {:?} from original pos {:?}", target, dir, original_coord);
                already_initialized.insert(target);
                new_additions.push(TileInfo {
                    tile_idx: chosen_tile,
                    world_space_position: target
                })
            }
        }

        // to satisfy rust borrow checker, we process after each iter from the new_additions vec.
        for change in new_additions.iter() {
            grid.add_cell(Cell::with_tile_idx(change.world_space_position, change.tile_idx))?;
        }

       
        ret.append(&mut new_additions);
    }

    // grid.print_state(true);
    
    let wrapper = TileInfoWrapper { data: ret };
    

    Ok(wrapper)
}