// not really a grid. 

use std::cell;
use std::collections::HashMap;

use crate::{TileIndex, tile_types};
use crate::{cell::Cell, Direction, coord::Coord};
use crate::error::{Result, PigError};
use crate::tile_types::*;

pub struct PhantomGrid {
    cells: HashMap<Coord, Cell>
}

impl PhantomGrid {
    pub fn new() -> Self {
        Self {
            cells: HashMap::new()
        }
    }

    pub fn add_cell(&mut self, cell: Cell) -> Result<()> {
        if self.cells.contains_key(cell.coord()) {
            return Err(PigError::TileAddError { why: format!("cell already exists at {:?} idiot", cell.coord()) });
        }
        
        self.cells.insert(cell.coord().clone(), cell);

        Ok(())
    }

    pub fn cell_at(&self, cell_coord: &Coord) -> Option<&Cell> {
        self.cells.get(cell_coord)
    }

    pub fn cell_at_mut(&mut self, cell_coord: &Coord) -> Option<&mut Cell> {
        self.cells.get_mut(cell_coord)
    }

    pub fn adjacent(&self, cell_coord: &Coord, dir: Direction) -> Option<&Cell> {
        match dir {
            Direction::UP => self.cell_at(&Coord::new(cell_coord.x(), cell_coord.y() + 1)),
            Direction::DOWN => self.cell_at(&Coord::new(cell_coord.x(), cell_coord.y() - 1)),
            Direction::LEFT => self.cell_at(&Coord::new(cell_coord.x() - 1, cell_coord.y())),
            Direction::RIGHT => self.cell_at(&Coord::new(cell_coord.x() + 1, cell_coord.y())),
        }
    }

    pub fn adjacent_mut(&mut self, cell_coord: &Coord, dir: Direction) -> Option<&mut Cell> {
        match dir {
            Direction::UP => self.cell_at_mut(&Coord::new(cell_coord.x(), cell_coord.y() + 1)),
            Direction::DOWN => self.cell_at_mut(&Coord::new(cell_coord.x(), cell_coord.y() - 1)),
            Direction::LEFT => self.cell_at_mut(&Coord::new(cell_coord.x() - 1, cell_coord.y())),
            Direction::RIGHT => self.cell_at_mut(&Coord::new(cell_coord.x() + 1, cell_coord.y())),
        }
    }
    
    pub fn radius_iter<'a>(&'a self, radius: u32, from_pos: Coord) -> impl Iterator<Item = Option<&Cell>> + 'a {
        let radius: i32 = radius as i32;

        (from_pos.y() - radius..=from_pos.y() + radius).flat_map(move |y| {
            (from_pos.x() - radius..=from_pos.x() + radius).map(move |x| {
                self.cell_at(&Coord::new(x, y))
            })
        })
    }

    pub fn is_surrounded_by_cells(&self, cell_pos: &Coord) -> bool {
        let cells = self.surrounding_cells(cell_pos);
        
        cells.iter().find(|(c, _)| {
            c.is_none()
        }).is_none()
    }

    pub fn surrounding_cells(&self, cell_pos: &Coord) -> Vec<(Option<&Cell>, Direction)> {
        let mut ret = Vec::new();

        for dir in [Direction::UP, Direction::DOWN, Direction::LEFT, Direction::RIGHT] {
            ret.push((self.adjacent(cell_pos, dir), dir));
        }

        ret
    }

    pub fn surrounding_nonexisting_cells(&self, cell_pos: &Coord) -> Vec<Direction> {
        let mut ret = Vec::new();

        for dir in [Direction::UP, Direction::DOWN, Direction::LEFT, Direction::RIGHT] {
            if self.adjacent(cell_pos, dir).is_none() {
                ret.push(dir);
            }
        }
        
        ret
    }

    pub fn surrounding_existing_cells(&self, cell_pos: &Coord) -> Vec<(&Cell, Direction)> {
        let mut ret = Vec::new();

        for dir in [Direction::UP, Direction::DOWN, Direction::LEFT, Direction::RIGHT] {
            if let Some(c) = self.adjacent(cell_pos, dir) {
                ret.push((c, dir));
            }
        }
        
        ret
    }

    pub fn print_state(&self, numerical: bool) {
        let mut state_str = String::new();

        let mut top = 0;
        let mut bottom = 0;
        let mut left = 0;
        let mut right = 0;

        for (_key, cell) in &self.cells {
            // println!("{:?}", cell);
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

        let tile_to_ascii = |tile_idx: u8| {
            match tile_idx {
                TOP_RIGHT_CORNER => '┐',
                BOTTOM_RIGHT_CORNER => '┘',
                TOP_LEFT_CORNER => '┌',
                BOTTOM_LEFT_CORNER => '└',
                TOP_WALL => '▔',
                BOTTOM_WALL => '▁',
                LEFT_WALL => '▏',
                RIGHT_WALL => '▕',
                EMPTY => '░',
                NONE => '◇',
                _ => unreachable!()
            }
        };

        println!("top: {top}, bottom: {bottom}\nleft: {left}, right: {right}");

        for y in (bottom..=top).rev() {
            for x in left..=right {
                let cell = self.cell_at(&Coord::new(x, y));

                if let Some(t) = cell {
                    if numerical {
                        state_str.push(tile_to_ascii(t.tile_index()));
                    } else {
                        state_str = format!("{state_str}{}", t.tile_index());
                    }
                } else {
                    if numerical {
                        state_str.push(' ');
                    } else {
                        state_str.push(' ');
                    }
                }
            }
            state_str.push('\n');
        }

        println!("{}", state_str);
    }
}