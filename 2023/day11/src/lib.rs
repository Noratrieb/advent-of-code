use std::fmt::Debug;

use helper::{Day, IteratorExt, Variants};

pub fn main() {
    helper::main::<Day11>(include_str!("../input.txt"));
}

struct Day11;

helper::define_variants! {
    day => crate::Day11;
    part1 {
        basic => crate::part1;
    }
    part2 {
        basic => crate::part2;
    }
}

impl Day for Day11 {
    fn part1() -> Variants {
        part1_variants!(construct_variants)
    }

    fn part2() -> Variants {
        part2_variants!(construct_variants)
    }
}

struct Universe {
    rows: Vec<Vec<bool>>,
}

impl Debug for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in &self.rows {
            for &col in row {
                write!(f, "{}", if col { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse(input: &str) -> Universe {
    Universe {
        rows: input
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect_vec())
            .collect_vec(),
    }
}

fn expand(universe: Universe) -> Universe {
    let mut rows = universe
        .rows
        .into_iter()
        .flat_map(|row| {
            if row.iter().all(|b| !b) {
                vec![row.clone(), row]
            } else {
                vec![row]
            }
        })
        .collect_vec();

    let mut col = 0;
    while col < rows[0].len() {
        if rows.iter().all(|row| !row[col]) {
            rows.iter_mut().for_each(|row| row.insert(col, false));
            col += 1;
        }
        col += 1;
    }

    Universe { rows }
}

fn part1(input: &str) -> u64 {
    let universe = parse(input);
    let universe = expand(universe);

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

            let distance_x = ((lhs.1 as i64) - (rhs.1 as i64)).abs();
            let distance_y = ((lhs.0 as i64) - (rhs.0 as i64)).abs();

            sum += distance_x + distance_y;
        }
    }

    sum as u64
}

fn part2(_input: &str) -> u64 {
    0
}

helper::tests! {
    day11 Day11;
    part1 {
        small => 374;
        default => 0;
    }
    part2 {
        small => 0;
        default => 0;
    }
}
helper::benchmarks! {}
