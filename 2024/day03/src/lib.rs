use helper::{Day, Variants};
use nom::FindSubstring;

pub fn main() {
    helper::main::<Day03>(include_str!("../input.txt"));
}

struct Day03;

helper::define_variants! {
    day => crate::Day03;
    part1 {
        basic => crate::part1;
    }
    part2 {
        basic => crate::part2;
    }
}

impl Day for Day03 {
    fn part1() -> Variants {
        part1_variants!(construct_variants)
    }

    fn part2() -> Variants {
        part2_variants!(construct_variants)
    }
}

fn part1(input: &str) -> u64 {
    let mut input = input.as_bytes();
    let mut total = 0;

    fn parse_digit(input: &[u8]) -> Option<(u64, &[u8])> {
        let mut result = 0;
        let mut i = 0;
        while input.len() > i && input[i].is_ascii_digit() {
            result *= 10;
            result += (input[i] - b'0') as u64;
            i += 1;
        }
        if i == 0 {
            return None;
        }
        Some((result, &input[i..]))
    }

    fn try_parse_mul(mut input: &[u8]) -> Option<(&[u8], u64)> {
        input = &input.get(4..)?;
        let (a, b);
        (a, input) = parse_digit(input)?;
        if input.get(0) != Some(&b',') {
            return None;
        }
        input = &input[1..];
        (b, input) = parse_digit(input)?;
        if input.get(0) != Some(&b')') {
            return None;
        }
        input = &input[1..];
        Some((input, a * b))
    }

    while let Some(next) = input.find_substring("mul(") {
        match try_parse_mul(&input[next..]) {
            Some((new_input, val)) => {
                total += val;
                input = new_input;
            }
            None => input = &input[(next+4)..]
        }
    }

    total
}

fn part2(_input: &str) -> u64 {
    0
}

helper::tests! {
    day03 Day03;
    part1 {
        small => 161;
        default => 187833789;
    }
    part2 {
        small => 0;
        default => 0;
    }
}
helper::benchmarks! {}
