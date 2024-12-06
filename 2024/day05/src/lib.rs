use helper::{parse_unwrap, Day, IteratorExt, Variants};

pub fn main() {
    helper::main::<Day05>(include_str!("../input.txt"));
}

struct Day05;

helper::define_variants! {
    day => crate::Day05;
    part1 {
        basic => crate::part1;
    }
    part2 {
        basic => crate::part2;
    }
}

impl Day for Day05 {
    fn part1() -> Variants {
        part1_variants!(construct_variants)
    }

    fn part2() -> Variants {
        part2_variants!(construct_variants)
    }
}

fn part1(input: &str) -> u64 {
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let values = line
            .split('|')
            .collect_array::<2>()
            .unwrap()
            .map(parse_unwrap);
        rules.push((values[0], values[1]));
    }
    while let Some(line) = lines.next() {
        let numbers = line.split(",").map(parse_unwrap).collect::<Vec<_>>();
        updates.push(numbers);
    }

    0
}

fn part2(_input: &str) -> u64 {
    0
}

helper::tests! {
    day05 Day05;
    part1 {
        small => 143;
        default => 0;
    }
    part2 {
        small => 0;
        default => 0;
    }
}
helper::benchmarks! {}
