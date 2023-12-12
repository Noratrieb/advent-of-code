use std::cmp::{max, min};

use helper::IteratorExt;

use crate::Universe;

fn height_width(universe: &Universe, count: u64) -> (Vec<u64>, Vec<u64>) {
    let rows = universe
        .rows
        .iter()
        .map(|row| if row.iter().all(|b| !b) { count } else { 1 })
        .collect_vec();

    let cols = (0..universe.rows[0].len())
        .map(|col| {
            if universe.rows.iter().all(|row| !row[col]) {
                count
            } else {
                1
            }
        })
        .collect_vec();

    (rows, cols)
}

fn solve(input: &str, count: u64) -> u64 {
    let universe = super::parse(input);
    let (row_height, col_width) = height_width(&universe, count);

    let galaxies = universe
        .rows
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .copied()
                .enumerate()
                .filter(|&(_, b)| b)
                .map(move |(j, _)| (i, j))
        })
        .collect_vec();

    let mut sum = 0;

    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let lhs = galaxies[i];
            let rhs = galaxies[j];

            let x_range = min(lhs.1, rhs.1)..max(lhs.1, rhs.1);
            let distance_x = x_range.map(|x| col_width[x]).sum::<u64>();

            let y_range = min(lhs.0, rhs.0)..max(lhs.0, rhs.0);
            let distance_y = y_range.map(|x| row_height[x]).sum::<u64>();

            sum += distance_x + distance_y;
        }
    }

    sum as u64
}

pub fn part1(input: &str) -> u64 {
    solve(input, 2)
}

pub fn part2(input: &str) -> u64 {
    solve(input, 1_000_000)
}
