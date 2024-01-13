// not really a grid. 

use std::os::linux::raw::stat;

use crate::{cell::Cell, Direction, coord::Coord};
use crate::error::{Result, PigError};

pub struct PhantomGrid {
    tiles: Vec<Cell>
}

impl PhantomGrid {
    pub fn new() -> Self {
        Self {
            tiles: Vec::new()
        }
    }

    pub fn add_tile(&mut self, tile: Cell) -> Result<()> {
        if self.tile_at(tile.coord()).is_some() {
            return Err(PigError::TileAddError { why: "uh huhuhuh uheu heh ehhe".to_string() });
        }
        self.tiles.push(tile);

        Ok(())
    }

    pub fn tile_at(&self, tile_coord: &Coord) -> Option<&Cell> {
        self.tiles.iter().find(|t| *t.coord() == *tile_coord)
    }

    pub fn tile_at_mut(&mut self, tile_coord: &Coord) -> Option<&mut Cell> {
        self.tiles.iter_mut().find(|t| *t.coord() == *tile_coord)
    }

    pub fn adjacent(&self, tile_coord: &Coord, dir: Direction) -> Option<&Cell> {
        match dir {
            Direction::UP => self.tile_at(&Coord::new(tile_coord.x(), tile_coord.y() + 1)),
            Direction::DOWN => self.tile_at(&Coord::new(tile_coord.x(), tile_coord.y() - 1)),
            Direction::LEFT => self.tile_at(&Coord::new(tile_coord.x() - 1, tile_coord.y())),
            Direction::RIGHT => self.tile_at(&Coord::new(tile_coord.x() + 1, tile_coord.y())),
        }
    }

    pub fn adjacent_mut(&mut self, tile_coord: Coord, dir: Direction) -> Option<&mut Cell> {
        match dir {
            Direction::UP => self.tile_at_mut(&Coord::new(tile_coord.x(), tile_coord.y() + 1)),
            Direction::DOWN => self.tile_at_mut(&Coord::new(tile_coord.x(), tile_coord.y() - 1)),
            Direction::LEFT => self.tile_at_mut(&Coord::new(tile_coord.x() - 1, tile_coord.y())),
            Direction::RIGHT => self.tile_at_mut(&Coord::new(tile_coord.x() + 1, tile_coord.y())),
        }
    }

    pub fn print_state(&self) {
        let mut state_str = String::new();

        let mut top = 0;
        let mut bottom = 0;
        let mut left = 0;
        let mut right = 0;

        for tile in &self.tiles {
            if tile.coord().y() > top {
                top = tile.coord().y();
            }
            if tile.coord().y() < bottom {
                bottom = tile.coord().y();
            }
            if tile.coord().x() > right {
                right = tile.coord().x();
            }
            if tile.coord().x() < left {
                left = tile.coord().x();
            }
        }

        println!("top: {top}, bottom: {bottom}\nleft: {left}, right: {right}");

        for y in (bottom..=top).rev() {
            for x in left..=right {
                let tile = self.tile_at(&Coord::new(x, y));

                if let Some(t) = tile {
                    state_str = format!("{state_str}{}", t.tile_index());
                } else {
                    state_str.push(' ');
                }
            }
            state_str.push('\n');
        }

        println!("{}", state_str);
    }
}