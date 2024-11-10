use helper::{parse_unwrap, Day, Variants};

pub fn main() {
    helper::main::<Day01>(include_str!("../input.txt"));
}

struct Day01;

helper::define_variants! {
    day => crate::Day01;
    part1 {
        basic => crate::part1;
    }
    part2 {
        basic => crate::part2;
    }
}

impl Day for Day01 {
    fn part1() -> Variants {
        part1_variants!(construct_variants)
    }

    fn part2() -> Variants {
        part2_variants!(construct_variants)
    }
}

fn part1(input: &str) -> u64 {
    input
        .trim()
        .split("\n\n")
        .map(|elf| elf.trim().split("\n").map(parse_unwrap).sum())
        .max()
        .unwrap()
}

fn part2(input: &str) -> u64 {
    let mut all = input
        .trim()
        .split("\n\n")
        .map(|elf| elf.trim().split("\n").map(parse_unwrap).sum())
        .collect::<Vec<u64>>();
    all.sort();
    all[(all.len() - 3)..].iter().copied().sum::<u64>()
}

helper::tests! {
    day01 Day01;
    part1 {
        small => 24000;
        default => 70698;
    }
    part2 {
        small => 45000;
        default => 206643;
    }
}
helper::benchmarks! {}
