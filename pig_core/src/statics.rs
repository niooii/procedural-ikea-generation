use lazy_static::lazy_static;

use crate::phantom_grid::PhantomGrid;

lazy_static! {
    pub static ref grid: PhantomGrid = PhantomGrid::new();
}