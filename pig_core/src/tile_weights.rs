use crate::TileIndex;

pub struct TileWeights {
    weights: Vec<f32>
}

impl TileWeights {
    pub fn new(weights: Vec<f32>) -> Self {
        Self {
            weights
        }
    }
}