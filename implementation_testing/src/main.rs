// use std::time::Instant;

// // use wfc::{self, AdjacencyRules, FrequencyHints, TileIndex};
// use pig_core::{self, phantom_grid::PhantomGrid, cell::Cell, error::Result, tile_weights::TileWeights, adjacency_rules::AdjacencyRules, tile_types::*, Direction::*, ffi::InputData, core::pig_generate_internal};
// use crate::pig_core::coord::Coord;
// use pig_core::allow;

// fn main() -> Result<()> {
//     let before = Instant::now();
//     let tile_weights = TileWeights::new(vec![
//         1.0, // BOTTOM_LEFT_CORNER
//         1.0, // BOTTOM_RIGHT_CORNER
//         1.0, // TOP_LEFT_CORNER
//         1.0, // TOP_RIGHT_CORNER
//         1.0, // LEFT_WALL
//         1.0, // RIGHT_WALL
//         1.0, // UP_WALL
//         1.0, // DOWN_WALL
//         2.0, // EMPTY
//         0.0, // NONE
//     ]);
    
//     let mut adjacency_rules = AdjacencyRules::default();
//     /*
//     @PARAMS
//     from: first parameter - tile to go in in_direction from.
//     in_direction: in direction
//     to: allowed to be the type "to" in direction "in_direction" from "from".
//      */
    
    

//     for n in 0..1 {
//         pig_generate_internal(20, Coord::new(n * 20, 0), &adjacency_rules, &tile_weights)?;
//     }

//     let input_data = InputData {
//         iters: 10,
//         coord: Coord::new(1, 2),
//         tile_weights: vec![
//             1.0,
//             1.0,
//             1.0,
//             1.0,
//             1.0,
//             1.0,
//             1.0,
//             1.0
//         ]
//     };

//     // let json = serde_json::to_string(&input_data);

//     // println!("{:?}", json);

//     // let input_data: InputData = serde_json::from_str("{\"iters\":0,\"coord\":{\"x\":0,\"y\":0},\"tile_weights\":[1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0]}").unwrap();

//     // println!("{:?}", input_data.coord);
    

//     println!("Program finished execution in {} seconds.", before.elapsed().as_secs_f64());

//     Ok(())
// }
