use std::u32;

use helper::{Day, Variants};
use rustc_hash::FxHashSet;

pub fn main() {
    helper::main::<Day12>(include_str!("../input.txt"));
}

struct Day12;

helper::define_variants! {
    day => crate::Day12;
    part1 {
        basic => crate::part1,sample_count=1000;
    }
    part2 {
        basic => crate::part2;
    }
}

impl Day for Day12 {
    fn part1() -> Variants {
        part1_variants!(construct_variants)
    }

    fn part2() -> Variants {
        part2_variants!(construct_variants)
    }
}

fn part1(input: &str) -> u64 {
    let width = input.bytes().position(|b| b == b'\n').unwrap() + 1;

    let mut regions = Vec::<FxHashSet<usize>>::new();
    let mut tile_exterior_edges = vec![None::<u32>; input.len()];

    let mut id = 0;

    for (i, c) in input.bytes().enumerate() {
        if c == b'\n' || tile_exterior_edges[i].is_some() {
            continue;
        }

        let region_id = id;
        id += 1;
        regions.push(FxHashSet::default());

        fn propagate(
            tile_exterior_edges: &mut [Option<u32>],
            regions: &mut Vec<FxHashSet<usize>>,
            input: &[u8],
            id: u32,
            i: usize,
            width: usize,
        ) {
            if tile_exterior_edges[i].is_some() {
                return;
            }
            tile_exterior_edges[i] = Some(u32::MAX);

            let mut exterior_edges = 0;

            regions[id as usize].insert(i);
            let c = input[i];
            if input.get(i + 1) == Some(&c) {
                propagate(tile_exterior_edges, regions, input, id, i + 1, width);
            } else {
                exterior_edges += 1;
            }
            if i > 0 && input[i - 1] == c {
                propagate(tile_exterior_edges, regions, input, id, i - 1, width);
            } else {
                exterior_edges += 1;
            }
            if input.get(i + width) == Some(&c) {
                propagate(tile_exterior_edges, regions, input, id, i + width, width);
            } else {
                exterior_edges += 1;
            }
            if i >= width && input[i - width] == c {
                propagate(tile_exterior_edges, regions, input, id, i - width, width);
            } else {
                exterior_edges += 1;
            }

            tile_exterior_edges[i] = Some(exterior_edges);
        }

        propagate(
            &mut tile_exterior_edges,
            &mut regions,
            input.as_bytes(),
            region_id,
            i,
            width,
        );
    }

    let mut total = 0;

    for region in regions {
        let area = region.len() as u64;
        let mut perimeter = 0;

        // To count the total perimeter, we just count all the non-interior edges of every tile.
        for tile in &region {
            perimeter += tile_exterior_edges[*tile].unwrap() as u64;
        }

        total += area * perimeter;
    }

    total
}

fn part2(input: &str) -> u64 {
    struct DontCount {
        top: bool,
        right: bool,
        bottom: bool,
        left: bool,
    }
    let width = input.bytes().position(|b| b == b'\n').unwrap() + 1;

    let mut regions = Vec::<FxHashSet<usize>>::new();
    let mut tile_exterior_edges = vec![None::<u32>; input.len()];

    let mut id = 0;

    for (i, c) in input.bytes().enumerate() {
        if c == b'\n' || tile_exterior_edges[i].is_some() {
            continue;
        }

        let region_id = id;
        id += 1;
        regions.push(FxHashSet::default());

        fn propagate(
            tile_exterior_edges: &mut [Option<u32>],
            regions: &mut Vec<FxHashSet<usize>>,
            input: &[u8],
            id: u32,
            i: usize,
            width: usize,
            dont_count: DontCount,
        ) {
            if tile_exterior_edges[i].is_some() {
                return;
            }
            tile_exterior_edges[i] = Some(u32::MAX);

            let mut exterior_edges = 0;

            regions[id as usize].insert(i);

            let c = input[i];

            let no_edge_top = i >= width && input[i - width] == c;
            let no_edge_right = input.get(i + 1) == Some(&c);
            let no_edge_bottom = input.get(i + width) == Some(&c);
            let no_edge_left = i > 0 && input[i - 1] == c;

            if no_edge_top {
                propagate(tile_exterior_edges, regions, input, id, i + 1, width, DontCount {
                    top: false,
                    bottom: false,
                });
            } else {
                exterior_edges += 1;
            }
            if no_edge_left {
                propagate(tile_exterior_edges, regions, input, id, i - 1, width);
            } else {
                exterior_edges += 1;
            }
            if no_edge_bottom {
                propagate(tile_exterior_edges, regions, input, id, i + width, width);
            } else {
                exterior_edges += 1;
            }
            if no_edge_top {
                propagate(tile_exterior_edges, regions, input, id, i - width, width);
            } else {
                exterior_edges += 1;
            }

            tile_exterior_edges[i] = Some(exterior_edges);
        }

        propagate(
            &mut tile_exterior_edges,
            &mut regions,
            input.as_bytes(),
            region_id,
            i,
            width,
            DontCount {
                top: false,
                right: false,
                bottom: false,
                left: false,
            },
        );
    }

    let mut total = 0;

    for region in regions {
        let area = region.len() as u64;
        let mut perimeter = 0;

        // To count the total perimeter, we just count all the non-interior edges of every tile.
        for tile in &region {
            perimeter += tile_exterior_edges[*tile].unwrap() as u64;
        }

        total += area * perimeter;
    }

    total
}

helper::tests! {
    day12 Day12;
    part1 {
        "../input_small.txt" => 140;
        "../input_small2.txt" => 772;
        "../input_small3.txt" => 1930;
        "../input.txt" => 0;
    }
    part2 {
        "../input_small.txt" => 80;
        "../input_small2.txt" => 436;
        "../input_small3.txt" => 1206;
        "../input.txt" => 0;
    }
}
helper::benchmarks! {}
