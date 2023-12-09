use helper::{Day, Variants};

pub fn main() {
    helper::main::<Day09>(include_str!("../input.txt"));
}

struct Day09;

helper::define_variants! {
    day => crate::Day09;
    part1 {
        basic => crate::part1;
    }
    part2 {
        basic => crate::part2;
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

fn part1(input: &str) -> u64 {
    parse(input)
        .map(|mut values| {
            let mut last_values = vec![*values.last().unwrap()];

            let mut derive = values.clone();

            while !derive.iter().all(|&n| n == 0) {
                values.clear();
                values.extend(derive.windows(2).map(|s| s[1] - s[0]));

                last_values.push(*values.last().unwrap());

                let tmp = derive;
                derive = values;
                values = tmp;
            }

            last_values.into_iter().rev().sum::<i64>()
        })
        .sum::<i64>() as u64
}

fn part2(input: &str) -> u64 {
    parse(input)
        .map(|mut values| {
            let mut first_values = vec![*values.first().unwrap()];

            let mut derive = values.clone();

            while !derive.iter().all(|&n| n == 0) {
                values.clear();
                values.extend(derive.windows(2).map(|s| s[1] - s[0]));

                first_values.push(*values.first().unwrap());

                let tmp = derive;
                derive = values;
                values = tmp;
            }
            first_values
                .into_iter()
                .rev()
                .fold(0, |acc, first| first - acc)
        })
        .sum::<i64>() as u64
}

helper::tests! {
    day09 Day09;
    part1 {
        small => 114;
        default => 1934898178;
    }
    part2 {
        small => 0;
        default => 0;
    }
}
helper::benchmarks! {}
