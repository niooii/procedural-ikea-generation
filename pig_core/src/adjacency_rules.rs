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
    pub fn allow(&mut self, from: TileIndex, direction: Direction, to: TileIndex) {
        self.rules.push(Rule {
            from,
            in_direction: direction,
            to
        });
    }
}