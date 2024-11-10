#![feature(iter_array_chunks)]

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
        basic => crate::part2_basic;
        no_alloc => crate::part2_no_alloc;
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

fn part2_basic(input: &str) -> u64 {
    let mut all = input
        .trim()
        .split("\n\n")
        .map(|elf| elf.trim().split("\n").map(parse_unwrap).sum())
        .collect::<Vec<u64>>();
    all.sort();
    all[(all.len() - 3)..].iter().copied().sum::<u64>()
}


fn part2_no_alloc(input: &str) -> u64 {
    let mut biggest = [0; 3];

    let elves = input
        .trim()
        .split("\n\n")
        .map(|elf| elf.trim().split("\n").map(parse_unwrap).sum::<u64>());

    for elf in elves {
        if elf > biggest[0] {
            if elf > biggest[1] {
                if elf > biggest[2] {
                    biggest[0] = biggest[1];
                    biggest[1] = biggest[2];
                    biggest[2] = elf;
                } else {
                    biggest[0] = biggest[1];
                    biggest[1] = elf;
                }
            } else {
                biggest[0] = elf;
            }
        }
    }

    biggest[0] + biggest[1] + biggest[2]
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
