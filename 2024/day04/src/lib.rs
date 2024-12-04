use helper::{Day, Variants};

pub fn main() {
    helper::main::<Day04>(include_str!("../input.txt"));
}

struct Day04;

helper::define_variants! {
    day => crate::Day04;
    part1 {
        basic => crate::part1;
    }
    part2 {
        basic => crate::part2;
    }
}

impl Day for Day04 {
    fn part1() -> Variants {
        part1_variants!(construct_variants)
    }

    fn part2() -> Variants {
        part2_variants!(construct_variants)
    }
}

fn part1(input: &str) -> u64 {
    const XMAS: &[u8] = b"XMAS";
    const SAMX: &[u8] = b"SAMX";
    let mut count = 0;

    let lines = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    // Horizontal rows
    for line in &lines {
        for i in 0..line.len() {
            if line[i..].starts_with(XMAS) || line[i..].starts_with(SAMX) {
                count += 1;
            }
        }
    }

    let line_length = lines[0].len();

    let cols = (0..line_length).map(|i| lines.iter().map(|line| line[i]).collect::<Vec<_>>());

    for col in cols {
        // dbg!(std::str::from_utf8(&col));
        for i in 0..col.len() {
            if col[i..].starts_with(XMAS) || col[i..].starts_with(SAMX) {
                count += 1;
            }
        }
    }

    let all = lines
        .iter()
        .flat_map(|line| *line)
        .copied()
        .collect::<Vec<_>>();

    for i in 0..all.len() {
        if i % line_length >= (line_length - 3) {
            continue;
        }
        (|| {
            let offset = line_length + 1;
            let a = *all.get(i)?;
            let b = *all.get(i + offset * 1)?;
            let c = *all.get(i + offset * 2)?;
            let d = *all.get(i + offset * 3)?;
            let chunk = [a, b, c, d];
            if chunk == XMAS || chunk == SAMX {
                count += 1;
            }

            Some(())
        })();
    }

    for i in 0..all.len() {
        if i % line_length < 3 {
            continue;
        }
        (|| {
            let offset = line_length - 1;
            let a = *all.get(i)?;
            let b = *all.get(i + offset * 1)?;
            let c = *all.get(i + offset * 2)?;
            let d = *all.get(i + offset * 3)?;
            let chunk = [a, b, c, d];
            if chunk == XMAS || chunk == SAMX {
                count += 1;
            }

            Some(())
        })();
    }

    count
}

fn part2(_input: &str) -> u64 {
    0
}

helper::tests! {
    day04 Day04;
    part1 {
        small => 18;
        default => 0;
    }
    part2 {
        small => 0;
        default => 0;
    }
}
helper::benchmarks! {}
