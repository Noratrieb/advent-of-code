use helper::{Day, IteratorExt, Variants};

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
        basic => crate::part2, sample_count=50;
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

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn winning_presses(&self) -> u64 {
        let t = self.time;

        (0..t)
            .filter(move |&i| {
                let t_run = t - i;
                let d = t_run * i;

                d > self.distance
            })
            .count() as u64
    }
}

fn parse(input: &str) -> Vec<Race> {
    let lines = input.lines().collect_array::<2>().unwrap();
    lines[0]
        .split_ascii_whitespace()
        .zip(lines[1].split_ascii_whitespace())
        .skip(1)
        .map(|(time, distance)| Race {
            time: time.parse().unwrap(),
            distance: distance.parse().unwrap(),
        })
        .collect()
}

fn part1(input: &str) -> u64 {
    let races = parse(input);

    races.iter().map(Race::winning_presses).product()
}

fn parse2(input: &str) -> Race {
    let lines = input.lines().collect_array::<2>().unwrap().map(|line| {
        line.split_ascii_whitespace()
            .skip(1)
            .collect::<String>()
            .parse()
            .unwrap()
    });

    Race {
        time: lines[0],
        distance: lines[1],
    }
}

fn part2(input: &str) -> u64 {
    let race = parse2(input);
    race.winning_presses()
}

helper::tests! {
    day06 Day06;
    part1 {
        small => 288;
        default => 6209190;
    }
    part2 {
        small => 71503;
        default => 28545089;
    }
}
helper::benchmarks! {}
