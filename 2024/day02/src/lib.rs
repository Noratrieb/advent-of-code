use helper::{parse_unwrap, Day, Variants};

pub fn main() {
    helper::main::<Day02>(include_str!("../input.txt"));
}

struct Day02;

helper::define_variants! {
    day => crate::Day02;
    part1 {
        basic => crate::part1;
    }
    part2 {
        basic => crate::part2;
        fast_parse => crate::part2_fast_parse;
    }
}

impl Day for Day02 {
    fn part1() -> Variants {
        part1_variants!(construct_variants)
    }

    fn part2() -> Variants {
        part2_variants!(construct_variants)
    }
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .filter(|report| {
            let levels = report
                .split_ascii_whitespace()
                .map(parse_unwrap)
                .collect::<Vec<_>>();

            let increasing = levels
                .windows(2)
                .all(|ab| ab[0] < ab[1] && ab[0] + 3 >= ab[1]);
            if increasing {
                return true;
            }
            let decreasing = levels
                .windows(2)
                .all(|ab| ab[1] < ab[0] && ab[1] + 3 >= ab[0]);

            decreasing
        })
        .count() as u64
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .filter(|report| {
            let levels = report
                .split_ascii_whitespace()
                .map(parse_unwrap)
                .collect::<Vec<_>>();

            let check_asc_desc = |forwards, smol, big| {
                if forwards {
                    smol < big && smol + 3 >= big
                } else {
                    big < smol && big + 3 >= smol
                }
            };

            let check_direction = |forwards| {
                let mut unsafe_transitions = vec![];
                for (i, ab) in levels.windows(2).enumerate() {
                    if !check_asc_desc(forwards, ab[0], ab[1]) {
                        unsafe_transitions.push(i);
                    }
                }
                match unsafe_transitions.len() {
                    0 => true,
                    1 => {
                        // 1 3 2 4 5
                        //    ^unsafe transition, index 1
                        // either idx 1 needs to go, or idx 2 needs to go (in this case 1)
                        let trans = unsafe_transitions[0];
                        if trans == 0 || trans == (levels.len() - 2) {
                            // It's the first or last element.
                            return true;
                        }

                        // Let's see what happens if we drop the first element (3 in the example).
                        if check_asc_desc(forwards, levels[trans - 1], levels[trans + 1]) {
                            // Dropping the first elem works!
                            return true;
                        }

                        // Let's see what happens if we drop the second element (2 in the example).
                        if check_asc_desc(forwards, levels[trans], levels[trans + 2]) {
                            // Dropping the secnd elem works!
                            return true;
                        }

                        false
                    }
                    2 => {
                        // 1 5 3 4 5
                        //  ^ ^ unsafe transitions, idx 0 and 1
                        // If the two transitions are not adjacent, there's no hope.
                        let (trans0, trans1) = (unsafe_transitions[0], unsafe_transitions[1]);
                        if trans0.abs_diff(trans1) != 1 {
                            return false;
                        }

                        // Let's try dropping the middle one.
                        let min = trans0.min(trans1);
                        if check_asc_desc(forwards, levels[min], levels[min + 2]) {
                            // Dropping it works!
                            return true;
                        }

                        false
                    }
                    _ => false,
                }
            };

            let forwards = check_direction(true);
            if forwards {
                return true;
            }
            check_direction(false)
        })
        .count() as u64
}

fn part2_fast_parse(input: &str) -> u64 {
    let mut levels = Vec::<u64>::new();

    let mut count = 0;

    let mut input = input.as_bytes();
    while let Some(end) = input.iter().position(|b| *b == b'\n') {
        let mut line = &input[..end];

        fn parse_digit(input: &[u8]) -> (u64, &[u8]) {
            let mut result = 0;
            let mut i = 0;
            while input.len() > i && input[i].is_ascii_digit() {
                result *= 10;
                result += (input[i] - b'0') as u64;
                i += 1;
            }
            (result, &input[i..])
        }

        let mut level;
        (level, line) = parse_digit(line);
        levels.push(level);

        while line.len() > 0 {
            line = &line[1..]; // space
            (level, line) = parse_digit(line);
            levels.push(level);
        }

        // Calculate
        let check_asc_desc = |forwards, smol, big| {
            if forwards {
                smol < big && smol + 3 >= big
            } else {
                big < smol && big + 3 >= smol
            }
        };

        let check_direction = |forwards| {
            let mut unsafe_transitions = vec![];
            for (i, ab) in levels.windows(2).enumerate() {
                if !check_asc_desc(forwards, ab[0], ab[1]) {
                    unsafe_transitions.push(i);
                }
            }
            match unsafe_transitions.len() {
                0 => true,
                1 => {
                    // 1 3 2 4 5
                    //    ^unsafe transition, index 1
                    // either idx 1 needs to go, or idx 2 needs to go (in this case 1)
                    let trans = unsafe_transitions[0];
                    if trans == 0 || trans == (levels.len() - 2) {
                        // It's the first or last element.
                        return true;
                    }

                    // Let's see what happens if we drop the first element (3 in the example).
                    if check_asc_desc(forwards, levels[trans - 1], levels[trans + 1]) {
                        // Dropping the first elem works!
                        return true;
                    }

                    // Let's see what happens if we drop the second element (2 in the example).
                    if check_asc_desc(forwards, levels[trans], levels[trans + 2]) {
                        // Dropping the secnd elem works!
                        return true;
                    }

                    false
                }
                2 => {
                    // 1 5 3 4 5
                    //  ^ ^ unsafe transitions, idx 0 and 1
                    // If the two transitions are not adjacent, there's no hope.
                    let (trans0, trans1) = (unsafe_transitions[0], unsafe_transitions[1]);
                    if trans0.abs_diff(trans1) != 1 {
                        return false;
                    }

                    // Let's try dropping the middle one.
                    let min = trans0.min(trans1);
                    if check_asc_desc(forwards, levels[min], levels[min + 2]) {
                        // Dropping it works!
                        return true;
                    }

                    false
                }
                _ => false,
            }
        };

        if check_direction(true) || check_direction(false) {
            count += 1;
        }

        levels.clear();
        input = &input[(end + 1)..];
    }

    count
}

helper::tests! {
    day02 Day02;
    part1 {
        small => 2;
        default => 287;
    }
    part2 {
        small => 4;
        default => 354;
    }
}
helper::benchmarks! {}
