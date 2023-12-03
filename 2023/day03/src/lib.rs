mod p1;
mod p2basic;
mod p2bytes;
mod p2faster_hash;
mod p2less_branching;
mod p2with_capacity;

use helper::Day;

pub fn main() {
    helper::main::<Day03>(include_str!("../input.txt"));
}

struct Day03;

helper::define_variants! {
    day => crate::Day03;
    part1 {
        basic => crate::p1::basic;
        bytes => crate::p1::bytes;
    }
    part2 {
        basic => crate::p2basic::part2;
        faster_hash => crate::p2faster_hash::part2;
        with_capacity => crate::p2with_capacity::part2;
        less_branching => crate::p2less_branching::part2;
        bytes => crate::p2bytes::part2;
    }
}

impl Day for Day03 {
    fn part1() -> helper::Variants {
        part1_variants!(construct_variants)
    }

    fn part2() -> helper::Variants {
        part2_variants!(construct_variants)
    }
}

helper::tests! {
    day03 Day03;
    part1 {
        small => 4361;
        default => 537832;
    }
    part2 {
        small => 467835;
        default => 81939900;
    }
}

helper::benchmarks! {}
