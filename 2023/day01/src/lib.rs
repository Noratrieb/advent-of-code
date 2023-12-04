use std::mem::MaybeUninit;

use helper::{Day, Variants};

mod branchless;
mod naive;
mod no_lines;
mod vectorized;
mod zero_alloc;

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
        naive => crate::naive::part2, sample_count=1000;
        zero_alloc => crate::zero_alloc::part2;
        branchless => |i| unsafe { crate::branchless::part2(i) };
        no_lines => |i| unsafe { crate::no_lines::part2(i) };
        vectorized => |i| unsafe { crate::vectorized::part2(i) };
    }
}

impl Day for Day01 {
    fn pad_input(input: &str) -> std::borrow::Cow<str> {
        let mut input = input.to_owned();
        input.reserve(10); // enough to read u64
        unsafe {
            input
                .as_mut_vec()
                .spare_capacity_mut()
                .fill(MaybeUninit::new(0))
        };
        std::borrow::Cow::Owned(input)
    }
    fn part1() -> Variants {
        part1_variants!(construct_variants)
    }

    fn part2() -> Variants {
        part2_variants!(construct_variants)
    }
}

fn part1(input: &str) -> u64 {
    let sum = input
        .lines()
        .map(|line| {
            let mut chars = line.chars().filter(|c| c.is_ascii_digit());
            let first = chars.next().unwrap();
            let last = chars.next_back().unwrap_or(first);

            [first, last]
                .into_iter()
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .sum::<u64>();

    sum
}

helper::tests! {
    day01 Day01;
    part1 {
        "../input_small1.txt" => 142;
        "../input.txt" => 54632;
    }
    part2 {
        "../input_small2.txt" => 281;
        "../input.txt" => 54019;
    }
}

helper::benchmarks! {}
