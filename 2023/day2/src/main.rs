use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
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
    let parse_cubes = map(
        tuple((digit1, preceded(tag(" "), parse_color))),
        |(a, c)| (a.parse::<u64>().unwrap(), c),
    );
    let parse_round = separated_list0(tag(", "), parse_cubes);
    let parse_game = separated_list0(tag("; "), parse_round);
    let parse_line = tuple((
        map(preceded(tag("Game "), digit1), |d: &str| {
            d.parse::<u64>().unwrap()
        }),
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

#[cfg(test)]
mod tests {
    #[test]
    fn part1_small() {
        assert_eq!(super::part1(include_str!("../input_small.txt")), 8);
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(include_str!("../input.txt")), 1931);
    }

    #[test]
    fn part2_small() {
        assert_eq!(super::part2(include_str!("../input_small.txt")), 2286);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(include_str!("../input.txt")), 83105);
    }
}
