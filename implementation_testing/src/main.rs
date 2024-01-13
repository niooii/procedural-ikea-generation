// use wfc::{self, AdjacencyRules, FrequencyHints, TileIndex};
use pig_core::{self, phantom_grid::PhantomGrid, cell::Cell};
use crate::pig_core::coord::Coord;

fn main() {
    // let frequency_hints = FrequencyHints::new(
    //     vec![
    //         2,
    //         2,
    //         2,
    //         2,
    //         2,
    //         2,
    //         2,
    //         2,
    //         2
    //     ]
    // );
    // let freq_hints = frequency_hints;
    // let mut adjacency_rules = AdjacencyRules::new(9);
    // // ----------! TILE INDICES !------------
    // // BOTTOM LEFT CORNER: 0
    // // BOTTOM RIGHT CORNER: 1
    // // TOP LEFT CORNER: 2
    // // TOP RIGHT CORNER: 3
    // // LEFT WALL: 4
    // // RIGHT WALL: 5
    // // UP WALL: 6
    // // DOWN WALL: 7
    // // EMPTY: 8
    // // my ass forgot i can define them as consts..
    // const BOTTOM_LEFT_CORNER: TileIndex = 0;
    // const BOTTOM_RIGHT_CORNER: TileIndex = 1;
    // const TOP_LEFT_CORNER: TileIndex = 2;
    // const TOP_RIGHT_CORNER: TileIndex = 3;
    // const LEFT_WALL: TileIndex = 4;
    // const RIGHT_WALL: TileIndex = 5;
    // const UP_WALL: TileIndex = 6;
    // const DOWN_WALL: TileIndex = 7;
    // const EMPTY: TileIndex = 8;
    // // Recall that an adjacency rule is of the form:
    // // “Tile index A may appear in the cell 1 space in DIRECTION from a cell containing tile index B”
    // // index 2 is the one its going DOWN From.
    // // Define adjacency rules
    // adjacency_rules.allow(BOTTOM_LEFT_CORNER, LEFT_WALL, wfc::DOWN);
    // adjacency_rules.allow(LEFT_WALL, BOTTOM_LEFT_CORNER, wfc::DOWN);

    // // Additional adjacency rules based on the provided information
    // adjacency_rules.allow(BOTTOM_LEFT_CORNER, BOTTOM_RIGHT_CORNER, wfc::RIGHT);
    // adjacency_rules.allow(BOTTOM_RIGHT_CORNER, BOTTOM_LEFT_CORNER, wfc::LEFT);

    // adjacency_rules.allow(BOTTOM_LEFT_CORNER, TOP_LEFT_CORNER, wfc::UP);
    // adjacency_rules.allow(TOP_LEFT_CORNER, BOTTOM_LEFT_CORNER, wfc::DOWN);

    // adjacency_rules.allow(BOTTOM_RIGHT_CORNER, TOP_RIGHT_CORNER, wfc::UP);
    // adjacency_rules.allow(TOP_RIGHT_CORNER, BOTTOM_RIGHT_CORNER, wfc::DOWN);

    // adjacency_rules.allow(LEFT_WALL, RIGHT_WALL, wfc::RIGHT);
    // adjacency_rules.allow(RIGHT_WALL, LEFT_WALL, wfc::LEFT);

    // adjacency_rules.allow(UP_WALL, DOWN_WALL, wfc::DOWN);
    // adjacency_rules.allow(DOWN_WALL, UP_WALL, wfc::UP);
    // let output = wfc::wfc_core(AdjacencyRules::new(9), freq_hints, (888, 888));

    // println!("{output:?}");

    let mut grid = PhantomGrid::new();

    grid.add_tile(Cell::new(Coord::new(2, 2)));
    grid.add_tile(Cell::new(Coord::new(2, 1)));
    grid.add_tile(Cell::new(Coord::new(-3, -5)));
    grid.add_tile(Cell::new(Coord::new(-2, -2)));

    grid.print_state();
}
