use std::ffi::{c_char, CStr, CString};

use serde::{Deserialize, Serialize};

use crate::{core::{pig_generate_internal, clear_grid}, adjacency_rules::AdjacencyRules, tile_weights::{TileWeights, self}, coord::Coord, statics::FFI_STATUS_MESSAGE};

// returns json

#[derive(Deserialize, Serialize)]
pub struct InputData {
    pub iters: u32,
    pub coord: Coord,
    pub tile_weights: Vec<f32>,
    pub seed: u64,
    pub search_radius: u32
}

#[no_mangle]
pub extern "C" fn pig_generate(json_str: *const c_char) -> *mut c_char {
    let adjacency_rules = AdjacencyRules::default();
    
    let c_str = unsafe { CStr::from_ptr(json_str) };
    let json_string = c_str.to_str().expect("Invalid UTF-8 string").to_string();
    let input_data: InputData = serde_json::from_str(&json_string).expect("Failed to deserialize JSON");
    
    let tile_weights = TileWeights::new(input_data.tile_weights);

    let tile_infos = pig_generate_internal(input_data.iters, input_data.coord, &adjacency_rules, &tile_weights, input_data.seed, input_data.search_radius).unwrap();

    let json_value = serde_json::to_string(&tile_infos).unwrap();
    
    let c_result = CString::new(json_value).expect("Failed to convert to CString");

    // Return the C string
    c_result.into_raw()
}

#[no_mangle]
pub extern "C" fn pig_reset() {
    clear_grid();
}

#[no_mangle]
pub extern "C" fn ffi_ptr_parse_test(json_str: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(json_str) };
    let json_string = c_str.to_str().expect("Invalid UTF-8 string").to_string();
    let result: Result<InputData, serde_json::Error> = serde_json::from_str(&json_string);

    if result.is_err() {
        return CString::new(format!("THERE IS ERROR IDIOT: {:?}", result.err())).unwrap().into_raw()
    } else {
        return CString::new("succvess").unwrap().into_raw();
    }
    
}


#[no_mangle]
pub extern "C" fn ffi_test_add_three(x: i32) -> i32 {
    x + 3
}