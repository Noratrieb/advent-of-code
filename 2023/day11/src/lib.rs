mod faster;
mod p1slow;

use std::fmt::Debug;

use helper::{Day, IteratorExt, Variants};

pub fn main() {
    helper::main::<Day11>(include_str!("../input.txt"));
}

struct Day11;

helper::define_variants! {
    day => crate::Day11;
    part1 {
        basic => crate::p1slow::part1;
        faster => crate::faster::part1, sample_count=200;
    }
    part2 {
        basic => crate::faster::part2, sample_count=200;
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

helper::tests! {
    day11 Day11;
    part1 {
        small => 374;
        default => 9370588;
    }
    part2 {
        small => 82000210;
        default => 746207878188;
    }
}
helper::benchmarks! {}
