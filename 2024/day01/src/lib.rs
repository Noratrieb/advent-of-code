use core::str;
use std::{collections::HashMap, hash::BuildHasherDefault};

use helper::{Day, IteratorExt, NoHasher, Variants};
use rustc_hash::FxHashMap;

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
        basic => crate::part2;
        hash => crate::part2_hash;
        hash_no_hash => crate::part2_hash_nohash;
        faster_parsing => crate::part2_parsing;
        array => crate::part2_array;
        μopt_parsing => crate::part2_μopt_parsing;
        bytes => crate::part2_bytes;
        assume_len => crate::part2_assume_len;
        simd => crate::part2_simd;
    }
}

impl Day for Day01 {
    fn part1() -> Variants {
        part1_variants!(construct_variants)
    }

    fn part2() -> Variants {
        part2_variants!(construct_variants)
    }
}

fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|part| part.parse::<u64>().unwrap())
                .collect_array::<2>()
                .unwrap()
        })
        .map(|[a, b]| (a, b))
        .unzip()
}

fn part1(input: &str) -> u64 {
    let (mut a, mut b) = parse(input);

    a.sort();
    b.sort();

    a.into_iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum()
}

fn part2(input: &str) -> u64 {
    let (left, right) = parse(input);

    let mut score = 0;

    for number in left {
        let occurs = right.iter().filter(|right| **right == number).count();
        score += number * (occurs as u64);
    }

    score
}

fn part2_hash(input: &str) -> u64 {
    let (left, right) = parse(input);

    let mut right_map = FxHashMap::<u64, u32>::default();
    for number in right {
        *right_map.entry(number).or_default() += 1;
    }

    let mut score = 0;

    for number in left {
        let occurs = right_map.get(&number).copied().unwrap_or_default();
        score += number * (occurs as u64);
    }

    score
}

fn part2_hash_nohash(input: &str) -> u64 {
    let (left, right) = parse(input);

    let mut right_map = HashMap::<u64, u32, BuildHasherDefault<NoHasher>>::default();
    for number in right {
        *right_map.entry(number).or_default() += 1;
    }

    let mut score = 0;

    for number in left {
        let occurs = right_map.get(&number).copied().unwrap_or_default();
        score += number * (occurs as u64);
    }

    score
}

fn part2_parsing(input: &str) -> u64 {
    let mut right_map = FxHashMap::<u64, u32>::with_capacity_and_hasher(
        input.len() / 8,
        rustc_hash::FxBuildHasher::default(),
    );
    let mut left = Vec::with_capacity(input.len() / 8);

    let mut input = input;
    loop {
        let Some(space) = input.as_bytes().iter().position(|b| *b == b' ') else {
            break;
        };
        let number = input[..space].parse::<u64>().unwrap();
        left.push(number);
        input = &input[(space + 3)..];
        let Some(newline) = input.as_bytes().iter().position(|b| *b == b'\n') else {
            break;
        };
        let number = input[..space].parse::<u64>().unwrap();
        *right_map.entry(number).or_default() += 1;
        input = &input[newline..];
        input = &input[1..];
    }

    let mut score = 0;

    for number in left {
        let occurs = right_map.get(&number).copied().unwrap_or_default();
        score += number * (occurs as u64);
    }

    score
}

fn part2_array(input: &str) -> u64 {
    const BIGGEST_ELEMENT: usize = 100_000;
    let mut right_map = vec![0_u16; BIGGEST_ELEMENT];
    let mut left = Vec::with_capacity(input.len() / 8);

    let mut input = input;
    loop {
        let Some(space) = input.as_bytes().iter().position(|b| *b == b' ') else {
            break;
        };
        let number = input[..space].parse::<u64>().unwrap();
        left.push(number);
        input = &input[(space + 3)..];
        let Some(newline) = input.as_bytes().iter().position(|b| *b == b'\n') else {
            break;
        };
        let number = input[..space].parse::<u64>().unwrap();
        right_map[number as usize] += 1;
        input = &input[newline..];

        input = &input[1..];
    }

    let mut score = 0;

    for number in left {
        let occurs = right_map[number as usize];
        score += number * (occurs as u64);
    }

    score
}

fn part2_μopt_parsing(input: &str) -> u64 {
    assert_eq!(input.as_bytes().last(), Some(&b'\n'));

    const BIGGEST_ELEMENT: usize = 100_000;
    let mut right_map = vec![0_u16; BIGGEST_ELEMENT];
    let mut left = Vec::with_capacity(input.len());

    let digit_len = input.as_bytes().iter().position(|b| *b == b' ').unwrap();
    let line_len = 2 * digit_len + 3 + 1;

    fn parse_digit(input: &str, len: usize) -> u64 {
        let mut result = 0;
        for i in 0..len {
            result *= 10;
            result += (input.as_bytes()[i] - b'0') as u64;
        }
        result
    }

    let mut input = input;
    while input.len() >= line_len {
        let number = parse_digit(input, digit_len);

        left.push(number as u32);
        input = &input[(digit_len + 3)..];

        let number = parse_digit(input, digit_len);
        right_map[number as usize] += 1;
        input = &input[digit_len..];
        input = &input[1..];
    }

    let mut score = 0;

    for number in left {
        let occurs = right_map[number as usize];
        score += (number as u64) * (occurs as u64);
    }

    score
}

fn part2_bytes(input: &str) -> u64 {
    let input = input.as_bytes();
    assert_eq!(input.last(), Some(&b'\n'));

    const BIGGEST_ELEMENT: usize = 100_000;
    let mut right_map = vec![0_u16; BIGGEST_ELEMENT];
    let mut left = Vec::<u32>::with_capacity(input.len());

    let digit_len = input.iter().position(|b| *b == b' ').unwrap();
    let line_len = 2 * digit_len + 3 + 1;

    fn parse_digit(input: &[u8], len: usize) -> u32 {
        let mut result = 0;
        for i in 0..len {
            result *= 10;
            result += (unsafe { input.get_unchecked(i) } - b'0') as u32;
        }
        result
    }

    let mut input = input;
    while input.len() >= line_len {
        let number = parse_digit(input, digit_len);

        left.push(number);
        input = unsafe { &input.get_unchecked((digit_len + 3)..) };

        let number = parse_digit(input, digit_len);
        right_map[number as usize] += 1;
        input = unsafe { &input.get_unchecked((digit_len + 1)..) };
    }

    let mut score = 0;

    for number in left {
        let occurs = right_map[number as usize];
        score += number * (occurs as u32);
    }

    score as u64
}

fn part2_assume_len(input_str: &str) -> u64 {
    let input = input_str.as_bytes();
    assert_eq!(input.last(), Some(&b'\n'));

    const BIGGEST_ELEMENT: usize = 100_000;
    let mut right_map = vec![0_u16; BIGGEST_ELEMENT];
    let mut left = Vec::<u32>::with_capacity(input.len());

    let digit_len = input.iter().position(|b| *b == b' ').unwrap();
    let line_len = 2 * digit_len + 3 + 1;

    if digit_len != 5 {
        return part2_bytes(input_str);
    }

    fn parse_digit(input: &[u8]) -> u32 {
        let mut result = 0;
        for i in 0..5 {
            result *= 10;
            result += (unsafe { input.get_unchecked(i) } - b'0') as u32;
        }
        result
    }

    let mut input = input;
    while input.len() >= line_len {
        let number = parse_digit(input);

        left.push(number);
        input = unsafe { &input.get_unchecked((digit_len + 3)..) };

        let number = parse_digit(input);
        right_map[number as usize] += 1;
        input = unsafe { &input.get_unchecked((digit_len + 1)..) };
    }

    let mut score = 0;

    for number in left {
        let occurs = right_map[number as usize];
        score += number * (occurs as u32);
    }

    score as u64
}

fn part2_simd(input_str: &str) -> u64 {
    let input = input_str.as_bytes();
    assert_eq!(input.last(), Some(&b'\n'));

    let digit_len = input.iter().position(|b| *b == b' ').unwrap();

    if digit_len != 5 {
        return part2_bytes(input_str);
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        return part2_assume_len(input_str);
    }
    #[cfg(target_arch = "x86_64")]
    {
        if !std::arch::is_x86_feature_detected!("sse4.1") {
            return part2_assume_len(input_str);
        }
        return do_sse41(input);
    }
    #[cfg(target_arch = "x86_64")]
    pub fn do_sse41(input: &[u8]) -> u64 {
        const DIGIT_LEN: usize = 5;
        const BIGGEST_ELEMENT: usize = 100_000;

        let mut right_map = vec![0_u16; BIGGEST_ELEMENT];
        let mut left = Vec::<u32>::with_capacity(input.len());

        let line_len = 2 * DIGIT_LEN + 3 + 1;

        #[target_feature(enable = "sse4.1")]
        unsafe fn parse_digit(input: &[u8]) -> u32 {
            use std::arch::x86_64;
            let vector: [u8; 4] = input.as_ptr().add(1).cast::<[u8; 4]>().read();
            let digits = std::mem::transmute([
                vector[0], vector[1], vector[2], vector[3], 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]);
            let numbers = x86_64::_mm_sub_epi8(digits, x86_64::_mm_set1_epi8(b'0' as i8));
            let numbers_wide = x86_64::_mm_cvtepu8_epi16(numbers);
            let factors = x86_64::_mm_set_epi16(0, 0, 0, 0, 1, 10, 100, 1000);
            let parts = x86_64::_mm_mullo_epi16(numbers_wide, factors);
            let parts_array = std::mem::transmute::<_, [u16; 8]>(parts);

            let high = (input.get_unchecked(0) - b'0') as u32 * 10_000;

            let low = parts_array[0] + parts_array[1] + parts_array[2] + parts_array[3];

            let result = high + low as u32;

            if cfg!(debug_assertions) {
                let naive = str::from_utf8(&input[..DIGIT_LEN])
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();
                assert_eq!(result, naive);
            }

            result
        }

        let mut input = input;
        while input.len() >= line_len {
            let number = unsafe { parse_digit(input) };

            left.push(number);
            input = unsafe { &input.get_unchecked((DIGIT_LEN + 3)..) };

            let number = unsafe { parse_digit(input) };
            right_map[number as usize] += 1;
            input = unsafe { &input.get_unchecked((DIGIT_LEN + 1)..) };
        }

        let mut score = 0;

        for number in left {
            let occurs = right_map[number as usize];
            score += number * (occurs as u32);
        }

        score as u64
    }
}

helper::tests! {
    day01 Day01;
    part1 {
        small => 11;
        default => 2580760;
    }
    part2 {
        small => 31;
        default => 25358365;
    }
}
helper::benchmarks! {}
