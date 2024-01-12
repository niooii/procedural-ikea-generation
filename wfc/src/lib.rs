use std::{collections::BinaryHeap, cmp::Ordering};

use rand::Rng;
use coord_2d::{Coord, Size};
use grid_2d::Grid;
use grid_2d::*;
use noisy_float::prelude::*;

// ------- TYPES ---------
pub type TileIndex = usize;

pub type Direction = usize;

pub const UP:    Direction = 0;
pub const DOWN:  Direction = 1;
pub const LEFT:  Direction = 2;
pub const RIGHT: Direction = 3;

pub const NUM_DIRECTIONS: usize = 4;
pub const ALL_DIRECTIONS: [Direction; NUM_DIRECTIONS] = [UP, DOWN, LEFT, RIGHT];

fn opposite(dir: Direction) -> Direction {
    match dir {
        UP => DOWN,
        LEFT => RIGHT,
        RIGHT => LEFT,
        DOWN => UP,
        _ => unreachable!("PLUH")
    }
}

#[derive(Clone)]
pub struct FrequencyHints {
    frequencies: Vec<usize>,
}


impl FrequencyHints {
    pub fn relative_frequency(&self, tile_index: TileIndex) -> usize { 
        *self.frequencies.get(tile_index).unwrap()
    }

    pub fn new(freqs: Vec<usize>) -> Self {
        Self {
            frequencies: freqs
        }
    }
}

#[derive(Clone)]
pub struct AdjacencyRules {
    num_tiles: usize,
    rules: Vec<(TileIndex, TileIndex, Direction)>,
}

impl AdjacencyRules {
    pub fn new(num_tiles: usize) -> Self {
        Self {
            num_tiles,
            rules: Vec::new()
        }
    }

    pub fn num_tiles(&self) -> usize {
        self.num_tiles
    }

    pub fn allow(&mut self, idx1: TileIndex, idx2: TileIndex, dir: Direction) {
        self.rules.push((idx1, idx2, dir));
    }

    pub fn compatible_tiles(&self, with: TileIndex, dir: Direction) -> Vec<TileIndex> {
        let mut compatible = Vec::new();

        // TODO! may be an error in logic
        for rule in &self.rules {
            if rule.1 == with && rule.2 == dir {
                compatible.push(rule.0);
                println!("found comptaible tile");
            }
        }

        compatible
    }
}

#[derive(Clone)]
pub struct TileEnablerCount {
    by_direction: [usize; 4],
}

impl TileEnablerCount {
    fn contains_any_zero_count(&self) -> bool {
        let ret = self.by_direction.iter().find(|d| **d == 0).is_some();
        println!("{:?}", self.by_direction);
        if ret {
            println!("found zero count heh")
        }
        ret
    }
}

#[derive(Clone)]
pub struct CoreCell {
    possible: Vec<bool>,
    sum_of_possible_tile_weights: usize,
    sum_of_possible_tile_weight_log_weights: f32,

    entropy_noise: f64,
    is_collapsed: bool,

    // `tile_enabler_counts[tile_index]` will be the counts for the corresponding thingy
    tile_enabler_counts: Vec<TileEnablerCount>,
}

impl CoreCell {
    fn has_no_possible_tiles(&self) -> bool {
        self.possible_tile_iter().next().is_none()
    }

    fn remove_tile(&mut self, tile_index: TileIndex, freq_hint: &FrequencyHints) {
        self.possible[tile_index] = false;

        let freq = freq_hint.relative_frequency(tile_index);

        self.sum_of_possible_tile_weights -= freq;

        self.sum_of_possible_tile_weight_log_weights -=
            (freq as f32) * (freq as f32).log2();
    }

    fn get_only_possible_tile_index(&self) -> TileIndex {
        let only_possible_index = self.possible.iter().enumerate().find(|(i, b)| {
            **b
        }).unwrap().0;

        // println!("{only_possible_index}");

        only_possible_index
    }

    fn total_possible_tile_frequency(&self, freq_hint: &FrequencyHints) -> usize {
        let mut total = 0;
        for (tile_index, &is_possible) in self.possible.iter().enumerate() {
            if is_possible {
                total += freq_hint.relative_frequency(tile_index);
            }
        }
        return total;
    }

    fn entropy(&self) -> f64 {
        return (self.sum_of_possible_tile_weights as f64).log2()
            - (self.sum_of_possible_tile_weight_log_weights as f64 /
                self.sum_of_possible_tile_weights as f64)
    }

    // brain cancer
    fn possible_tile_iter<'a>(&'a self) -> impl Iterator<Item = TileIndex> + 'a {
        self.possible.iter().enumerate().filter(|(_, b)| **b)
            .map(|(i, _)| i)
    }    

    fn choose_tile_index(&self, frequency_hints: &FrequencyHints) -> TileIndex {
        let mut remaining =
            rand::thread_rng().gen_range(0..self.sum_of_possible_tile_weights);

        for possible_tile_index in self.possible_tile_iter() {

            let weight =
                frequency_hints.relative_frequency(possible_tile_index);

            if remaining >= weight {
                remaining -= weight;
            } else {
                return possible_tile_index;
            }
        }

        unreachable!("sum_of_possible_weights was inconsistent with \
            possible_tile_iter and FrequencyHints::relative_frequency");
    }
}

fn initial_tile_enabler_counts(
    num_tiles: usize,
    adjacency_rules: &AdjacencyRules,
) -> Vec<TileEnablerCount>
{
    let mut ret = Vec::new();

    for tile_a in 0..num_tiles {

        let mut counts = TileEnablerCount {
            by_direction: [0, 0, 0, 0],
        };

        for &direction in ALL_DIRECTIONS.iter() {

            for tile_b in adjacency_rules.compatible_tiles(tile_a, direction) {
                println!("loop once bruh");
                counts.by_direction[direction] += 1;
            }
            ret.push(counts.clone());
        }
        return ret;
    }
    ret
}


#[derive(PartialEq, Eq, PartialOrd)]
pub struct EntropyCoord {
    entropy: N64,
    coord: Coord,
}

impl Ord for EntropyCoord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.entropy.cmp(&other.entropy)
    }
}

pub struct RemovalUpdate {
    tile_index: TileIndex,
    coord: Coord,
}

pub struct CoreState {
    grid: Grid<CoreCell>,
    remaining_uncollapsed_cells: usize,
    adjacency_rules: AdjacencyRules,
    frequency_hints: FrequencyHints,
    entropy_heap: BinaryHeap<EntropyCoord>,
    tile_removals: Vec<RemovalUpdate>
}

impl CoreState {

    // return the coordinate of the next cell to collapse
    fn choose_next_cell(&mut self) -> Coord {
        while let Some(entropy_coord) = self.entropy_heap.pop() {
            let cell = self.grid.get(entropy_coord.coord).unwrap();
            if !cell.is_collapsed {
                return entropy_coord.coord;
            }
        }

        // should not end up here
        unreachable!("entropy_heap is empty, but there are still \
            uncollapsed cells");
    }

    fn collapse_cell_at(&mut self, coord: Coord) {
        let cell = &mut self.grid.get_mut(coord).unwrap();
        let tile_index_to_lock_in = cell.choose_tile_index(&self.frequency_hints);

        cell.is_collapsed = true;

        for (tile_index, possible) in cell.possible.iter_mut().enumerate() {
            if tile_index != tile_index_to_lock_in {
                *possible = false;
            }
        }
    }

    // remove possibilities based on collapsed cell
    fn propagate(&mut self) {
        while let Some(removal_update) = self.tile_removals.pop() {

            for &direction in ALL_DIRECTIONS.iter() {
                let neighbour_coord = match direction {
                    // TODO! may be wrong.
                    UP => Coord::new(removal_update.coord.x, removal_update.coord.y - 1),
                    DOWN => Coord::new(removal_update.coord.x, removal_update.coord.y + 1),
                    LEFT => Coord::new(removal_update.coord.x - 1, removal_update.coord.y),
                    RIGHT => Coord::new(removal_update.coord.x + 1, removal_update.coord.y),
                    _ => unreachable!("PLUH")
                };
                let neighbour_cell = self.grid.get_checked_mut(neighbour_coord);

                for compatible_tile in self.adjacency_rules.compatible_tiles(
                    removal_update.tile_index,
                    direction,
                ) {

                    // relative to `neighbour_cell`, the cell at
                    // `removal_update.coord` is in the opposite direction to
                    // `direction`
                    let opposite_direction = opposite(direction);

                    // look up the count of enablers for this tile

                    // check if we're about to decrement this to 0
                    if neighbour_cell.tile_enabler_counts[compatible_tile].by_direction[direction] == 1 {

                        // if there is a zero count in another direction,
                        // the potential tile has already been removed,
                        // and we want to avoid removing it again
                        if !neighbour_cell.tile_enabler_counts[compatible_tile].contains_any_zero_count() {
                            // remove the possibility
                            neighbour_cell.remove_tile(
                                compatible_tile,
                                &self.frequency_hints,
                            );
                            // check for contradiction
                            if neighbour_cell.has_no_possible_tiles() {
                                panic!("contradiction");
                                // CONTRADICTION!!!
                            }
                            // this probably changed the cell's entropy
                            self.entropy_heap.push(EntropyCoord {
                                entropy: N64::new(neighbour_cell.entropy()),
                                coord: neighbour_coord,
                            });
                            // add the update to the stack
                            self.tile_removals.push(RemovalUpdate {
                                tile_index: compatible_tile,
                                coord: neighbour_coord,
                            });
                        }
                    }

                    neighbour_cell.tile_enabler_counts[compatible_tile].by_direction[direction] -= 1;
                }
            }
        }
    }

    fn run(&mut self) {
        while self.remaining_uncollapsed_cells > 0 {
            let next_coord = self.choose_next_cell();
            self.collapse_cell_at(next_coord);
            self.propagate();
            self.remaining_uncollapsed_cells -= 1;
        }
    }
}

pub fn wfc_core(
    adjacency_rules: AdjacencyRules,
    frequency_hints: FrequencyHints,
    output_size: (u32, u32),
) -> Grid<TileIndex>
{
    let num_tiles = adjacency_rules.num_tiles();

    let cell_template = CoreCell {
        possible: (0..num_tiles).map(|_| true).collect(),
        sum_of_possible_tile_weights:
            (0..num_tiles)
                .map(|index| frequency_hints.relative_frequency(index))
                .sum(),
        sum_of_possible_tile_weight_log_weights:
            (0..num_tiles)
                .map(|index| {
                    let w = frequency_hints.relative_frequency(index) as f32;
                    return w * w.log2();
                })
                .sum(),
        entropy_noise: rand::thread_rng().gen_range(0_f64..0.0000001),
        is_collapsed: false,
        tile_enabler_counts:
            initial_tile_enabler_counts(num_tiles, &adjacency_rules),
    };

    let grid = Grid::new_clone(Size::new(output_size.0, output_size.1), cell_template);

    let mut entropy_heap = BinaryHeap::<EntropyCoord>::new();
    
    for (coord, cell) in grid.enumerate() {
        entropy_heap.push(
            EntropyCoord {
                entropy: N64::new(cell.entropy()),
                coord: coord,
            }
        )
    }
    
    let mut core_state = CoreState {
        grid,
        remaining_uncollapsed_cells: output_size.0 as usize * output_size.1 as usize,
        adjacency_rules,
        frequency_hints,
        entropy_heap: entropy_heap, // starts empty, why am i filling it wtf my algorithm is deadA
        tile_removals: Vec::new(),
    };


    core_state.run();

    let mut output_grid = Grid::new_copy(Size::new(output_size.0, output_size.1), 0);
    for (coord, cell) in core_state.grid.enumerate() {
        let tile_index = cell.get_only_possible_tile_index();
        let gridcell = output_grid.get_checked_mut(coord);
        *gridcell = tile_index;
    }

    return output_grid;
}