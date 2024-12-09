use std::iter;

use helper::{Day, Variants};

pub fn main() {
    helper::main::<Day09>(include_str!("../input.txt"));
}

struct Day09;

helper::define_variants! {
    day => crate::Day09;
    part1 {
        basic => crate::part1,sample_count=1000;
    }
    part2 {
        basic => crate::part2,sample_count=1;
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
    let mut id = 0_u16;
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

fn part2(input: &str) -> u64 {
    #[derive(Clone, Copy, Debug)]
    enum Block {
        File { id: u16, size: u8 },
        Space { space_here: u8 },
    }
    #[expect(dead_code)]
    fn print_blocks(blocks: &[Block]) {
        for b in blocks {
            match b {
                Block::Space { .. } => eprint!("."),
                Block::File { id, .. } => eprint!("{}", id),
            }
        }
        eprintln!();
    }

    let input = &input[0..input.len() - 1];

    let mut blocks = Vec::<Block>::new();
    let mut is_file = true;
    let mut id = 0_u16;
    for b in input.bytes() {
        debug_assert!(b.is_ascii_digit());
        let len = (b - b'0') as usize;
        if is_file {
            blocks.extend(iter::repeat_n(
                Block::File {
                    id,
                    size: len as u8,
                },
                len,
            ));
            id += 1;
        } else {
            for i in (1..=len).rev() {
                blocks.push(Block::Space {
                    space_here: i as u8,
                });
            }
        }
        is_file = !is_file;
    }

    let mut i = blocks.len() - 1;

    while i > 0 {
        match blocks[i] {
            Block::File { id: _, size } => {
                let end = i + 1;

                // note: only check to the left.
                let insert_position = blocks[..i].iter().position(|b| match b {
                    Block::Space { space_here } => *space_here >= size,
                    _ => false,
                });
                let size_usize = size as usize;
                match insert_position {
                    Some(insert_position) => {
                        blocks.copy_within((end - size_usize)..end, insert_position);
                        blocks[(end - size_usize)..][..size_usize]
                            .fill(Block::Space { space_here: 0 }); // 0 does not matter
                        i = i.saturating_sub(size_usize);
                    }
                    None => {
                        i = i.saturating_sub(size_usize);
                    }
                }
            }
            Block::Space { .. } => {
                i = i.saturating_sub(1);
            }
        }
    }

    blocks
        .iter()
        .enumerate()
        .filter_map(|(i, tile)| match tile {
            Block::File { id, .. } => Some(*id as u64 * i as u64),
            _ => None,
        })
        .sum()
}

helper::tests! {
    day09 Day09;
    part1 {
        small => 1928;
        default => 6432869891895;
    }
    part2 {
        small => 2858;
        default => 0;
    }
}
helper::benchmarks! {}
