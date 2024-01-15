use std::{collections::hash_map::DefaultHasher, hash::{Hasher, Hash}};

use serde::{Serialize, Deserialize};

use crate::Direction;


#[derive(Clone, Copy, Debug, Eq, Hash, Serialize, Deserialize)]
pub struct Coord {
    x: i32,
    y: i32
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn translated(&self, dir: Direction) -> Self {
        match dir {
            Direction::UP => Coord::new(self.x(), self.y() + 1),
            Direction::DOWN => Coord::new(self.x(), self.y() - 1),
            Direction::LEFT => Coord::new(self.x() - 1, self.y()),
            Direction::RIGHT => Coord::new(self.x() + 1, self.y()),
        }
    }

    pub fn hash_coordinate(&self, seed: u64) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        seed.hash(&mut hasher);
        hasher.finish()
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}