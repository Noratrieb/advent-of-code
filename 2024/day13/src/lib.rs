use helper::{Day, Variants};

pub fn main() {
    helper::main::<Day13>(include_str!("../input.txt"));
}

struct Day13;

helper::define_variants! {
    day => crate::Day13;
    part1 {
        basic => crate::part1;
    }
    part2 {
        basic => crate::part2;
    }
}

impl Day for Day13 {
    fn part1() -> Variants {
        part1_variants!(construct_variants)
    }

    fn part2() -> Variants {
        part2_variants!(construct_variants)
    }
}

fn part1(_input: &str) -> u64 {
    0
}

fn part2(_input: &str) -> u64 {
    0
}

helper::tests! {
    day13 Day13;
    part1 {
        small => 0;
        default => 0;
    }
    part2 {
        small => 0;
        default => 0;
    }
}
helper::benchmarks! {}
