use std::collections::HashSet;

use rand::distributions::{WeightedIndex, Distribution};
use rand_chacha::ChaCha20Rng;
use serde::{Serialize, Deserialize};

use crate::{TileIndex, tile_types::NONE};

#[derive(Clone, Deserialize)]
pub struct TileWeights {
    weights: Vec<f32>
}

impl TileWeights {
    pub fn new(weights: Vec<f32>) -> Self {
        Self {
            weights
        }
    }

    pub fn from_allowed_indices(&self, allowed: HashSet<TileIndex>, seeded_rng: &mut ChaCha20Rng) -> TileIndex {
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

        weighted_index_result.unwrap().sample(seeded_rng) as u8
    }

    pub fn from_all(&self, seeded_rng: &mut ChaCha20Rng) -> TileIndex {

        let weighted_index_result = WeightedIndex::new(self.weights.iter().copied());

        if let Err(e) = weighted_index_result {
            return NONE;
        }

        weighted_index_result.unwrap().sample(seeded_rng) as u8
    }
}