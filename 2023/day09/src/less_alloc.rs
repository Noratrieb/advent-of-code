use std::ops::Add;

fn parse(input: &str) -> impl Iterator<Item = impl Iterator<Item = i64> + '_> + '_ {
    input
        .lines()
        .map(|line| line.split_ascii_whitespace().map(|s| s.parse().unwrap()))
}
fn execute(
    input: impl Iterator<Item = impl Iterator<Item = i64>>,
    last_or_first: impl Fn(&[i64]) -> i64,
    fold: impl Fn(i64, i64) -> i64 + Copy,
) -> i64 {
    let mut values = Vec::with_capacity(128);
    let mut derive = Vec::with_capacity(128);

    let mut last_values = Vec::with_capacity(16);

    input
        .map(|values_iter| {
            values.clear();

            values.extend(values_iter);
            derive.clone_from(&values);

            last_values.clear();
            last_values.push(last_or_first(&values));

            while !derive.iter().all(|&n| n == 0) {
                values.clear();
                values.extend(derive.windows(2).map(|s| s[1] - s[0]));

                last_values.push(last_or_first(&values));

                std::mem::swap(&mut values, &mut derive);
            }

            last_values.iter().copied().rev().fold(0, fold)
        })
        .sum::<i64>()
}

pub fn part1(input: &str) -> u64 {
    execute(parse(input), |s| *s.last().unwrap(), Add::add) as u64
}

pub fn part2(input: &str) -> u64 {
    execute(parse(input), |s| *s.first().unwrap(), |a, b| b - a) as u64
}
