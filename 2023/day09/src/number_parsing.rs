use std::ops::Add;

fn parse(line: &str, into: &mut Vec<i64>) {
    let mut neg = false;
    let mut acc = 0;

    for byte in line.bytes() {
        match byte {
            b' ' => {
                if neg {
                    acc = -acc
                };
                into.push(acc);
                acc = 0;
                neg = false;
            }
            b'-' => neg = true,
            // must be b'0'..=b'9'
            _ => {
                let value = byte - b'0';
                acc = (acc * 10) + (value as i64);
            }
        }
    }
    if neg {
        acc = -acc
    };
    into.push(acc);
}

fn execute(
    input: &str,
    last_or_first: impl Fn(&[i64]) -> i64,
    fold: impl Fn(i64, i64) -> i64 + Copy,
) -> i64 {
    let mut row1 = Vec::with_capacity(128);
    let mut row2 = Vec::with_capacity(128);

    let mut last_values = Vec::with_capacity(16);

    input
        .lines()
        .map(|line| {
            row1.clear();
            parse(line, &mut row1);
            row2.clone_from(&row1);

            last_values.clear();
            last_values.push(last_or_first(&row1));

            while !row2.iter().all(|&n| n == 0) {
                row1.clear();
                row1.extend(row2.windows(2).map(|s| s[1] - s[0]));

                last_values.push(last_or_first(&row1));

                std::mem::swap(&mut row1, &mut row2);
            }

            last_values.iter().copied().rev().fold(0, fold)
        })
        .sum::<i64>()
}

pub fn part1(input: &str) -> u64 {
    execute(input, |s| *s.last().unwrap(), Add::add) as u64
}

pub fn part2(input: &str) -> u64 {
    execute(input, |s| *s.first().unwrap(), |a, b| b - a) as u64
}
