mod p2cache;

use helper::{Day, Variants};

pub fn main() {
    helper::main::<Day04>(include_str!("../input.txt"));
}

struct Day04;

helper::define_variants! {
    day => crate::Day04;
    part1 {
        basic => crate::part1, sample_count=1000;
    }
    part2 {
        basic => crate::part2, sample_count=100;
        cache => crate::p2cache::part2, sample_count=1000;
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

fn line_match_count(line: &str) -> usize {
    let mut numbers = line.split(':').nth(1).unwrap().split("|");
    let winning = numbers
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>();
    let you_have = numbers.next().unwrap().split_whitespace();

    you_have
        .filter(|have| winning.iter().any(|w| w == have))
        .count()
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let win_amount = line_match_count(line);

            if win_amount > 0 {
                1 << (win_amount - 1)
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let lines = input.lines().map(line_match_count).collect::<Vec<_>>();
    let mut processed = 0;

    let mut todo = (0..lines.len()).collect::<Vec<_>>();

    while let Some(line) = todo.pop() {
        let matches = lines[line];
        todo.extend((line + 1)..(line + 1 + matches));
        processed += 1;
    }

    processed
}

helper::tests! {
    day04 Day04;
    part1 {
        small => 13;
        default => 24733;
    }
    part2 {
        small => 30;
        default => 5422730;
    }
}
helper::benchmarks! {}
