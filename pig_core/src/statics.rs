use std::{sync::Mutex, ffi::CString};

use lazy_static::lazy_static;

use crate::phantom_grid::PhantomGrid;

lazy_static! {
    pub static ref GRID: Mutex<PhantomGrid> = Mutex::new(PhantomGrid::new());
    pub static ref FFI_STATUS_MESSAGE: Mutex<CString> = Mutex::new(CString::new("new str").unwrap());
}