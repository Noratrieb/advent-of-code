use helper::{parse_unwrap, Day, Variants};
use rustc_hash::FxHashMap;

pub fn main() {
    helper::main::<Day11>(include_str!("../input.txt"));
}

struct Day11;

helper::define_variants! {
    day => crate::Day11;
    part1 {
        basic => crate::part1;
    }
    part2 {
        basic => crate::part2;
    }
}

impl Day for Day11 {
    fn part1() -> Variants {
        part1_variants!(construct_variants)
    }

    fn part2() -> Variants {
        part2_variants!(construct_variants)
    }
}

fn part1(input: &str) -> u64 {
    let mut cache = FxHashMap::default();

    fn expanded_count(cache: &mut FxHashMap<(u64, u64), u32>, stone: u64, blinks: u64) -> u64 {
        let result_count = if blinks == 0 {
            1
        } else {
            if let Some(result) = cache.get(&(stone, blinks)) {
                return *result as u64;
            }

            if stone == 0 {
                expanded_count(cache, 1, blinks - 1)
            } else if (stone.ilog10() + 1) % 2 == 0 {
                //   3456 -> ilog=3, split =  100 (10^2)
                // 123456 -> ilog=5, split = 1000 (10^3)
                let split = 10_u64.pow((stone.ilog10() / 2) + 1);
                let lhs = stone / split;
                let rhs = stone % split;

                let lhs_count = expanded_count(cache, lhs, blinks - 1);
                let rhs_count = expanded_count(cache, rhs, blinks - 1);
                lhs_count + rhs_count
            } else {
                expanded_count(cache, stone * 2024, blinks - 1)
            }
        };

        cache.insert((stone, blinks), result_count.try_into().unwrap());

        result_count
    }

    let is_example = input.len() < 10;

    let total_blinks = if is_example { 6 } else { 25 };

    input
        .trim()
        .split(" ")
        .map(parse_unwrap)
        .map(|x| expanded_count(&mut cache, x, total_blinks))
        .sum()
}

fn part2(_input: &str) -> u64 {
    0
}

helper::tests! {
    day11 Day11;
    part1 {
        small => 22;
        default => 204022;
    }
    part2 {
        small => 0;
        default => 0;
    }
}
helper::benchmarks! {}
