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
        brute_force => crate::part2, sample_count=2;
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
    #[repr(u8)]
    enum GuardState {
        Up,
        Down,
        Left,
        Right,
    }

    let input = input.as_bytes();

    let width = input.iter().position(|&byte| byte == b'\n').unwrap() + 1; // account for newline
    let mut guard_pos = input
        .iter()
        .position(|&byte| byte == b'^' || byte == b'>' || byte == b'<' || byte == b'v')
        .unwrap();
    let mut guard_state = match input[guard_pos] {
        b'^' => GuardState::Up,
        b'v' => GuardState::Down,
        b'<' => GuardState::Left,
        b'>' => GuardState::Right,
        _ => unreachable!(),
    };

    let mut reached_tiles = vec![false; input.len()];
    reached_tiles[guard_pos] = true;

    loop {
        let new_pos = match guard_state {
            GuardState::Up => guard_pos.checked_sub(width),
            GuardState::Down => guard_pos.checked_add(width),
            GuardState::Left => guard_pos.checked_sub(1),
            GuardState::Right => guard_pos.checked_add(1),
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
                        GuardState::Up => GuardState::Right,
                        GuardState::Down => GuardState::Left,
                        GuardState::Left => GuardState::Up,
                        GuardState::Right => GuardState::Down,
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

fn part2(input: &str) -> u64 {
    #[derive(Clone, Copy)]
    #[repr(u8)]
    enum GuardState {
        Up,
        Down,
        Left,
        Right,
    }

    let input = input.as_bytes();

    fn check_if_stuck(input: &[u8], mut guard_pos: usize, width: usize) -> bool {
        let mut guard_state = match input[guard_pos] {
            b'^' => GuardState::Up,
            b'v' => GuardState::Down,
            b'<' => GuardState::Left,
            b'>' => GuardState::Right,
            _ => unreachable!(),
        };

        let mut reached_tiles = vec![false; input.len() * 4];
        let idx_reached =
            |guard_pos: usize, guard_state: GuardState| (guard_pos << 2) | guard_state as usize;

        loop {
            if reached_tiles[idx_reached(guard_pos, guard_state)] {
                return true;
            }
            reached_tiles[idx_reached(guard_pos, guard_state)] = true;

            let new_pos = match guard_state {
                GuardState::Up => guard_pos.checked_sub(width),
                GuardState::Down => guard_pos.checked_add(width),
                GuardState::Left => guard_pos.checked_sub(1),
                GuardState::Right => guard_pos.checked_add(1),
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
                            GuardState::Up => GuardState::Right,
                            GuardState::Down => GuardState::Left,
                            GuardState::Left => GuardState::Up,
                            GuardState::Right => GuardState::Down,
                        };
                    } else {
                        guard_pos = new_pos;
                    }
                }
            }
        }

        false
    }

    let initial_guard_pos = input
        .iter()
        .position(|&byte| byte == b'^' || byte == b'>' || byte == b'<' || byte == b'v')
        .unwrap();
    let width = input.iter().position(|&byte| byte == b'\n').unwrap() + 1; // account for newline

    let mut count = 0;
    let mut modified_input = input.to_owned();
    for i in 0..input.len() {
        if input[i] == b'\n' || input[i] == b'#' || i == initial_guard_pos {
            continue;
        }
        let prev = modified_input[i];

        modified_input[i] = b'#';
        if check_if_stuck(&modified_input, initial_guard_pos, width) {
            count += 1;
        }

        modified_input[i] = prev;
    }

    count
}

helper::tests! {
    day06 Day06;
    part1 {
        small => 41;
        default => 4454;
    }
    part2 {
        small => 6;
        default => 1503;
    }
}
helper::benchmarks! {}
