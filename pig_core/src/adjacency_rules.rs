use crate::{TileIndex, Direction};

pub struct Rule {
    from: TileIndex,
    to: TileIndex,
    in_direction: Direction
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
            in_direction: direction,
            to
        });
    }

    // pub fn 
}