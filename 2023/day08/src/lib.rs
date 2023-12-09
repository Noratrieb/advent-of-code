#![allow(unused)]

mod p1basic;
mod p2basic;

use helper::{Day, Variants};

pub fn main() {
    helper::main::<Day08>(include_str!("../input.txt"));
}

struct Day08;

helper::define_variants! {
    day => crate::Day08;
    part1 {
        basic => crate::p1basic::part1;
    }
    part2 {
        basic => crate::p2basic::part2;
    }
}

impl Day for Day08 {
    fn part1() -> Variants {
        part1_variants!(construct_variants)
    }

    fn part2() -> Variants {
        part2_variants!(construct_variants)
    }
}


helper::tests! {
    day08 Day08;
    part1 {
        "../input_small1.txt" => 6;
        "../input.txt" => 12361;
    }
    part2 {
        "../input_small2.txt" => 6;
        "../input.txt" => 6 /* TODO */;
    }
}

helper::benchmarks! {}
