use std::iter;

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

fn part1(input: &str) -> u64 {
    let input = &input[0..input.len() - 1];

    let mut blocks = Vec::new();
    let mut is_file = true;
    let mut id = 0;
    for b in input.bytes() {
        debug_assert!(b.is_ascii_digit());
        let len = (b - b'0') as usize;
        if is_file {
            blocks.extend(iter::repeat_n(Some(id), len));
            id += 1;
        } else {
            blocks.extend(iter::repeat_n(None, len));
        }
        is_file = !is_file;
    }

    let mut next_insert_idx = blocks.iter().position(|b| b.is_none()).unwrap();
    for i in (0..blocks.len()).rev() {
        if next_insert_idx >= i {
            break;
        }

        match blocks[i] {
            Some(block) => {
                blocks[next_insert_idx] = Some(block);
                blocks[i] = None;
                next_insert_idx = next_insert_idx
                    + blocks[next_insert_idx..]
                        .iter()
                        .position(|b| b.is_none())
                        .unwrap();
            }
            None => {}
        }
    }

    blocks
        .iter()
        .enumerate()
        .filter_map(|(i, b)| b.map(|b| b as u64 * i as u64))
        .sum()
}

fn part2(_input: &str) -> u64 {
    0
}

helper::tests! {
    day09 Day09;
    part1 {
        small => 1928;
        default => 6432869891895;
    }
    part2 {
        small => 0;
        default => 0;
    }
}
helper::benchmarks! {}
