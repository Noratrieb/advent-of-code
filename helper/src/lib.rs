use nom::{character::complete::digit1, combinator::map, IResult};

pub fn integer(input: &str) -> IResult<&str, u64> {
    map(digit1, |d: &str| d.parse::<u64>().unwrap())(input)
}
