use std::time::Instant;

// use wfc::{self, AdjacencyRules, FrequencyHints, TileIndex};
use pig_core::{self, phantom_grid::PhantomGrid, cell::Cell, error::Result, tile_weights::TileWeights, adjacency_rules::AdjacencyRules, tile_types::*, Direction::*, core::pig_generate};
use crate::pig_core::coord::Coord;
use pig_core::allow;

fn main() -> Result<()> {
    let before = Instant::now();
    let tile_weights = TileWeights::new(vec![
        1.0, // BOTTOM_LEFT_CORNER
        1.0, // BOTTOM_RIGHT_CORNER
        1.0, // TOP_LEFT_CORNER
        1.0, // TOP_RIGHT_CORNER
        1.0, // LEFT_WALL
        1.0, // RIGHT_WALL
        1.0, // UP_WALL
        1.0, // DOWN_WALL
        2.0, // EMPTY
        0.0, // NONE
    ]);
    
    let mut adjacency_rules = AdjacencyRules::default();
    /*
    @PARAMS
    from: first parameter - tile to go in in_direction from.
    in_direction: in direction
    to: allowed to be the type "to" in direction "in_direction" from "from".
     */
    
    

    pig_generate(200, Coord::new(0, 0), &adjacency_rules, &tile_weights);
    pig_generate(20, Coord::new(20, 0), &adjacency_rules, &tile_weights);
    pig_generate(20, Coord::new(40, 0), &adjacency_rules, &tile_weights);
    pig_generate(20, Coord::new(60, 0), &adjacency_rules, &tile_weights);
    pig_generate(20, Coord::new(80, 0), &adjacency_rules, &tile_weights);

    println!("Program finished execution in {} seconds.", before.elapsed().as_secs_f64());

    Ok(())
}
