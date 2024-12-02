use helper::{parse_unwrap, Day, Variants};

pub fn main() {
    helper::main::<Day02>(include_str!("../input.txt"));
}

struct Day02;

helper::define_variants! {
    day => crate::Day02;
    part1 {
        basic => crate::part1;
    }
    part2 {
        basic => crate::part2;
    }
}

impl Day for Day02 {
    fn part1() -> Variants {
        part1_variants!(construct_variants)
    }

    fn part2() -> Variants {
        part2_variants!(construct_variants)
    }
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .filter(|report| {
            let levels = report
                .split_ascii_whitespace()
                .map(parse_unwrap)
                .collect::<Vec<_>>();

            let increasing = levels
                .windows(2)
                .all(|ab| ab[0] < ab[1] && ab[0] + 3 >= ab[1]);
            if increasing {
                return true;
            }
            let decreasing = levels
                .windows(2)
                .all(|ab| ab[1] < ab[0] && ab[1] + 3 >= ab[0]);

            decreasing
        })
        .count() as u64
}

fn part2(_input: &str) -> u64 {
    0
}

helper::tests! {
    day02 Day02;
    part1 {
        small => 2;
        default => 287;
    }
    part2 {
        small => 0;
        default => 0;
    }
}
helper::benchmarks! {}
