use helper::{Day, Variants};

pub fn main() {
    helper::main::<Day04>(include_str!("../input.txt"));
}

struct Day04;

helper::define_variants! {
    day => crate::Day04;
    part1 {
        basic => crate::part1;
    }
    part2 {
        basic => crate::part2;
    }
}

impl Day for Day04 {
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
        .map(|line| {
            let mut numbers = line.split(':').nth(1).unwrap().split("|");
            let winning = numbers
                .next()
                .unwrap()
                .split_whitespace()
                .collect::<Vec<_>>();
            let you_have = numbers.next().unwrap().split_whitespace();

            let win_amount = you_have
                .filter(|have| winning.iter().any(|w| w == have))
                .count();

            if win_amount > 0 {
                1 << (win_amount - 1)
            } else {
                0
            }
        })
        .sum()
}

fn part2(_input: &str) -> u64 {
    0
}

helper::tests! {
    day04 Day04;
    part1 {
        small => 13;
        default => 24733;
    }
    part2 {
        small => 0;
        default => 0;
    }
}
helper::benchmarks! {}
