// use wfc::{self, AdjacencyRules, FrequencyHints, TileIndex};
use pig_core::{self, phantom_grid::PhantomGrid, cell::Cell, error::Result, tile_weights::TileWeights, adjacency_rules::AdjacencyRules, tile_types::*, Direction::*, core::pig_generate};
use crate::pig_core::coord::Coord;
use pig_core::allow;

fn main() -> Result<()> {
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
    
    let mut adjacency_rules = AdjacencyRules::new();
    /*
    @PARAMS
    from: first parameter - tile to go in in_direction from.
    in_direction: in direction
    to: allowed to be the type "to" in direction "in_direction" from "from".
     */
    
    allow!(adjacency_rules, EMPTY, UP, EMPTY);
    allow!(adjacency_rules, EMPTY, DOWN, EMPTY);
    allow!(adjacency_rules, EMPTY, LEFT, EMPTY);
    allow!(adjacency_rules, EMPTY, RIGHT, EMPTY);
    
    // allow empty for every FUCKING tile type.
    for tile_type in ALL_TYPES {
        allow!(adjacency_rules, EMPTY, UP, tile_type);
        allow!(adjacency_rules, EMPTY, DOWN, tile_type);
        allow!(adjacency_rules, EMPTY, LEFT, tile_type);
        allow!(adjacency_rules, EMPTY, RIGHT, tile_type);

        allow!(adjacency_rules, tile_type, UP, EMPTY);
        allow!(adjacency_rules, tile_type, DOWN, EMPTY);
        allow!(adjacency_rules, tile_type, LEFT, EMPTY);
        allow!(adjacency_rules, tile_type, RIGHT, EMPTY);
    }

    // corner wall interactions
    allow!(adjacency_rules, TOP_LEFT_CORNER, DOWN, LEFT_WALL);
    allow!(adjacency_rules, TOP_RIGHT_CORNER, LEFT, TOP_WALL);
    allow!(adjacency_rules, BOTTOM_LEFT_CORNER, RIGHT, BOTTOM_WALL);
    allow!(adjacency_rules, BOTTOM_RIGHT_CORNER, LEFT, RIGHT_WALL);
    // theres more interactiosn LOL 

    // wall to wall interactions
    allow!(adjacency_rules, TOP_WALL, LEFT, TOP_WALL);
    allow!(adjacency_rules, TOP_WALL, RIGHT, TOP_WALL);
    allow!(adjacency_rules, BOTTOM_WALL, LEFT, BOTTOM_WALL);
    allow!(adjacency_rules, BOTTOM_WALL, RIGHT, BOTTOM_WALL);

    allow!(adjacency_rules, LEFT_WALL, UP, LEFT_WALL);
    allow!(adjacency_rules, LEFT_WALL, DOWN, LEFT_WALL);
    allow!(adjacency_rules, RIGHT_WALL, UP, RIGHT_WALL);
    allow!(adjacency_rules, RIGHT_WALL, DOWN, RIGHT_WALL);

    pig_generate(20, Coord::new(0, 0), adjacency_rules.clone(), tile_weights.clone());

    Ok(())
}
