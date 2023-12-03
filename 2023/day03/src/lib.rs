use std::collections::HashMap;

use helper::Day;

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
    fn part1() -> helper::Variants {
        part1_variants!(construct_variants)
    }

    fn part2() -> helper::Variants {
        part2_variants!(construct_variants)
    }
}

fn part1(input: &str) -> u64 {
    fn contains_symbol(s: &str) -> bool {
        s.chars().any(|c| !c.is_ascii_digit() && c != '.')
    }

    let len = input.lines().next().unwrap().len();
    let empty_border = std::iter::repeat('.').take(len).collect::<String>();

    let mut prev2 = empty_border.as_str();
    let mut prev1 = input.lines().next().unwrap();

    let mut acc = 0;

    for next in input.lines().skip(1).chain(Some(empty_border.as_str())) {
        let mut numbers = prev1.char_indices().peekable();
        while let Some((start, c)) = numbers.next() {
            if c.is_ascii_digit() {
                let mut end = start;
                while let Some((idx, '0'..='9')) = numbers.next() {
                    end = idx;
                }

                let box_bounds = (start.saturating_sub(1))..std::cmp::min(end + 2, len);
                let number = prev1[start..=end].parse::<u64>().unwrap();

                if contains_symbol(&prev2[box_bounds.clone()])
                    || contains_symbol(&prev1[box_bounds.clone()])
                    || contains_symbol(&next[box_bounds])
                {
                    acc += number;
                }
            }
        }

        prev2 = prev1;
        prev1 = next;
    }

    acc
}

fn part2(input: &str) -> u64 {
    fn contains_gear(s: &str) -> Option<usize> {
        s.chars().position(|c| !c.is_ascii_digit() && c != '.')
    }

    let len = input.lines().next().unwrap().len();
    let empty_border = std::iter::repeat('.').take(len).collect::<String>();

    let mut prev2 = empty_border.as_str();
    let mut prev1 = input.lines().next().unwrap();

    let mut gears: HashMap<(usize, usize), Vec<u64>> = HashMap::new();

    for (line_number, next) in input
        .lines()
        .skip(1)
        .chain(Some(empty_border.as_str()))
        .enumerate()
    {
        let mut numbers = prev1.char_indices().peekable();
        while let Some((start, c)) = numbers.next() {
            if c.is_ascii_digit() {
                let mut end = start;
                while let Some((idx, '0'..='9')) = numbers.next() {
                    end = idx;
                }

                let box_bounds = (start.saturating_sub(1))..std::cmp::min(end + 2, len);
                let number = prev1[start..=end].parse::<u64>().unwrap();

                if let Some(position) = contains_gear(&prev2[box_bounds.clone()]) {
                    let key = (line_number - 1, box_bounds.start + position);
                    gears.entry(key).or_default().push(number);
                }

                if let Some(position) = contains_gear(&prev1[box_bounds.clone()]) {
                    let key = (line_number, box_bounds.start + position);
                    gears.entry(key).or_default().push(number);
                }

                if let Some(position) = contains_gear(&next[box_bounds.clone()]) {
                    let key = (line_number + 1, box_bounds.start + position);
                    gears.entry(key).or_default().push(number);
                }
            }
        }

        prev2 = prev1;
        prev1 = next;
    }

    gears
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum()
}

helper::tests! {
    day03 Day03;
    part1 {
        small => 4361;
        default => 537832;
    }
    part2 {
        small => 467835;
        default => 81939900;
    }
}

helper::benchmarks! {}
