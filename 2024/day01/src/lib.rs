#![feature(portable_simd)]
#![feature(
    slice_swap_unchecked,
    generic_const_exprs,
    iter_array_chunks,
    get_many_mut,
    maybe_uninit_uninit_array,
    iter_collect_into,
    let_chains,
    anonymous_lifetime_in_impl_trait,
    array_windows,
    slice_take,
    test,
    slice_as_chunks,
    array_chunks,
    slice_split_once,
    core_intrinsics
)]#![feature(hint_assert_unchecked)]
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
        //basic => crate::part2;
        //hash => crate::part2_hash;
        //hash_no_hash => crate::part2_hash_nohash;
        //faster_parsing => crate::part2_parsing;
        //array => crate::part2_array;
        //μopt_parsing => crate::part2_μopt_parsing;
        //bytes => crate::part2_bytes;
        //assume_len => crate::part2_assume_len;
        nora => crate::part2_simd;
        clubby => crate::part2_clubby;
        part2_max397 => crate::part2_max397;
        bendn => crate::part2_bendn;
        symmetriccats => crate::p2;
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
    const DIGIT_LEN: usize = 5;
    let input = input_str.as_bytes();
    assert_eq!(input.last(), Some(&b'\n'));

    const BIGGEST_ELEMENT: usize = 100_000;

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

pub fn part2_clubby(input: &str) -> u64 {
    use std::{
        hint::assert_unchecked,
        simd::{num::SimdUint, LaneCount, Simd, SupportedLaneCount},
    };

    fn for_each_line<F>(input: &str, mut f: F)
    where
        F: FnMut(u64, u64),
    {
        let mut input = input.as_bytes();
        let line_length = memchr::memchr(b'\n', input).unwrap();
        // Length of first column
        let first_col_len = memchr::memchr(b' ', &input[..line_length]).unwrap();
        // SAFETY: `memchr` returns a value less than the length
        unsafe { assert_unchecked(first_col_len < line_length) };

        // Offset from start to second column
        let second_col_offset = memchr::memrchr(b' ', &input[..line_length]).unwrap() + 1;
        // SAFETY: `memchr` returns a value less than the length
        unsafe { assert_unchecked(second_col_offset < line_length) };
        assert!(second_col_offset > first_col_len);

        while !input.is_empty() {
            assert!(input.len() > line_length);

            let (num1, num2) =
                parse_line_simd(input, first_col_len, second_col_offset, line_length);
            f(num1, num2);
            input = &input[line_length + 1..];
        }
    }

    fn parse_line_simd(
        input: &[u8],
        first_col_len: usize,
        second_col_offset: usize,
        line_length: usize,
    ) -> (u64, u64) {
        assert!(input.len() >= second_col_offset);
        assert!(input.len() >= first_col_len);
        assert!(input.len() >= line_length);

        match (first_col_len, second_col_offset, line_length) {
            (5, 8, 13) => (
                simd_parse_start::<8, 5>(input[..second_col_offset].try_into().unwrap()),
                simd_parse_end::<8, 3>(input[first_col_len..line_length].try_into().unwrap()),
            ),
            (1, 4, 5) => (
                simd_parse_start::<4, 1>(input[..second_col_offset].try_into().unwrap()),
                simd_parse_end::<4, 3>(input[first_col_len..line_length].try_into().unwrap()),
            ),
            _ => unimplemented!(),
        }
    }

    fn simd_parse_start<const INP_LEN: usize, const NUM_SIZE: usize>(line: &[u8; INP_LEN]) -> u64
    where
        LaneCount<INP_LEN>: SupportedLaneCount,
    {
        let multipliers = Simd::from(std::array::from_fn(|i| 10u64.pow(i as u32))).reverse();
        let mask = Simd::from(std::array::from_fn(
            |i| if i < NUM_SIZE { u64::MAX } else { 0 },
        ));
        let line = Simd::<u8, INP_LEN>::load_or_default(line);
        let digits = line - Simd::splat(b'0');
        let digits: Simd<u64, INP_LEN> = digits.cast();
        let digits = digits & mask;
        (digits * multipliers).reduce_sum() / 10u64.pow((INP_LEN - NUM_SIZE) as u32)
    }

    fn simd_parse_end<const INP_LEN: usize, const GAP_LEN: usize>(line: &[u8; INP_LEN]) -> u64
    where
        LaneCount<INP_LEN>: SupportedLaneCount,
    {
        let multipliers = Simd::from(std::array::from_fn(|i| 10u64.pow(i as u32))).reverse();
        let mask = Simd::from(std::array::from_fn(
            |i| if i >= GAP_LEN { u64::MAX } else { 0 },
        ));
        let line = Simd::<u8, INP_LEN>::load_or_default(line);
        let digits = line - Simd::splat(b'0');
        let digits: Simd<u64, INP_LEN> = digits.cast();
        let digits = digits & mask;
        (digits * multipliers).reduce_sum()
    }

    let mut num_counts = vec![0u16; 99999];
    let line_length = memchr::memchr(b'\n', input.as_bytes()).unwrap();
    let lines = input.len() / line_length;
    let mut appeared = Vec::with_capacity(lines);

    for_each_line(input, |num1, num2| {
        appeared.push(num1);
        num_counts[num2 as usize] += 1;
    });
    appeared
        .iter()
        .map(|&num| num * num_counts[num as usize] as u64)
        .sum()
}

fn part2_max397(input: &str) -> u64 {
    use bstr::ByteSlice;

    pub fn part2(input: &str) -> u64 {
        let input = input.as_bytes();
        let mut col1: Vec<usize> = Vec::with_capacity(1000);
        let mut counts: [u16; 100000] = [0; 100000];
        let idx1 = unsafe { input.find_byte(b' ').unwrap_unchecked() };
        let idx2 = idx1 + 3;
        let idx3 = idx1 + idx2;
        input.lines().for_each(|line| {
            col1.push(line[0..idx1].as_num());
            counts[line[idx2..idx3].as_num::<usize>()] += 1;
        });

        col1.iter()
            .map(|num| num * counts[*num] as usize)
            .sum::<usize>() as u64
    }
    // the parsing
    use num_traits::PrimInt;

    pub trait ByteParsing {
        fn as_num<T: PrimInt>(&self) -> T;
    }

    impl ByteParsing for [u8] {
        fn as_num<T: PrimInt>(&self) -> T {
            let mut out = T::zero();
            for byte in self {
                out = out * T::from(10).unwrap() + T::from(byte - b'0').unwrap();
            }
            out
        }
    }

    part2(input)
}


pub fn p2(input: &str) -> u64 {
    let (a, b) = parse_input_fast(input).unwrap();
 
    let seen = a.iter().copied().collect::<rustc_hash::FxHashSet<_>>();
    b.iter()
        .fold(0, |acc, &val| acc + val * seen.contains(&val) as u32) as u64
}
 
pub fn parse_input_fast(input: &str) -> Result<(Vec<u32>, Vec<u32>), Box<dyn std::error::Error>> {
    const LINE_LENGTH: usize = 14;
 
    let elements = input.len() / LINE_LENGTH;
    let mut x = vec![0; elements];
    let mut y = vec![0; elements];
 
    for line in input.as_bytes().chunks_exact(LINE_LENGTH) {
        let a0 = (line[0] - b'0') as u32 * 10_000;
        let a1 = (line[1] - b'0') as u32 * 1_000;
        let a2 = (line[2] - b'0') as u32 * 100;
        let a3 = (line[3] - b'0') as u32 * 10;
        let a4 = (line[4] - b'0') as u32 * 1;
        let a = a0 + a1 + a2 + a3 + a4;
 
        let b0 = (line[8] - b'0') as u32 * 10_000;
        let b1 = (line[9] - b'0') as u32 * 1_000;
        let b2 = (line[10] - b'0') as u32 * 100;
        let b3 = (line[11] - b'0') as u32 * 10;
        let b4 = (line[12] - b'0') as u32 * 1;
        let b = b0 + b1 + b2 + b3 + b4;
 
        x.push(a);
        y.push(b);
    }
 
    Ok((x, y))
}
 

pub fn part2_bendn(i: &str) -> u64 {
    use util::Widen;
    extern crate test;
    mod util {
        #![allow(non_snake_case, unused_macros)]

        use rustc_hash::FxHashMap as HashMap;
        use rustc_hash::FxHashSet as HashSet;
        use std::{
            cmp::Reverse,
            collections::{hash_map::Entry, BinaryHeap},
            fmt::{Debug, Display, Write},
            hash::Hash,
            mem::swap,
            ops::RangeInclusive,
            str::FromStr,
        };

        pub mod prelude {
            #[allow(unused_imports)]
            pub(crate) use super::{bits, dang, leek, mat, shucks, C};
            pub use super::{
                even, gcd, gt, l, lcm, lt, pa, r, rand, reading, reading::Ext, sort, Dir, FilterBy,
                FilterBy3, GreekTools, IntoCombinations, IntoLines, IterͶ, NumTupleIterTools,
                ParseIter, Printable, Skip, TakeLine, TupleIterTools2, TupleIterTools2R,
                TupleIterTools3, TupleUtils, UnifiedTupleUtils, UnsoundUtilities, Widen, Ͷ, Α, Κ,
                Λ, Μ,
            };
            pub use itertools::izip;
            pub use itertools::Itertools;
            pub use rustc_hash::FxHashMap as HashMap;
            pub use rustc_hash::FxHashSet as HashSet;
            pub use std::{
                cmp::Ordering::*,
                cmp::{max, min},
                collections::{hash_map::Entry, VecDeque},
                fmt::{Debug, Display},
                hint::black_box as boxd,
                io::{self, Read, Write},
                iter,
                mem::{replace as rplc, swap, transmute as rint},
                ops::Range,
            };
        }

        macro_rules! C {
    ($obj:ident.$what:ident$($tt:tt)+) => {{
        let x = &mut $obj.$what;
        C!( x$($tt)+ )
    }};
    (&$buf:ident[$n:expr]) => {{
        #[allow(unused_unsafe)]
        unsafe {
            $buf.get_unchecked($n)
        }
    }};
    ($buf:ident[$n:expr]) => {{
        #[allow(unused_unsafe)]
        *unsafe {
            $buf.get_unchecked($n)
        }
    }};
    (&mut $buf:ident[$n:expr]) => {{
        #[allow(unused_unsafe)]
        unsafe {
            $buf.get_unchecked_mut($n)
        }
    }};
    ($buf:ident[$a:expr] = $rbuf:ident[$b:expr]) => {
        *unsafe { $buf.get_unchecked_mut($a) } = unsafe { *$rbuf.get_unchecked($b) }
    };
    ($buf:ident[$n:expr] = $e:expr) => {
        *unsafe { $buf.get_unchecked_mut($n) } = $e
    };
    ($buf:ident[$a:expr][$b:expr]) => {
        unsafe { *$buf.get_unchecked($a).get_unchecked($b) }
    };
    ($buf:ident[$a:expr][$b:expr] = $rbuf:ident[$ra:expr]) => {
        *unsafe { $buf.get_unchecked_mut($a).get_unchecked_mut($b) } =
            unsafe { *$rbuf.get_unchecked($ra) }
    };
    ($buf:ident[$a:expr][$b:expr] = $rbuf:ident[$ra:expr][$rb:expr]) => {
        *unsafe { $buf.get_unchecked_mut($a).get_unchecked_mut($b) } =
            unsafe { *$rbuf.get_unchecked($ra).get_unchecked($rb) }
    };
    ($buf:ident[$a:expr][$b:expr] = $c:expr) => {{
        #[allow(unused_unsafe)]
        {
            *unsafe { $buf.get_unchecked_mut($a).get_unchecked_mut($b) } = unsafe { $c }
        }
    }};
}
        pub(crate) use C;

        macro_rules! shucks {
    () => {
        if cfg!(debug_assertions) {
            unreachable!();
        } else {
            unsafe { std::hint::unreachable_unchecked() }
        }
    };
    ($fmt:literal $(, $args:expr)* $(,)?) => {
        if cfg!(debug_assertions) {
            unreachable!($fmt $(, $args)*);
        } else {
            unsafe { std::hint::unreachable_unchecked() }
        }
    };
    (if $x:expr) => {
        if $x {
            if cfg!(debug_assertions) {
                unreachable!();
            } else {
                unsafe { std::hint::unreachable_unchecked() }
            }
        }
    };
}
        pub(crate) use shucks;

        macro_rules! dang {
            () => {
                panic!()
            };
        }
        pub(crate) use dang;

        macro_rules! leek {
    ($($allocation:ident)+) => {
        $(std::mem::forget($allocation);)+
    };
}
        pub(crate) use leek;

        macro_rules! mat {
    ($thing:ident { $($what:pat => $b:expr,)+ }) => {
        match $thing { $($what => { $b })+ _ => shucks!() }
    };
}
        pub(crate) use mat;

        #[cfg(target_feature = "avx2")]
        unsafe fn count_avx<const N: usize>(hay: &[u8; N], needle: u8) -> usize {
            use std::arch::x86_64::*;
            let find = _mm256_set1_epi8(needle as i8);
            let mut counts = _mm256_setzero_si256();
            for i in 0..(N / 32) {
                counts = _mm256_sub_epi8(
                    counts,
                    _mm256_cmpeq_epi8(
                        _mm256_loadu_si256(hay.as_ptr().add(i * 32) as *const _),
                        find,
                    ),
                );
            }
            const MASK: [u8; 64] = [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                255, 255,
            ];
            counts = _mm256_sub_epi8(
                counts,
                _mm256_and_si256(
                    _mm256_cmpeq_epi8(
                        _mm256_loadu_si256(hay.as_ptr().add(N - 32) as *const _),
                        find,
                    ),
                    _mm256_loadu_si256(MASK.as_ptr().add(N % 32) as *const _),
                ),
            );

            let sums = _mm256_sad_epu8(counts, _mm256_setzero_si256());
            (_mm256_extract_epi64(sums, 0)
                + _mm256_extract_epi64(sums, 1)
                + _mm256_extract_epi64(sums, 2)
                + _mm256_extract_epi64(sums, 3)) as usize
        }

        pub fn count<const N: usize>(hay: &[u8; N], what: u8) -> usize {
            #[cfg(target_feature = "avx2")]
            return unsafe { count_avx(hay, what) };
            #[cfg(not(target_feature = "avx2"))]
            hay.iter().filter(|&&x| x == what).count()
        }

        pub fn lcm(n: impl IntoIterator<Item = u64>) -> u64 {
            let mut x = n.into_iter();
            let mut lcm = x.by_ref().next().expect("cannot compute LCM of 0 numbers");
            let mut g;
            for x in x {
                g = gcd(x, lcm);
                lcm = (lcm * x) / g;
            }
            lcm
        }

        #[repr(u8)]
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
        pub enum Dir {
            N = b'U',
            E = b'R',
            S = b'D',
            W = b'L',
        }

        pub trait UnsoundUtilities<T> {
            fn ψ(self) -> T;
        }

        impl<T> UnsoundUtilities<T> for Option<T> {
            fn ψ(self) -> T {
                if cfg!(debug_assertions) && self.is_none() {
                    panic!();
                }
                unsafe { self.unwrap_unchecked() }
            }
        }

        impl<T, E> UnsoundUtilities<T> for Result<T, E> {
            #[cfg_attr(debug_assertions, track_caller)]
            fn ψ(self) -> T {
                if cfg!(debug_assertions) && self.is_err() {
                    panic!();
                }
                unsafe { self.unwrap_unchecked() }
            }
        }

        pub struct LMap<K, V, F>(HashMap<K, V>, F)
        where
            F: Fn(K) -> Option<V>,
            K: Eq + Hash + Copy;
        impl<K: Eq + Hash + Copy, V, F> LMap<K, V, F>
        where
            F: Fn(K) -> Option<V>,
        {
            pub fn new(f: F) -> Self {
                Self {
                    0: HashMap::default(),
                    1: f,
                }
            }

            pub fn get(&mut self, k: K) -> Option<&mut V> {
                match self.0.entry(k) {
                    Entry::Occupied(x) => Some(x.into_mut()),
                    Entry::Vacant(e) => match self.1(k) {
                        Some(v) => Some(e.insert(v)),
                        None => None,
                    },
                }
            }
        }

        pub fn countg<N: Debug + PartialEq + Hash + Eq + Copy, I: Iterator<Item = N>>(
            start: N,
            graph: &mut impl Fn(N) -> I,
            sum: &mut usize,
            end: &mut impl Fn(N) -> bool,
            has: &mut HashSet<N>,
        ) {
            if end(start) {
                *sum += 1;
            } else {
                graph(start)
                    .map(|x| {
                        if has.insert(x) {
                            countg(x, graph, sum, end, has);
                        }
                    })
                    .Θ();
            }
        }

        // pub fn appearances(x: )

        pub fn iterg<N: Debug + Copy, I: Iterator<Item = N>>(
            start: N,
            graph: &mut impl Fn(N) -> I,
            end: &mut impl Fn(N) -> bool,
            finally: &mut impl FnMut(N),
        ) {
            if end(start) {
                finally(start);
            } else {
                graph(start).map(|x| iterg(x, graph, end, finally)).Θ();
            };
        }

        pub fn show<N: Debug + Eq + Hash + Copy + Ord, I: Iterator<Item = (N, u16)>, D: Display>(
            graph: impl Fn(N) -> I,
            start: N,
            end: impl Fn(N) -> bool,
            name: impl Fn(N) -> D,
        ) {
            println!("digraph {{");
            let mut s = HashSet::default();
            let mut q = BinaryHeap::new();
            q.push(Reverse((0, start)));
            while let Some(Reverse((c, n))) = q.pop() {
                if end(n) {
                    println!("}}");
                    return;
                }
                if !s.insert(n) {
                    continue;
                }
                print!("\t{}", name(n));
                for (n, d) in graph(n) {
                    if s.contains(&n) {
                        continue;
                    }
                    print!(" -> {}", name(n));
                    q.push(Reverse((c + d, n)));
                }
                println!(";");
            }
            dang!();
        }

        pub fn dijkstra_h<N: Debug + Eq + Hash + Copy + Ord, I: Iterator<Item = (N, u16)>>(
            graph: impl Fn(N) -> I,
            start: N,
            end: impl Fn(N) -> bool,
            h: impl Fn(N) -> u16,
        ) -> u16 {
            let mut q = BinaryHeap::new();
            let mut s = HashSet::default();
            q.push(Reverse((h(start), 0, start)));
            while let Some(Reverse((_, c, n))) = q.pop() {
                if end(n) {
                    return c;
                }
                if !s.insert(n) {
                    continue;
                }
                for (n, d) in graph(n) {
                    if s.contains(&n) {
                        continue;
                    }
                    q.push(Reverse((h(n) + c + d, c + d, n)));
                }
            }
            dang!()
        }

        pub fn dijkstra<N: Debug + Eq + Hash + Copy + Ord, I: Iterator<Item = (N, u16)>>(
            graph: impl Fn(N) -> I,
            start: N,
            end: impl Fn(N) -> bool,
        ) -> u16 {
            let mut q = BinaryHeap::new();
            let mut s = HashSet::default();
            q.push(Reverse((0, start)));
            while let Some(Reverse((c, n))) = q.pop() {
                if end(n) {
                    return c;
                }
                if !s.insert(n) {
                    continue;
                }
                for (n, d) in graph(n) {
                    if s.contains(&n) {
                        continue;
                    }
                    q.push(Reverse((c + d, n)));
                }
            }
            dang!()
        }

        impl std::ops::Add<(i64, i64)> for Dir {
            type Output = (i64, i64);
            fn add(self, (x, y): (i64, i64)) -> Self::Output {
                match self {
                    Dir::N => (x, y - 1),
                    Dir::E => (x + 1, y),
                    Dir::S => (x, y + 1),
                    Dir::W => (x - 1, y),
                }
            }
        }

        impl std::ops::Add<(i32, i32)> for Dir {
            type Output = (i32, i32);
            fn add(self, (x, y): (i32, i32)) -> Self::Output {
                match self {
                    Dir::N => (x, y - 1),
                    Dir::E => (x + 1, y),
                    Dir::S => (x, y + 1),
                    Dir::W => (x - 1, y),
                }
            }
        }

        impl std::ops::Add<(u16, u16)> for Dir {
            type Output = (u16, u16);

            fn add(self, (x, y): (u16, u16)) -> Self::Output {
                match self {
                    Dir::N => (x, y - 1),
                    Dir::E => (x + 1, y),
                    Dir::S => (x, y + 1),
                    Dir::W => (x - 1, y),
                }
            }
        }

        impl std::ops::Add<(i16, i16)> for Dir {
            type Output = (i16, i16);
            fn add(self, (x, y): (i16, i16)) -> Self::Output {
                match self {
                    Dir::N => (x, y - 1),
                    Dir::E => (x + 1, y),
                    Dir::S => (x, y + 1),
                    Dir::W => (x - 1, y),
                }
            }
        }

        impl std::ops::Add<(u8, u8)> for Dir {
            type Output = Option<(u8, u8)>;

            fn add(self, (x, y): (u8, u8)) -> Self::Output {
                match self {
                    Dir::N => Some((x, y.checked_sub(1)?)),
                    Dir::E => Some((x + 1, y)),
                    Dir::S => Some((x, y + 1)),
                    Dir::W => Some((x.checked_sub(1)?, y)),
                }
            }
        }

        pub fn pa<T: std::fmt::Debug>(a: &[T]) {
            for e in a {
                print!("{e:?}");
            }
            println!();
        }

        pub fn gcd(mut a: u64, mut b: u64) -> u64 {
            if a == 0 || b == 0 {
                return a | b;
            }
            let shift = (a | b).trailing_zeros();
            a >>= shift;
            loop {
                b >>= b.trailing_zeros();
                if a > b {
                    swap(&mut a, &mut b);
                }
                b -= a;
                if b == 0 {
                    break;
                }
            }
            a << shift
        }

        pub trait Λ {
            fn λ<T: FromStr>(&self) -> T
            where
                <T as FromStr>::Err: std::fmt::Display;
        }

        impl Λ for String {
            fn λ<T: FromStr>(&self) -> T
            where
                <T as FromStr>::Err: std::fmt::Display,
            {
                self.as_str().λ()
            }
        }
        impl Λ for &[u8] {
            fn λ<T: FromStr>(&self) -> T
            where
                <T as FromStr>::Err: std::fmt::Display,
            {
                std::str::from_utf8(self).α().λ()
            }
        }
        impl Λ for &str {
            /// parse, unwrap
            fn λ<T: FromStr>(&self) -> T
            where
                <T as FromStr>::Err: std::fmt::Display,
            {
                match self.parse() {
                    Ok(v) => v,
                    Err(e) => {
                        panic!(
                            "{e}: {self} should parse into {}",
                            std::any::type_name::<T>()
                        )
                    }
                }
            }
        }
        pub trait Κ {
            fn κ<T: FromStr>(self) -> impl Iterator<Item = T>
            where
                <T as FromStr>::Err: std::fmt::Display;
        }

        impl Κ for &[u8] {
            fn κ<T: FromStr>(self) -> impl Iterator<Item = T>
            where
                <T as FromStr>::Err: std::fmt::Display,
            {
                std::str::from_utf8(self).unwrap().κ()
            }
        }

        impl Κ for &str {
            fn κ<T: FromStr>(self) -> impl Iterator<Item = T>
            where
                <T as FromStr>::Err: std::fmt::Display,
            {
                self.split_ascii_whitespace().map(|x| x.λ())
            }
        }

        pub trait Α<T> {
            fn α(self) -> T;
        }

        impl<T, E: std::fmt::Display> Α<T> for Result<T, E> {
            #[cfg_attr(debug_assertions, track_caller)]
            fn α(self) -> T {
                match self {
                    Ok(v) => v,
                    Err(e) => {
                        panic!("unwrap failed: {e}");
                    }
                }
            }
        }
        impl<T> Α<T> for Option<T> {
            #[cfg_attr(debug_assertions, track_caller)]
            fn α(self) -> T {
                match self {
                    Some(v) => v,
                    None => panic!("nothingness!"),
                }
            }
        }

        pub trait Ͷ {
            fn ͷ(self) -> impl Iterator<Item = u8>;
        }

        macro_rules! digs {
            ($for:ty) => {
                impl Ͷ for $for {
                    fn ͷ(self) -> impl Iterator<Item = u8> {
                        let digits = (self.ilog10() + 1) as u8;
                        (0..digits)
                            .rev()
                            .map(move |n| ((self / (10 as $for).pow(n as _)) % 10) as u8)
                    }
                }
            };
        }
        digs!(u64);
        digs!(i64);
        digs!(i32);
        digs!(u32);
        digs!(u16);
        digs!(i16);
        digs!(u8);
        digs!(i8);

        #[derive(Copy, Clone, PartialEq, PartialOrd)]
        pub struct Ronge {
            pub begin: u16,
            pub end: u16,
        }

        impl Debug for Ronge {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}..{}", self.begin, self.end)
            }
        }

        impl Display for Ronge {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}..{}", self.begin, self.end)
            }
        }

        impl From<RangeInclusive<u16>> for Ronge {
            fn from(value: RangeInclusive<u16>) -> Self {
                Self {
                    begin: *value.start(),
                    end: *value.end(),
                }
            }
        }

        impl PartialEq<RangeInclusive<u16>> for Ronge {
            fn eq(&self, other: &RangeInclusive<u16>) -> bool {
                self == &Self::from(other.clone())
            }
        }

        impl Ronge {
            pub fn sane(self) -> bool {
                self.end >= self.begin
            }
            pub fn checked_len(self) -> Option<u16> {
                self.sane().then(|| self.len())
            }
            pub fn len(self) -> u16 {
                self.end - self.begin
            }

            /// push up
            pub fn pushu(&mut self, to: u16) {
                self.begin = self.begin.max(to);
            }

            /// push down
            pub fn pushd(&mut self, to: u16) {
                self.end = self.end.min(to);
            }

            pub fn intersect(self, with: Self) -> Self {
                Self {
                    begin: self.begin.max(with.begin),
                    end: self.end.min(with.end),
                }
            }

            pub fn news(&self, begin: u16) -> Self {
                Self {
                    begin,
                    end: self.end,
                }
            }

            pub fn newe(&self, end: u16) -> Self {
                Self {
                    begin: self.begin,
                    end,
                }
            }

            pub fn shrink(&mut self, with: Ronge) {
                self.pushu(with.begin);
                self.pushd(with.end);
            }
        }

        impl IntoIterator for Ronge {
            type Item = u16;

            type IntoIter = std::ops::Range<u16>;

            fn into_iter(self) -> Self::IntoIter {
                self.begin..self.end
            }
        }

        pub trait Μ where
            Self: Sized,
        {
            fn μ(self, d: char) -> (Self, Self);
            fn μκ<T: FromStr>(self, d: char) -> impl Iterator<Item = (T, T)>
            where
                <T as FromStr>::Err: std::fmt::Display;

            fn μ1(self, d: char) -> Self {
                self.μ(d).1
            }

            fn μ0(self, d: char) -> Self {
                self.μ(d).0
            }

            fn between(self, a: char, b: char) -> Self {
                self.μ1(a).μ0(b)
            }
        }

        impl Μ for &[u8] {
            fn μ(self, d: char) -> (Self, Self) {
                let i = self
                    .iter()
                    .position(|&x| x == d as u8)
                    .unwrap_or_else(|| shucks!("{} should split at {d} fine", self.p()));
                (&self[..i], &self[i + 1..])
            }

            fn μκ<T: FromStr>(self, d: char) -> impl Iterator<Item = (T, T)>
            where
                <T as FromStr>::Err: std::fmt::Display,
            {
                let (α, β) = self.μ(d);
                α.κ::<T>().zip(β.κ::<T>())
            }
        }

        pub fn gt<A: std::cmp::PartialOrd<T>, T>(n: T) -> impl Fn(A) -> bool {
            move |a| a > n
        }

        pub fn lt<A: std::cmp::PartialOrd<T>, T>(n: T) -> impl Fn(A) -> bool {
            move |a| a < n
        }

        impl Μ for &str {
            fn μ(self, d: char) -> (Self, Self) {
                self.split_once(d)
                    .unwrap_or_else(|| shucks!("{self} should split at {d} fine"))
            }

            fn μκ<T: FromStr>(self, d: char) -> impl Iterator<Item = (T, T)>
            where
                <T as FromStr>::Err: std::fmt::Display,
            {
                let (α, β) = self.μ(d);
                α.κ::<T>().zip(β.κ::<T>())
            }
        }

        pub trait IterͶ: Iterator {
            fn ͷ(self) -> impl Iterator<Item = u8>;
        }

        impl<I: Iterator<Item = u64>> IterͶ for I {
            fn ͷ(self) -> impl Iterator<Item = u8> {
                self.flat_map(Ͷ::ͷ)
            }
        }

        pub trait TupleIterTools3<T, U, V>: Iterator {
            fn l(self) -> impl Iterator<Item = T>;
            fn m(self) -> impl Iterator<Item = U>;
            fn r(self) -> impl Iterator<Item = V>;
            fn lm(self) -> impl Iterator<Item = (T, U)>;
            fn lr(self) -> impl Iterator<Item = (T, V)>;
            fn mr(self) -> impl Iterator<Item = (U, V)>;
        }

        pub trait TupleIterTools2<T, U>: Iterator {
            fn l(self) -> impl Iterator<Item = T>;
            fn r(self) -> impl Iterator<Item = U>;
        }

        pub trait TupleIterTools2R<T, U>: Iterator {
            fn l(self) -> impl Iterator<Item = T>;
            fn r(self) -> impl Iterator<Item = U>;
        }

        pub fn l<R, T, U>(f: impl Fn(T) -> R) -> impl Fn((T, U)) -> R {
            move |(x, _)| f(x)
        }
        pub fn r<R, T, U>(f: impl Fn(U) -> R) -> impl Fn((T, U)) -> R {
            move |(_, x)| f(x)
        }

        pub trait FilterBy3<T, U, V>: Iterator {
            fn fl(self, f: impl Fn(T) -> bool) -> impl Iterator<Item = (T, U, V)>;
            fn fm(self, f: impl Fn(U) -> bool) -> impl Iterator<Item = (T, U, V)>;
            fn fr(self, f: impl Fn(V) -> bool) -> impl Iterator<Item = (T, U, V)>;
        }

        impl<T: Copy, U: Copy, V: Copy, I: Iterator<Item = (T, U, V)>> FilterBy3<T, U, V> for I {
            fn fl(self, f: impl Fn(T) -> bool) -> impl Iterator<Item = (T, U, V)> {
                self.filter(move |(x, _, _)| f(*x))
            }

            fn fm(self, f: impl Fn(U) -> bool) -> impl Iterator<Item = (T, U, V)> {
                self.filter(move |(_, x, _)| f(*x))
            }
            fn fr(self, f: impl Fn(V) -> bool) -> impl Iterator<Item = (T, U, V)> {
                self.filter(move |(_, _, x)| f(*x))
            }
        }
        pub trait FilterBy<T, U>: Iterator {
            fn fl(self, f: impl Fn(T) -> bool) -> impl Iterator<Item = (T, U)>;
            fn fr(self, f: impl Fn(U) -> bool) -> impl Iterator<Item = (T, U)>;
        }

        impl<T: Copy, U: Copy, I: Iterator<Item = (T, U)>> FilterBy<T, U> for I {
            fn fl(self, f: impl Fn(T) -> bool) -> impl Iterator<Item = (T, U)> {
                self.filter(move |(x, _)| f(*x))
            }

            fn fr(self, f: impl Fn(U) -> bool) -> impl Iterator<Item = (T, U)> {
                self.filter(move |(_, x)| f(*x))
            }
        }

        pub trait NumTupleIterTools {
            fn πολλαπλασιάζω_και_αθροίζω(&mut self) -> u64;
        }

        impl<I: Iterator<Item = (u64, u64)>> NumTupleIterTools for I {
            fn πολλαπλασιάζω_και_αθροίζω(&mut self) -> u64 {
                self.map(|(a, b)| a * b).sum()
            }
        }

        impl<T, U, I: Iterator<Item = (T, U)>> TupleIterTools2<T, U> for I {
            fn l(self) -> impl Iterator<Item = T> {
                self.map(|(x, _)| x)
            }

            fn r(self) -> impl Iterator<Item = U> {
                self.map(|(_, x)| x)
            }
        }

        impl<'a, T: Copy + 'a, U: Copy + 'a, I: Iterator<Item = &'a (T, U)>> TupleIterTools2R<T, U> for I {
            fn l(self) -> impl Iterator<Item = T> {
                self.map(|&(x, _)| x)
            }
            fn r(self) -> impl Iterator<Item = U> {
                self.map(|&(_, x)| x)
            }
        }

        impl<T, U, V, I: Iterator<Item = (T, U, V)>> TupleIterTools3<T, U, V> for I {
            fn l(self) -> impl Iterator<Item = T> {
                self.map(|(x, _, _)| x)
            }

            fn m(self) -> impl Iterator<Item = U> {
                self.map(|(_, x, _)| x)
            }

            fn r(self) -> impl Iterator<Item = V> {
                self.map(|(_, _, x)| x)
            }

            fn lm(self) -> impl Iterator<Item = (T, U)> {
                self.map(|(a, b, _)| (a, b))
            }

            fn lr(self) -> impl Iterator<Item = (T, V)> {
                self.map(|(a, _, b)| (a, b))
            }

            fn mr(self) -> impl Iterator<Item = (U, V)> {
                self.map(|(_, a, b)| (a, b))
            }
        }

        pub trait GreekTools<T>: Iterator {
            fn Δ(&mut self) -> T;
            fn ι<N>(&mut self) -> impl Iterator<Item = (T, N)>
            where
                Self: Ι<T, N>;
            fn ι1<N>(&mut self) -> impl Iterator<Item = (T, N)>
            where
                Self: Ι<T, N>;
            fn ν<const N: usize>(&mut self, into: &mut [T; N]) -> usize;
            fn Θ(&mut self);
        }

        pub trait ParseIter {
            fn κ<T: FromStr>(&mut self) -> impl Iterator<Item = T>
            where
                <T as FromStr>::Err: std::fmt::Display;
        }

        impl<'x, I: Iterator<Item = &'x [u8]>> ParseIter for I {
            fn κ<T: FromStr>(&mut self) -> impl Iterator<Item = T>
            where
                <T as FromStr>::Err: std::fmt::Display,
            {
                self.flat_map(|x| x.κ())
            }
        }

        pub trait Ι<T, N>: Iterator {
            fn ι(&mut self) -> impl Iterator<Item = (T, N)>;
            fn ι1(&mut self) -> impl Iterator<Item = (T, N)>;
        }

        macro_rules! ι {
            ($t:ty) => {
                impl<T, I: Iterator<Item = T>> Ι<T, $t> for I {
                    fn ι(&mut self) -> impl Iterator<Item = (T, $t)> {
                        self.zip(0..)
                    }

                    fn ι1(&mut self) -> impl Iterator<Item = (T, $t)> {
                        self.zip(1..)
                    }
                }
            };
        }
        ι!(u8);
        ι!(u16);
        ι!(u32);
        ι!(u64);
        ι!(usize);

        pub fn nail<const N: usize>(x: &[u8]) -> [u8; N] {
            unsafe { (x.as_ptr() as *const [u8; N]).read() }
        }

        pub mod reading {
            pub fn 八(n: u64) -> u64 {
                // reinterpret as u64 ("92233721" => 92233721)
                // let n = u64::from_le_bytes(s);
                // combine 4 pairs of single digits:
                // split pieces into odd and even
                //  1_7_3_2_ (le repr)
                // _2_3_2_9
                // then combine
                // _21_37_23_92 (le repr, each byte as 2 digits)
                let n = ((n & 0x0f000f000f000f00) >> 8) + ((n & 0x000f000f000f000f) * 10);
                // combine 2 pairs of 2 digits:
                // split again
                // _21___23__
                // ___37___92
                // then combine
                // __14|137__36|7 (le repr, pipes separating bytes)
                let n = ((n & 0x00ff000000ff0000) >> 16) + ((n & 0x000000ff000000ff) * 100);
                // combine pair of 4 digits
                // split again
                // __14|137____ (then moved to ______14|137, as u64:3721)
                // ______36|07 (as u64: 9223)
                // then combine
                ((n & 0x0000ffff00000000) >> 32) + ((n & 0x000000000000ffff) * 10000)
            }

            use std::{
                io::{self, Read},
                ops::{Add, BitOrAssign, Shl},
            };
            pub trait Ext {
                fn rd<const N: usize>(&mut self) -> io::Result<[u8; N]>;
                fn by(&mut self) -> io::Result<u8> {
                    Ok(self.rd::<1>()?[0])
                }
            }

            impl<T: Read> Ext for T {
                fn rd<const N: usize>(&mut self) -> io::Result<[u8; N]> {
                    let mut buf = [0; N];
                    self.read_exact(&mut buf)?;
                    Ok(buf)
                }
            }
            use super::prelude::*;
            pub fn κ(x: &[u8], v: &mut Vec<u64>) {
                let mut s = 0;
                for &b in x {
                    match b {
                        b' ' => {
                            v.push(s);
                            s = 0;
                        }
                        b => {
                            s = s * 10 + (b - b'0') as u64;
                        }
                    }
                }
            }
            pub trait Ten {
                fn ten() -> Self;
            }
            macro_rules! tenz {
                ($for:ty) => {
                    impl Ten for $for {
                        fn ten() -> $for {
                            10
                        }
                    }
                };
            }
            tenz!(u8);
            tenz!(u16);
            tenz!(u32);
            tenz!(u64);
            tenz!(u128);
            tenz!(i8);
            tenz!(i16);
            tenz!(i32);
            tenz!(i64);
            tenz!(i128);

            const DIG: [u8; 256] = [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7,
                8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 11, 12, 13, 14, 15, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0,
            ];

            pub fn hex_dig(b: u8) -> u8 {
                DIG[b.nat()]
                // (b & 0xF) + 9 * (b >> 6)
            }

            pub fn hexN<
                T: From<u8> + TryFrom<usize> + Shl<T, Output = T> + BitOrAssign<T>,
                const N: usize,
            >(
                a: [u8; N],
            ) -> T {
                let mut c = T::from(hex_dig(a[0])) << T::try_from((N - 1) * 4).ψ();
                for (&n, sh) in a[1..].iter().zip((0..N - 1).rev()) {
                    c |= T::from(hex_dig(n)) << T::try_from(sh * 4).ψ();
                }
                c
            }

            pub fn hex(mut d: &[u8]) -> Result<u32, ()> {
                let &b = d.take_first().ok_or(())?;
                let mut num = hex_dig(b) as u32;
                while let Some(&b) = d.take_first() {
                    num = num * 16 + hex_dig(b) as u32;
                }
                Ok(num)
            }

            pub fn 迄または完了<
                T: Default + std::ops::Mul<T, Output = T> + Add<T, Output = T> + From<u8> + Copy + Ten,
            >(
                x: &mut &[u8],
                until: u8,
            ) -> T {
                let mut n = T::default();
                while let Ok(x) = x.by() {
                    if x == until {
                        return n;
                    }
                    n = n * T::ten() + T::from(x - b'0')
                }
                n
            }

            pub fn 負迄(x: &mut &[u8], until: u8) -> i64 {
                let (sign, mut n) = match x.by().ψ() {
                    b'-' => (-1, 0),
                    b => (1, i64::from(b - b'0')),
                };
                loop {
                    let byte = x.by().ψ();
                    if byte == until {
                        return n * sign as i64;
                    }
                    n = n * 10 + i64::from(byte - b'0');
                }
            }

            pub fn 迄<
                T: Default + std::ops::Mul<T, Output = T> + Add<T, Output = T> + From<u8> + Copy + Ten,
            >(
                x: &mut &[u8],
                until: u8,
            ) -> T {
                let mut n = T::default();
                loop {
                    let byte = x.by().ψ();
                    if byte == until {
                        return n;
                    }
                    n = n * T::ten() + T::from(byte - b'0');
                }
            }

            pub fn all<
                T: Default + std::ops::Mul<T, Output = T> + Add<T, Output = T> + From<u8> + Copy + Ten,
            >(
                x: &[u8],
            ) -> T {
                let mut n = T::default();
                for &byte in x {
                    n = n * T::ten() + T::from(byte - b'0');
                }
                n
            }
        }

        pub fn even(x: &usize) -> bool {
            x % 2 == 0
        }

        impl<T, I: Iterator<Item = T>> GreekTools<T> for I {
            #[cfg_attr(debug_assertions, track_caller)]
            fn Δ(&mut self) -> T {
                self.next().α()
            }

            fn ν<const N: usize>(&mut self, into: &mut [T; N]) -> usize {
                let mut set = 0;
                for e in into {
                    let Some(y) = self.next() else { break };
                    *e = y;
                    set += 1;
                }
                set
            }

            fn ι<N>(&mut self) -> impl Iterator<Item = (T, N)>
            where
                Self: Ι<T, N>,
            {
                self.ι()
            }

            fn ι1<N>(&mut self) -> impl Iterator<Item = (T, N)>
            where
                Self: Ι<T, N>,
            {
                self.ι1()
            }

            fn Θ(&mut self) {
                for _ in self {}
            }
        }

        pub trait TupleUtils<T, U> {
            fn mr<W>(self, f: impl FnOnce(U) -> W) -> (T, W);
            fn ml<V>(self, f: impl FnOnce(T) -> V) -> (V, U);
            fn rev(self) -> (U, T);
        }

        pub trait Widen<Wide> {
            fn nat(self) -> usize;
            fn widen(self) -> Wide;
        }

        macro_rules! wide {
            ($t:ty: $upper:ty) => {
                impl Widen<$upper> for $t {
                    fn nat(self) -> usize {
                        self as _
                    }

                    fn widen(self) -> $upper {
                        self as _
                    }
                }
            };
        }
        wide!(u8: u16);
        wide!(u16: u32);
        wide!(u32: u64);
        wide!(u64: u128);

        pub trait UnifiedTupleUtils<T> {
            fn mb<U>(self, f: impl FnMut(T) -> U) -> (U, U);
        }

        impl<T> UnifiedTupleUtils<T> for (T, T) {
            fn mb<U>(self, mut f: impl FnMut(T) -> U) -> (U, U) {
                (f(self.0), f(self.1))
            }
        }

        impl<T, U> TupleUtils<T, U> for (T, U) {
            fn mr<W>(self, f: impl FnOnce(U) -> W) -> (T, W) {
                (self.0, f(self.1))
            }
            fn ml<V>(self, f: impl FnOnce(T) -> V) -> (V, U) {
                (f(self.0), self.1)
            }
            fn rev(self) -> (U, T) {
                (self.1, self.0)
            }
        }

        #[allow(dead_code)]
        fn cast_to<T: From<bool>>(x: bool, _to: T) -> T {
            x.into()
        }

        #[allow(unused_macros)]
        macro_rules! bits {
            ($bitset:ident + $bit:expr) => {
                $bitset |= 1 << $bit
            };
            ($holder:ident[$index:expr] + $bit:expr) => {
                $holder[$index] |= 1 << $bit;
            };
            ($bitset:ident[$bit:expr]) => {
                ($bitset & 1 << $bit) != 0
            };
            ($holder:ident[$index:expr][$bit:expr]) => {
                ($holder[$index] & 1 << $bit) != 0
            };
            ($holder:ident[$index:expr][$index2:expr][$bit:expr]) => {
                ($holder[$index][$index2] & 1 << $bit) != 0
            };
            ($holder:ident[$index:expr][$index2:expr] + $bit:expr) => {
                $holder[$index][$index2] |= 1 << $bit
            };
            ($bitset:ident[$bit:expr] = $val:expr) => {
                $bitset = ($bitset & !(1 << $bit)) | (util::cast_to($val, $bitset) << $bit)
            };
            ($bitset:ident - $bit:expr) => {
                $bitset &= !(1 << $bit)
            };
            ($bitset:ident ! $bit:expr) => {
                $bitset ^= 1 << $bit
            };
        }
        pub(crate) use bits;

        pub struct Lines<'a> {
            bytes: &'a [u8],
        }

        impl<'a> Iterator for Lines<'a> {
            type Item = &'a [u8];

            fn next(&mut self) -> Option<Self::Item> {
                self.bytes.take_line()
            }
        }

        impl<'a> std::iter::FusedIterator for Lines<'a> {}

        impl<'a> DoubleEndedIterator for Lines<'a> {
            #[inline]
            fn next_back(&mut self) -> Option<Self::Item> {
                self.bytes.take_backline()
            }
        }

        pub trait IntoLines {
            fn 行(&self) -> Lines<'_>;
        }

        impl<T: AsRef<[u8]>> IntoLines for T {
            fn 行(&self) -> Lines<'_> {
                Lines {
                    bytes: self.as_ref(),
                }
            }
        }

        pub trait Printable {
            fn p(&self) -> impl std::fmt::Display;
        }

        struct PrintU8s<'a>(&'a [u8]);
        impl std::fmt::Display for PrintU8s<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                for &b in self.0 {
                    if b.is_ascii() {
                        f.write_char(b as char)?;
                    } else {
                        write!(f, "\\x{b:x}")?;
                    }
                }
                Ok(())
            }
        }

        struct PrintManyU8s<'a, T: AsRef<[u8]>>(&'a [T]);
        impl<T: AsRef<[u8]>> std::fmt::Display for PrintManyU8s<'_, T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                for row in self.0.as_ref() {
                    write!(f, "{},", row.as_ref().p())?;
                }
                Ok(())
            }
        }
        impl Printable for [Vec<u8>] {
            fn p(&self) -> impl std::fmt::Display {
                PrintManyU8s(self)
            }
        }

        impl Printable for [&&[u8]] {
            fn p(&self) -> impl std::fmt::Display {
                PrintManyU8s(self)
            }
        }

        impl Printable for [&[u8]] {
            fn p(&self) -> impl std::fmt::Display {
                PrintManyU8s(self)
            }
        }

        impl Printable for [u8] {
            fn p(&self) -> impl std::fmt::Display {
                PrintU8s(self)
            }
        }

        impl Printable for Vec<u8> {
            fn p(&self) -> impl std::fmt::Display {
                PrintU8s(self)
            }
        }

        pub fn sort<T: Ord>(mut x: Vec<T>) -> Vec<T> {
            x.sort_unstable();
            x
        }

        pub trait TakeLine<'b> {
            fn take_line<'a>(&'a mut self) -> Option<&'b [u8]>;
            fn take_backline<'a>(&'a mut self) -> Option<&'b [u8]>;
        }

        impl<'b> TakeLine<'b> for &'b [u8] {
            fn take_line<'a>(&'a mut self) -> Option<&'b [u8]> {
                match memchr::memchr(b'\n', self) {
                    None if self.is_empty() => None,
                    None => Some(std::mem::replace(self, b"")),
                    Some(end) => {
                        let line = &self[..end];
                        *self = &self[end + 1..];
                        Some(line)
                    }
                }
            }

            fn take_backline<'a>(&'a mut self) -> Option<&'b [u8]> {
                let end = self.len().checked_sub(1)?;
                match memchr::memrchr(b'\n', &self[..end]) {
                    None => Some(std::mem::replace(self, b"")),
                    Some(end) => {
                        let line = &self[end + 1..];
                        *self = &self[..end];
                        Some(line)
                    }
                }
            }
        }

        impl<'b> TakeLine<'b> for &'b str {
            fn take_line<'a>(&'a mut self) -> Option<&'b [u8]> {
                match memchr::memchr(b'\n', self.as_bytes()) {
                    None if self.is_empty() => None,
                    None => Some(std::mem::replace(self, "").as_bytes()),
                    Some(end) => {
                        let line = self[..end].as_bytes();
                        *self = &self[end + 1..];
                        Some(line)
                    }
                }
            }

            fn take_backline<'a>(&'a mut self) -> Option<&'b [u8]> {
                let end = self.len().checked_sub(1)?;
                match memchr::memrchr(b'\n', &self.as_bytes()[..end]) {
                    None => Some(std::mem::replace(self, "").as_bytes()),
                    Some(end) => {
                        let line = &self[end + 1..];
                        *self = &self[..end];
                        Some(line.as_bytes())
                    }
                }
            }
        }

        pub trait IntoCombinations<T: Copy>: Iterator {
            /// LEAKY
            fn combine(self) -> impl Iterator<Item = (T, T)>;
        }

        impl<T: Copy + 'static, I: Iterator<Item = T>> IntoCombinations<T> for I {
            fn combine(self) -> impl Iterator<Item = (T, T)> {
                let x = Box::leak(self.collect::<Box<[_]>>());
                x.iter()
                    .enumerate()
                    .flat_map(|(i, &a)| x[i..].iter().map(move |&b| (a, b)))
            }
        }

        pub trait Skip {
            fn skip(&mut self, n: usize);
        }

        impl<T> Skip for &[T] {
            #[cfg_attr(debug_assertions, track_caller)]
            fn skip(&mut self, n: usize) {
                if cfg!(debug_assertions) {
                    *self = &self[n..];
                } else {
                    *self = C! { &self[n..] };
                }
            }
        }

        impl Skip for &str {
            #[cfg_attr(debug_assertions, track_caller)]
            fn skip(&mut self, n: usize) {
                if cfg!(debug_assertions) {
                    *self = &self[n..];
                } else {
                    *self = C! { &self[n..] };
                }
            }
        }

        /// WYRAND based rng's
        pub mod rand {
            /// WYRAND
            pub fn u64() -> u64 {
                static mut STATE: u64 = 0;
                let tmp = unsafe {
                    STATE = STATE.wrapping_add(0x60bee2bee120fc15);
                    (STATE as u128).wrapping_mul(0xa3b195354a39b70d)
                };
                let m1 = (tmp >> 64) ^ tmp;
                let tmp = m1.wrapping_mul(0x1b03738712fad5c9);
                ((tmp >> 64) ^ tmp) as u64
            }

            /// 0..N
            pub mod limit {
                use super::super::Widen;

                pub fn u64(of: u64) -> u64 {
                    ((super::u64().widen().wrapping_mul(of.widen())) >> 64) as u64
                }
            }

            pub fn u32() -> u32 {
                u64() as u32
            }

            pub fn u16() -> u16 {
                u64() as u16
            }

            pub fn f32() -> f32 {
                (1.0 / ((1u32 << 24) as f32)) * ((u32() >> 8) as f32)
            }

            pub fn f64() -> f64 {
                (1.0 / ((1u64 << 53) as f64)) * ((u64() >> 11) as f64)
            }
        }
    }
    use atools::prelude::*;
    use hinted::HintExt;
    pub use util::prelude::*;
    fn p1(i: &str) -> impl Display {
        static mut a: [i32; 1000] = [0; 1000];
        static mut b: [i32; 1000] = [0; 1000];

        unsafe {
            i.as_bytes()
                .as_chunks_unchecked::<14>()
                .into_iter()
                .map(|x| (reading::all(&x[0..5]), reading::all(&x[8..13])))
                .enumerate()
                .for_each(|(i, (x, y))| {
                    *a.get_unchecked_mut(i) = x;
                    *b.get_unchecked_mut(i) = y;
                });
            radsort::sort(&mut a);
            radsort::sort(&mut b);
            a.iter()
                .copied()
                .zip(b)
                .map(|(x, y)| (x - y).abs())
                .sum::<i32>()
        }
    }

    static mut a: [u32; 1000] = [0; 1000];
    static mut map: [u32; 100000] = [0; 100000];
    let i = i.as_bytes();

    unsafe {
        let x = C! { &i[..14] };
        let (x, y) = (reading::all(&x[0..5]), reading::all::<u32>(&x[8..13]));
        *a.get_unchecked_mut(0) = x;
        *map.get_unchecked_mut(y as usize) += 1;

        for n in 1..1000 {
            let x = util::reading::八(
                u64::from_le_bytes(util::nail::<8>(i.get_unchecked(n * 14 - 3..)))
                    & 0xffffffffff000000,
            );
            let y = util::reading::八(u64::from_le_bytes(util::nail::<8>(
                i.get_unchecked(n * 14 + 5..),
            )));
            *a.get_unchecked_mut(n) = x as u32;
            *map.get_unchecked_mut(y as usize) += 1;
        }
        a.iter()
            .copied()
            .map(|x| x * map.get_unchecked(x as usize))
            .sum::<u32>() as u64
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
