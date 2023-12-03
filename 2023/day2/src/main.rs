use helper::{Day, Variants};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map},
    multi::separated_list0,
    sequence::{preceded, tuple},
    Finish, IResult,
};

fn main() {
    let kind = std::env::args().nth(1).unwrap_or("naive".into());

    let input = std::hint::black_box(include_str!("../input.txt")).to_owned();

    let result = match kind.as_str() {
        "part1" => part1(&input),
        "part2" => part2(&input),
        _ => {
            eprintln!("error: invalid mode, must be part1");
            std::process::exit(1);
        }
    };

    println!("result: {result}");
}

struct Day2;

impl Day for Day2 {
    fn part1() -> helper::Variants {
        Variants::basic(part1)
    }

    fn part2() -> helper::Variants {
        Variants::basic(part2)
    }
}

#[derive(Debug, Clone, Copy)]
enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
}

fn parse_line(line: &str) -> (u64, Vec<Vec<(u64, Color)>>) {
    let parse_color = |i| -> IResult<&str, Color> {
        alt((
            map(tag("blue"), |_| Color::Blue),
            map(tag("red"), |_| Color::Red),
            map(tag("green"), |_| Color::Green),
        ))(i)
    };
    let parse_cubes = tuple((helper::integer, preceded(tag(" "), parse_color)));
    let parse_round = separated_list0(tag(", "), parse_cubes);
    let parse_game = separated_list0(tag("; "), parse_round);
    let parse_line = tuple((
        preceded(tag("Game "), helper::integer),
        preceded(tag(": "), parse_game),
    ));

    all_consuming(parse_line)(line).finish().unwrap().1
}

fn part1(input: &str) -> u64 {
    const MAX: [u64; 3] = [12, 13, 14];

    input
        .lines()
        .filter_map(|line| {
            let line = parse_line(line);
            for round in line.1 {
                for (amount, color) in round {
                    if MAX[color as usize] < amount {
                        return None;
                    }
                }
            }

            Some(line.0)
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let line = parse_line(line);
            let mut min = [0, 0, 0];
            for round in line.1 {
                for (amount, color) in round {
                    min[color as usize] = min[color as usize].max(amount);
                }
            }

            let power = min[0] * min[1] * min[2];
            power
        })
        .sum()
}

helper::tests! {
    day2 Day2;
    part1 {
        small => 8;
        default => 1931;
    }
    part2 {
        small => 2286;
        default => 83105;
    }
}
