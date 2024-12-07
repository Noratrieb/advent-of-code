use helper::{Day, Variants};

pub fn main() {
    helper::main::<Day06>(include_str!("../input.txt"));
}

struct Day06;

helper::define_variants! {
    day => crate::Day06;
    part1 {
        basic => crate::part1;
    }
    part2 {
        basic => crate::part2;
    }
}

impl Day for Day06 {
    fn part1() -> Variants {
        part1_variants!(construct_variants)
    }

    fn part2() -> Variants {
        part2_variants!(construct_variants)
    }
}

fn part1(input: &str) -> u64 {
    let input = input.as_bytes();

    let width = input.iter().position(|&byte| byte == b'\n').unwrap() + 1; // account for newline
    let mut guard_pos = input
        .iter()
        .position(|&byte| byte == b'^' || byte == b'>' || byte == b'<' || byte == b'v')
        .unwrap();
    let mut guard_state = input[guard_pos];

    let mut reached_tiles = vec![false; input.len()];
    reached_tiles[guard_pos] = true;

    loop {
        let new_pos = match guard_state {
            b'^' => guard_pos.checked_sub(width),
            b'v' => guard_pos.checked_add(width),
            b'<' => guard_pos.checked_sub(1),
            b'>' => guard_pos.checked_add(1),
            _ => unreachable!(),
        };
        match new_pos {
            None => {
                break;
            }
            Some(new_pos) if input.len() <= new_pos => {
                break;
            }
            Some(new_pos) if input[new_pos] == b'\n' => {
                break;
            }
            Some(new_pos) => {
                if input[new_pos] == b'#' {
                    guard_state = match guard_state {
                        b'^' => b'>',
                        b'v' => b'<',
                        b'<' => b'^',
                        b'>' => b'v',
                        _ => unreachable!(),
                    }
                } else {
                    reached_tiles[new_pos] = true;
                    guard_pos = new_pos;
                }
            }
        }
    }

    reached_tiles.iter().filter(|reached| **reached).count() as u64
}

fn part2(_input: &str) -> u64 {
    0
}

helper::tests! {
    day06 Day06;
    part1 {
        small => 41;
        default => 4454;
    }
    part2 {
        small => 0;
        default => 0;
    }
}
helper::benchmarks! {}
