use std::collections::HashSet;

use rand::distributions::{WeightedIndex, Distribution};

use crate::{TileIndex, tile_types::NONE};

#[derive(Clone)]
pub struct TileWeights {
    weights: Vec<f32>
}

impl TileWeights {
    pub fn new(weights: Vec<f32>) -> Self {
        Self {
            weights
        }
    }

    pub fn from_allowed_indices(&self, allowed: HashSet<TileIndex>) -> TileIndex {
        // println!("allowed: {:?}", allowed);
        let weights = self.weights.iter().enumerate().map(|(i, w)| {
            if allowed.contains(&(i as u8)) {
                w
            } else {
                &0.0
            }
        }).collect::<Vec<&f32>>();

        let weighted_index_result = WeightedIndex::new(weights);

        if let Err(e) = weighted_index_result {
            return NONE;
        }

        let mut rng = rand::thread_rng();
        weighted_index_result.unwrap().sample(&mut rng) as u8
    }
}