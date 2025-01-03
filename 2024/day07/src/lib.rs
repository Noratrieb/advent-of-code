use core::str;

use helper::{parse_unwrap, Day, Variants};

pub fn main() {
    helper::main::<Day07>(include_str!("../input.txt"));
}

struct Day07;

helper::define_variants! {
    day => crate::Day07;
    part1 {
        basic => crate::part1,sample_count=1000;
    }
    part2 {
        basic => crate::part2,sample_count=500;
        no_string_fmt => crate::part2_no_string_fmt,sample_count=1000;
        order => crate::part2_order,sample_count=1000;
        parsing => crate::part2_parsing,sample_count=2000;
        parsing2 => crate::part2_parsing2,sample_count=2000;
        parsing3 => crate::part2_parsing3;
        // This is fast on CPUs that have slow division, but slower on CPUs with fast division.
        no_div => crate::part2_no_div;
    }
}

impl Day for Day07 {
    fn part1() -> Variants {
        part1_variants!(construct_variants)
    }

    fn part2() -> Variants {
        part2_variants!(construct_variants)
    }
}

fn part1(input: &str) -> u64 {
    let mut total = 0;
    for line in input.lines() {
        let (result, values) = line.split_once(": ").unwrap();
        let result = parse_unwrap(result);
        let values = values.split(" ").map(parse_unwrap).collect::<Vec<_>>();

        fn does_work(values: &[u64], result: u64) -> bool {
            if values.len() == 1 {
                return values[0] == result;
            }

            let l = *values.last().unwrap();
            let next = &values[..(values.len() - 1)];

            let mul_works = result % l == 0;
            let sub_works = l <= result;

            (mul_works && does_work(next, result / l)) || (sub_works && does_work(next, result - l))
        }

        if does_work(&values, result) {
            total += result;
        }
    }

    total
}

fn part2(input: &str) -> u64 {
    let mut total = 0;
    for line in input.lines() {
        let (result, values) = line.split_once(": ").unwrap();
        let result = parse_unwrap(result);
        let values = values
            .split(" ")
            .map(|val| (val, parse_unwrap(val)))
            .collect::<Vec<_>>();

        fn does_work(values: &[(&str, u64)], result: u64) -> bool {
            if values.len() == 1 {
                return values[0].1 == result;
            }

            let (l_str, l) = *values.last().unwrap();
            let next = &values[..(values.len() - 1)];

            let mul_works = result % l == 0;
            let sub_works = l <= result;

            (mul_works && does_work(next, result / l))
                || (sub_works && does_work(next, result - l))
                || (result.to_string().ends_with(l_str)
                    && does_work(next, result / (10_u64.pow(l_str.len() as u32))))
        }

        if does_work(&values, result) {
            total += result;
        }
    }
    total
}

fn part2_no_string_fmt(input: &str) -> u64 {
    let mut total = 0;
    for line in input.lines() {
        let (result, values) = line.split_once(": ").unwrap();
        let result = parse_unwrap(result);
        let values = values
            .split(" ")
            .map(|val| (val, parse_unwrap(val)))
            .collect::<Vec<_>>();

        fn does_work(values: &[(&str, u64)], result: u64) -> bool {
            if values.len() == 1 {
                return values[0].1 == result;
            }

            let (l_str, l) = *values.last().unwrap();
            let next = &values[..(values.len() - 1)];

            let mul_works = result % l == 0;
            let sub_works = l <= result;

            (mul_works && does_work(next, result / l))
                || (sub_works && does_work(next, result - l))
                || {
                    let thousand = 10_u64.pow(l_str.len() as u32);
                    let concat_works = result % thousand == l;
                    concat_works && does_work(next, result / thousand)
                }
        }

        if does_work(&values, result) {
            total += result;
        }
    }
    total
}

fn part2_order(input: &str) -> u64 {
    let mut total = 0;
    for line in input.lines() {
        let (result, values) = line.split_once(": ").unwrap();
        let result = parse_unwrap(result);
        let values = values
            .split(" ")
            .map(|val| (val, parse_unwrap(val)))
            .collect::<Vec<_>>();

        fn does_work(values: &[(&str, u64)], result: u64) -> bool {
            if values.len() == 1 {
                return values[0].1 == result;
            }

            let (l_str, l) = *values.last().unwrap();
            let next = &values[..(values.len() - 1)];
            let thousand = 10_u64.pow(l_str.len() as u32);

            let concat_works = result % thousand == l;
            let mul_works = result % l == 0;
            let sub_works = l <= result;

            // Concat is the least common, so doing it first helps remove the most possibilities.
            false
                || (concat_works && does_work(next, result / thousand))
                || (mul_works && does_work(next, result / l))
                || (sub_works && does_work(next, result - l))
        }

        if does_work(&values, result) {
            total += result;
        }
    }
    total
}

fn part2_parsing(input: &str) -> u64 {
    let mut total = 0;

    let mut values = Vec::new();

    for line in input.lines() {
        values.clear();
        let (result, values_str) = line.split_once(": ").unwrap();
        let result = parse_unwrap(result);
        values.extend(values_str.split(' ').map(str::as_bytes).map(|vals| {
            let parse1 = |i| (vals[i] - b'0') as u64;
            (
                10_u64.pow(vals.len() as u32),
                match vals.len() {
                    1 => parse1(0),
                    2 => parse1(0) * 10 + parse1(1),
                    3 => parse1(0) * 100 + parse1(1) * 10 + parse1(2),
                    _ => unreachable!(),
                },
            )
        }));

        fn does_work(values: &[(u64, u64)], result: u64) -> bool {
            if values.len() == 1 {
                return values[0].1 == result;
            }

            let (thousand, l) = *values.last().unwrap();
            let next = &values[..(values.len() - 1)];

            let concat_works = result % thousand == l;
            let mul_works = result % l == 0;
            let sub_works = l <= result;

            // Concat is the least common, so doing it first helps remove the most possibilities.
            false
                || (concat_works && does_work(next, result / thousand))
                || (mul_works && does_work(next, result / l))
                || (sub_works && does_work(next, result - l))
        }

        if does_work(&values, result) {
            total += result;
        }
    }
    total
}

fn part2_parsing2(input: &str) -> u64 {
    let mut total = 0;

    let mut values = Vec::new();

    for line in input.lines() {
        values.clear();

        let parse1 = |a: u8| (a - b'0') as u64;

        let line = line.as_bytes();
        let mut i = 0;
        let mut result = 0;

        while line[i] != b':' {
            let next = parse1(line[i]);
            result *= 10;
            result += next;
            i += 1;
        }
        i += 2; // ': '

        while i < line.len() {
            let mut val = parse1(line[i]);
            i += 1;
            if i < line.len() && line[i] != b' ' {
                val *= 10;
                val += parse1(line[i]);
                i += 1;

                if i < line.len() && line[i] != b' ' {
                    val *= 10;
                    val += parse1(line[i]);
                    i += 1;
                    values.push((1000, val));
                } else {
                    values.push((100, val));
                }
            } else {
                values.push((10, val));
            }

            i += 1; // ' '
        }

        fn does_work(values: &[(u64, u64)], result: u64) -> bool {
            if values.len() == 1 {
                return values[0].1 == result;
            }

            let (thousand, l) = *values.last().unwrap();
            let next = &values[..(values.len() - 1)];

            let concat_works = result % thousand == l;
            let mul_works = result % l == 0;
            let sub_works = l <= result;

            // Concat is the least common, so doing it first helps remove the most possibilities.
            false
                || (concat_works && does_work(next, result / thousand))
                || (mul_works && does_work(next, result / l))
                || (sub_works && does_work(next, result - l))
        }

        if does_work(&values, result) {
            total += result;
        }
    }
    total
}

fn part2_parsing3(input: &str) -> u64 {
    let mut total = 0;

    let mut values = Vec::with_capacity(100);

    let mut input = input.as_bytes();
    while input.len() > 1 {
        values.clear();

        let parse1 = |a: u8| (a - b'0') as u64;

        let mut i = 0;
        let mut result = 0;

        while input[i] != b':' {
            let next = parse1(input[i]);
            result *= 10;
            result += next;
            i += 1;
        }
        i += 2; // ': '

        loop {
            let mut val = parse1(input[i]);
            i += 1;
            if input[i] >= b'0' {
                val *= 10;
                val += parse1(input[i]);
                i += 1;

                if input[i] >= b'0' {
                    val *= 10;
                    val += parse1(input[i]);
                    i += 1;
                    values.push((1000, val));
                } else {
                    values.push((100, val));
                }
            } else {
                values.push((10, val));
            }

            if input[i] == b'\n' {
                break;
            }
            i += 1; // ' '
        }
        i += 1;
        input = &input[i..];

        fn does_work(values: &[(u64, u64)], result: u64) -> bool {
            if values.len() == 1 {
                return values[0].1 == result;
            }

            let (thousand, l) = *values.last().unwrap();
            let next = &values[..(values.len() - 1)];

            let concat_works = result % thousand == l;
            let mul_works = result % l == 0;
            let sub_works = l <= result;

            // Concat is the least common, so doing it first helps remove the most possibilities.
            false
                || (concat_works && does_work(next, result / thousand))
                || (mul_works && does_work(next, result / l))
                || (sub_works && does_work(next, result - l))
        }

        if does_work(&values, result) {
            total += result;
        }
    }
    total
}

fn part2_no_div(input: &str) -> u64 {
    let mut total = 0;

    let mut values: Vec<(fn(u64) -> (u64, u64), u64)> = Vec::with_capacity(100);

    let mut input = input.as_bytes();
    while input.len() > 1 {
        values.clear();

        let parse1 = |a: u8| (a - b'0') as u64;

        let mut i = 0;
        let mut result = 0;

        while input[i] != b':' {
            let next = parse1(input[i]);
            result *= 10;
            result += next;
            i += 1;
        }
        i += 2; // ': '

        loop {
            let mut val = parse1(input[i]);
            i += 1;
            if input[i] >= b'0' {
                val *= 10;
                val += parse1(input[i]);
                i += 1;

                if input[i] >= b'0' {
                    val *= 10;
                    val += parse1(input[i]);
                    i += 1;
                    values.push((|x| (x / 1000, x % 1000), val));
                } else {
                    values.push((|x| (x / 100, x % 100), val));
                }
            } else {
                values.push((|x| (x / 10, x % 10), val));
            }

            if input[i] == b'\n' {
                break;
            }
            i += 1; // ' '
        }
        i += 1;
        input = &input[i..];

        fn does_work(values: &[(fn(u64) -> (u64, u64), u64)], result: u64) -> bool {
            if values.len() == 1 {
                return values[0].1 == result;
            }

            let (thousand, l) = *values.last().unwrap();
            let next = &values[..(values.len() - 1)];

            let (concat_result, concat_mod) = thousand(result);
            let concat_works = concat_mod == l;
            let mul_works = result % l == 0;
            let mul_result = result / l;
            let sub_works = l <= result;

            // Concat is the least common, so doing it first helps remove the most possibilities.
            false
                || (concat_works && does_work(next, concat_result))
                || (mul_works && does_work(next, mul_result))
                || (sub_works && does_work(next, result - l))
        }

        if does_work(&values, result) {
            total += result;
        }
    }
    total
}


helper::tests! {
    day07 Day07;
    part1 {
        small => 3749;
        default => 303876485655;
    }
    part2 {
        small => 11387;
        default => 146111650210682;
    }
}
helper::benchmarks! {}
