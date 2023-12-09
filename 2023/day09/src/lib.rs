mod less_alloc;

use std::ops::Add;

use helper::{Day, Variants};

pub fn main() {
    helper::main::<Day09>(include_str!("../input.txt"));
}

struct Day09;

helper::define_variants! {
    day => crate::Day09;
    part1 {
        basic => crate::part1;
        less_alloc => crate::less_alloc::part1;
    }
    part2 {
        basic => crate::part2;
        less_alloc => crate::less_alloc::part2;
    }
}

impl Day for Day09 {
    fn part1() -> Variants {
        part1_variants!(construct_variants)
    }

    fn part2() -> Variants {
        part2_variants!(construct_variants)
    }
}

fn parse(input: &str) -> impl Iterator<Item = Vec<i64>> + '_ {
    input.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect()
    })
}

fn execute(
    input: impl Iterator<Item = Vec<i64>>,
    last_or_first: impl Fn(&[i64]) -> i64,
    fold: impl Fn(i64, i64) -> i64 + Copy,
) -> i64 {
    input
        .map(|mut values| {
            let mut last_values = vec![last_or_first(&values)];

            let mut derive = values.clone();

            while !derive.iter().all(|&n| n == 0) {
                values.clear();
                values.extend(derive.windows(2).map(|s| s[1] - s[0]));

                last_values.push(last_or_first(&values));

                let tmp = derive;
                derive = values;
                values = tmp;
            }

            last_values.into_iter().rev().fold(0, fold)
        })
        .sum::<i64>()
}

fn part1(input: &str) -> u64 {
    execute(parse(input), |s| *s.last().unwrap(), Add::add) as u64
}

fn part2(input: &str) -> u64 {
    execute(parse(input), |s| *s.first().unwrap(), |a, b| b - a) as u64
}

helper::tests! {
    day09 Day09;
    part1 {
        small => 114;
        default => 1934898178;
    }
    part2 {
        small => 2;
        default => 1129;
    }
}
helper::benchmarks! {}
