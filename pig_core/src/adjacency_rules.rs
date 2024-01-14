use std::collections::HashSet;

use crate::{TileIndex, Direction};

// SANITY CHECKS
#[macro_export]
macro_rules! allow {
    ($adjacency_rules:expr, $from:ident, $dir:expr, $to:ident) => {
        $adjacency_rules.allow($from, $dir, $to);
    };
}

pub struct Rule {
    from: TileIndex,
    to: TileIndex,
    in_dir: Direction
}

pub struct AdjacencyRules {
    rules: Vec<Rule>
}

impl AdjacencyRules {
    pub fn new() -> Self {
        Self {
            rules: Vec::new()
        }
    }
    pub fn allow(&mut self, from: TileIndex, direction: Direction, to: TileIndex) {
        self.rules.push(Rule {
            from,
            in_dir: direction,
            to
        });
    }
    pub fn allowed_tile_indices(&self, from_tile: TileIndex, in_dir: Direction) -> HashSet<TileIndex> {
        let mut allowed_tiles = HashSet::new();

        for rule in &self.rules {
            if rule.from == from_tile && rule.in_dir == in_dir {
                allowed_tiles.insert(rule.to);
                // println!("found allowed rule: {} to {} in direction {:?}", rule.from, rule.to, rule.in_dir);
            }
        }

        allowed_tiles
    }

}