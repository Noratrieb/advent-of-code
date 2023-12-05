use helper::{Day, IteratorExt, Variants};

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

#[derive(Debug)]
struct MappedRange {
    dest_start: u64,
    source_start: u64,
    len: u64,
}

fn parse_maps<'a>(mut lines: impl Iterator<Item = &'a str>) -> Vec<Vec<MappedRange>> {
    let mut maps = vec![];
    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }

        if line.contains(" map") {
            maps.push(Vec::new());
            continue;
        }

        let numbers = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect_array::<3>()
            .unwrap();

        maps.last_mut().unwrap().push(MappedRange {
            dest_start: numbers[0],
            source_start: numbers[1],
            len: numbers[2],
        });
    }

    maps
}

fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| n.parse::<u64>().unwrap());

    let maps = parse_maps(lines);

    let mut min_loc = u64::MAX;

    for mut seed in seeds {
        for ranges in &maps {
            match ranges.iter().find(|range| {
                (range.source_start..(range.source_start + range.len)).contains(&seed)
            }) {
                Some(range) => {
                    let offset = seed - range.source_start;
                    let new = range.dest_start + offset;
                    seed = new;
                }
                None => {}
            }
        }

        min_loc = std::cmp::min(min_loc, seed);
    }

    min_loc
}

fn part2(_input: &str) -> u64 {
    0
}

helper::tests! {
    day05 Day05;
    part1 {
        small => 35;
        default => 457535844;
    }
    part2 {
        small => 0;
        default => 0;
    }
}
helper::benchmarks! {}
