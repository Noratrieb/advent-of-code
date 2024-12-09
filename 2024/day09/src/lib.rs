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
        index => crate::part2_index,sample_count=1000;
        index_index => crate::part2_index_index;
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
    use std::iter;

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
    use std::iter;

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

fn part2_index(input: &str) -> u64 {
    use std::iter;

    #[derive(Clone, Copy, Debug)]
    enum Block {
        File { id: u16, size: u8 },
        Space,
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

    let mut index = [const { Vec::<usize>::new() }; 10];

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
            if len > 0 {
                index[len].push(blocks.len());
            }
            blocks.extend(iter::repeat_n(Block::Space, len));
        }
        is_file = !is_file;
    }

    index.iter_mut().for_each(|v| v.reverse());

    let mut i = blocks.len() - 1;

    // print_blocks(&blocks);

    while i > 0 {
        match blocks[i] {
            Block::File { id: _, size } => {
                let end = i + 1;
                let size_usize = size as usize;

                let possible_gaps = &mut index[size_usize..];
                let gap = possible_gaps
                    .iter_mut()
                    .enumerate()
                    .filter(|(_, gaps)| gaps.last().is_some_and(|gap| *gap < i))
                    .min_by_key(|(_, gaps)| *gaps.last().unwrap());

                match gap {
                    Some((gap_size, chosen_gap)) => {
                        let gap_size = gap_size + size_usize;
                        let chosen_gap = chosen_gap.pop().unwrap();

                        blocks.copy_within((end - size_usize)..end, chosen_gap);
                        blocks[(end - size_usize)..][..size_usize].fill(Block::Space);
                        i = i.saturating_sub(size_usize);

                        // add gap back to index
                        if gap_size > size_usize {
                            let remaining_size = gap_size - size_usize;
                            let remaining_index = chosen_gap + size_usize;
                            let bucket = &mut index[remaining_size];
                            let back_offset_insert =
                                bucket.iter().rev().position(|gap| *gap > remaining_index);
                            match back_offset_insert {
                                Some(back_offset_insert) => {
                                    let len = bucket.len();
                                    bucket.insert(len - back_offset_insert, remaining_index);
                                }
                                None => bucket.push(remaining_index),
                            }
                        }

                        // print_blocks(&blocks);
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
    // print_blocks(&blocks);

    blocks
        .iter()
        .enumerate()
        .filter_map(|(i, tile)| match tile {
            Block::File { id, .. } => Some(*id as u64 * i as u64),
            _ => None,
        })
        .sum()
}

fn part2_index_index(input: &str) -> u64 {
    use std::iter;

    #[derive(Clone, Copy, Debug)]
    enum Block {
        File { id: u16, size: u8 },
        Space,
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

    let mut index = [const { Vec::<usize>::new() }; 10];

    let mut blocks = Vec::<Block>::with_capacity(input.len());
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
            if len > 0 {
                index[len].push(blocks.len());
            }
            blocks.extend(iter::repeat_n(Block::Space, len));
        }
        is_file = !is_file;
    }

    index.iter_mut().for_each(|v| v.reverse());

    let mut min_index = [None::<(usize, usize)>; 10];
    min_index[min_index.len() - 1] = index[min_index.len() - 1]
        .last()
        .copied()
        .map(|gap| (index.len() - 1, gap));
    for i in (1..(min_index.len() - 1)).rev() {
        min_index[i] = match (index[i].last().copied(), min_index[i + 1]) {
            (None, None) => None,
            (Some(x), None) => Some((i, x)),
            (None, Some(x)) => Some(x),
            (Some(x), Some(y)) => {
                if x < y.1 {
                    Some((i, x))
                } else {
                    Some(y)
                }
            }
        }
    }

    fn verify_min_index(index: &[Vec<usize>], min_index: &[Option<(usize, usize)>]) {
        for bucket in 1..10 {
            let actual_min_bucket = index[bucket..]
                .iter()
                .enumerate()
                .filter_map(|(i, gaps)| gaps.last().map(|gap| (i + bucket, *gap)))
                .min_by_key(|(_, gap)| *gap);
            assert_eq!(min_index[bucket], actual_min_bucket, "for {bucket}");
        }
    }

    let mut i = blocks.len() - 1;

    // print_blocks(&blocks);

    while i > 0 {
        if cfg!(debug_assertions) {
            verify_min_index(&index, &min_index);
        }

        match blocks[i] {
            Block::File { id: _, size } => {
                let end = i + 1;
                let size_usize = size as usize;

                let gap = min_index[size_usize];

                match gap {
                    Some((_, chosen_gap)) if chosen_gap > i => {
                        i = i.saturating_sub(size_usize);
                    }
                    Some((gap_size, chosen_gap)) => {
                        let poppped_gap_offset = index[gap_size].pop().unwrap();
                        debug_assert_eq!(poppped_gap_offset, chosen_gap);

                        blocks.copy_within((end - size_usize)..end, chosen_gap);
                        blocks[(end - size_usize)..][..size_usize].fill(Block::Space);
                        i = i.saturating_sub(size_usize);

                        // add gap back to index
                        if gap_size > size_usize {
                            let remaining_size = gap_size - size_usize;
                            let remaining_index = chosen_gap + size_usize;
                            let bucket = &mut index[remaining_size];
                            let back_offset_insert =
                                bucket.iter().rev().position(|gap| *gap > remaining_index);
                            match back_offset_insert {
                                Some(back_offset_insert) => {
                                    let len = bucket.len();
                                    bucket.insert(len - back_offset_insert, remaining_index);
                                }
                                None => bucket.push(remaining_index),
                            }
                        }

                        // recompute min_index
                        if gap_size < index.len() - 1 {
                            min_index[gap_size] =
                                match (index[gap_size].last(), min_index[gap_size + 1]) {
                                    (None, None) => None,
                                    (Some(x), None) => Some((gap_size, *x)),
                                    (None, Some(x)) => Some(x),
                                    (Some(x), Some(y)) => {
                                        if *x < y.1 {
                                            Some((gap_size, *x))
                                        } else {
                                            Some(y)
                                        }
                                    }
                                }
                        } else {
                            min_index[gap_size] =
                                index[gap_size].last().map(|gap| (gap_size, *gap));
                        }
                        for i in (1..gap_size).rev() {
                            min_index[i] = match (index[i].last(), min_index[i + 1]) {
                                (None, None) => None,
                                (Some(x), None) => Some((i, *x)),
                                (None, Some(y)) => Some(y),
                                (Some(x), Some(y)) => {
                                    if *x < y.1 {
                                        Some((i, *x))
                                    } else {
                                        Some(y)
                                    }
                                }
                            }
                        }

                        // print_blocks(&blocks);
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

    // print_blocks(&blocks);

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
        default => 6467290479134;
    }
}
helper::benchmarks! {}
