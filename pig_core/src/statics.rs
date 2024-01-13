use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::phantom_grid::PhantomGrid;

lazy_static! {
    pub static ref GRID: Mutex<PhantomGrid> = Mutex::new(PhantomGrid::new());
}