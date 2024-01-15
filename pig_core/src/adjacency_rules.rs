use std::collections::{HashSet, HashMap};
use crate::Direction;
use crate::{TileIndex, Direction::*, tile_types::*};

// SANITY CHECKS
#[macro_export]
macro_rules! allow {
    ($adjacency_rules:expr, $from:ident, $dir:expr, $to:ident) => {
        $adjacency_rules.allow($from, $dir, $to);
    };
}

#[derive(Clone, Copy)]
pub struct Rule {
    from: TileIndex,
    to: TileIndex,
    in_dir: Direction
}

#[derive(Clone)]
pub struct AdjacencyRules {
    rules: HashMap<TileIndex, Vec<Rule>>
}

impl AdjacencyRules {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new()
        }
    }
    pub fn default() -> Self {
        let mut adjacency_rules = Self::new();

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

        // Corner corner interactions
        allow!(adjacency_rules, TOP_LEFT_CORNER, RIGHT, TOP_RIGHT_CORNER); // normal
        allow!(adjacency_rules, TOP_RIGHT_CORNER, LEFT, TOP_LEFT_CORNER); // inverted

        allow!(adjacency_rules, TOP_LEFT_CORNER, DOWN, BOTTOM_LEFT_CORNER); // normal
        allow!(adjacency_rules, BOTTOM_LEFT_CORNER, UP, TOP_LEFT_CORNER); // inverted

        allow!(adjacency_rules, TOP_RIGHT_CORNER, DOWN, BOTTOM_RIGHT_CORNER); // normal
        allow!(adjacency_rules, BOTTOM_RIGHT_CORNER, UP, TOP_RIGHT_CORNER); // inverted

        allow!(adjacency_rules, BOTTOM_LEFT_CORNER, RIGHT, BOTTOM_RIGHT_CORNER); // normal
        allow!(adjacency_rules, BOTTOM_RIGHT_CORNER, LEFT, BOTTOM_LEFT_CORNER); // inverted


        // Corner wall interactions
        allow!(adjacency_rules, TOP_LEFT_CORNER, DOWN, LEFT_WALL); // normal
        allow!(adjacency_rules, LEFT_WALL, UP, TOP_LEFT_CORNER); // inverted

        allow!(adjacency_rules, TOP_RIGHT_CORNER, LEFT, TOP_WALL); // normal
        allow!(adjacency_rules, TOP_WALL, RIGHT, TOP_RIGHT_CORNER); // inverted

        allow!(adjacency_rules, BOTTOM_LEFT_CORNER, RIGHT, BOTTOM_WALL); // normal
        allow!(adjacency_rules, BOTTOM_WALL, LEFT, BOTTOM_LEFT_CORNER); // inverted

        allow!(adjacency_rules, BOTTOM_RIGHT_CORNER, LEFT, RIGHT_WALL); // normal
        allow!(adjacency_rules, RIGHT_WALL, RIGHT, BOTTOM_RIGHT_CORNER); // inverted

        allow!(adjacency_rules, TOP_LEFT_CORNER, RIGHT, TOP_WALL); // normal
        allow!(adjacency_rules, TOP_WALL, LEFT, TOP_LEFT_CORNER); // inverted

        allow!(adjacency_rules, TOP_RIGHT_CORNER, LEFT, TOP_WALL); // normal
        allow!(adjacency_rules, TOP_WALL, RIGHT, TOP_RIGHT_CORNER); // inverted

        allow!(adjacency_rules, BOTTOM_LEFT_CORNER, RIGHT, BOTTOM_WALL); // normal
        allow!(adjacency_rules, BOTTOM_WALL, LEFT, BOTTOM_LEFT_CORNER); // inverted

        allow!(adjacency_rules, BOTTOM_RIGHT_CORNER, LEFT, BOTTOM_WALL); // normal
        allow!(adjacency_rules, BOTTOM_WALL, RIGHT, BOTTOM_RIGHT_CORNER); // inverted

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

        return adjacency_rules;
    }
    pub fn allow(&mut self, from: TileIndex, direction: Direction, to: TileIndex) {
        if !self.rules.contains_key(&from) {
            self.rules.insert(from, Vec::new());
        }

        self.rules.get_mut(&from).unwrap().push(Rule {
            from,
            in_dir: direction,
            to
        });
    }
    pub fn allowed_tile_indices(&self, from_tile: TileIndex, in_dir: Direction) -> HashSet<TileIndex> {
        let mut allowed_tiles = HashSet::new();

        for rule in self.rules.get(&from_tile).unwrap() {
            if rule.in_dir == in_dir {
                allowed_tiles.insert(rule.to);
                // println!("found allowed rule: {} to {} in direction {:?}", rule.from, rule.to, rule.in_dir);
            }
        }

        allowed_tiles
    }

}