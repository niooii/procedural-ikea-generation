// not really a grid. 

use crate::TileIndex;
use crate::{cell::Cell, Direction, coord::Coord};
use crate::error::{Result, PigError};

pub struct PhantomGrid {
    cells: Vec<Cell>
}

impl PhantomGrid {
    pub fn new() -> Self {
        Self {
            cells: Vec::new()
        }
    }

    pub fn add_tile(&mut self, tile: Cell) -> Result<()> {
        if self.cell_at(tile.coord()).is_some() {
            return Err(PigError::TileAddError { why: "uh huhuhuh uheu heh ehhe".to_string() });
        }
        self.cells.push(tile);

        Ok(())
    }

    pub fn cell_at(&self, tile_coord: &Coord) -> Option<&Cell> {
        self.cells.iter().find(|t| *t.coord() == *tile_coord)
    }

    pub fn cell_at_mut(&mut self, tile_coord: &Coord) -> Option<&mut Cell> {
        self.cells.iter_mut().find(|t| *t.coord() == *tile_coord)
    }

    pub fn adjacent(&self, tile_coord: &Coord, dir: Direction) -> Option<&Cell> {
        match dir {
            Direction::UP => self.cell_at(&Coord::new(tile_coord.x(), tile_coord.y() + 1)),
            Direction::DOWN => self.cell_at(&Coord::new(tile_coord.x(), tile_coord.y() - 1)),
            Direction::LEFT => self.cell_at(&Coord::new(tile_coord.x() - 1, tile_coord.y())),
            Direction::RIGHT => self.cell_at(&Coord::new(tile_coord.x() + 1, tile_coord.y())),
        }
    }

    pub fn adjacent_mut(&mut self, tile_coord: &Coord, dir: Direction) -> Option<&mut Cell> {
        match dir {
            Direction::UP => self.cell_at_mut(&Coord::new(tile_coord.x(), tile_coord.y() + 1)),
            Direction::DOWN => self.cell_at_mut(&Coord::new(tile_coord.x(), tile_coord.y() - 1)),
            Direction::LEFT => self.cell_at_mut(&Coord::new(tile_coord.x() - 1, tile_coord.y())),
            Direction::RIGHT => self.cell_at_mut(&Coord::new(tile_coord.x() + 1, tile_coord.y())),
        }
    }
    
    pub fn radius_iter<'a>(&'a self, radius: u32, from_pos: Coord) -> impl Iterator<Item = Option<&Cell>> + 'a {
        let radius: i32 = radius as i32;
        // self.cells.iter().filter(|c| {
        //     let coord = c.coord();
        //     coord.x() > from_pos.x() - radius && coord.x() < from_pos.x() + radius &&
        //     coord.y() < from_pos.y() + radius && coord.y() > from_pos.y() - radius
        // }).into_iter()

        (from_pos.y() - radius..=from_pos.y() + radius).flat_map(move |y| {
            (from_pos.x() - radius..=from_pos.x() + radius).map(move |x| {
                self.cell_at(&Coord::new(x, y))
            })
        })
    }

    pub fn surrounded_by_collapsed_cells(&self, tile_pos: &Coord) -> bool {
        todo!();
    }

    pub fn with_least_options(&self) -> Coord {
        let iter = self.cells.iter().filter(|c| {
            !c.is_collapsed()
        });

        // optimize later for it to return early if it hits a cell with two or 1.
        for c in iter {
            c.possible_indicies().count()
        }
    }

    pub fn print_state(&self) {
        let mut state_str = String::new();

        let mut top = 0;
        let mut bottom = 0;
        let mut left = 0;
        let mut right = 0;

        for cell in &self.cells {
            if cell.coord().y() > top {
                top = cell.coord().y();
            }
            if cell.coord().y() < bottom {
                bottom = cell.coord().y();
            }
            if cell.coord().x() > right {
                right = cell.coord().x();
            }
            if cell.coord().x() < left {
                left = cell.coord().x();
            }
        }

        println!("top: {top}, bottom: {bottom}\nleft: {left}, right: {right}");

        for y in (bottom..=top).rev() {
            for x in left..=right {
                let tile = self.cell_at(&Coord::new(x, y));

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