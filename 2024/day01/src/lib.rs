use helper::{Day, IteratorExt, Variants};

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

fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|part| part.parse::<u64>().unwrap())
                .collect_array::<2>()
                .unwrap()
        })
        .map(|[a, b]| (a, b))
        .unzip()
}

fn part1(input: &str) -> u64 {
    let (mut a, mut b) = parse(input);

    a.sort();
    b.sort();

    a.into_iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum()
}

fn part2(input: &str) -> u64 {
    let (left, right) = parse(input);

    let mut score = 0;

    for number in left {
        let occurs = right.iter().filter(|right| **right == number).count();
        score += number * (occurs as u64);
    }

    score
}

helper::tests! {
    day01 Day01;
    part1 {
        small => 11;
        default => 2580760;
    }
    part2 {
        small => 31;
        default => 25358365;
    }
}
helper::benchmarks! {}
