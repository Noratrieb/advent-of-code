use std::cmp::{self, Ordering};

use helper::{Day, IteratorExt, Variants};

pub fn main() {
    helper::main::<Day07>(include_str!("../input.txt"));
}

struct Day07;

helper::define_variants! {
    day => crate::Day07;
    part1 {
        basic => crate::part1;
    }
    part2 {
        basic => crate::part2;
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveSame = 0,
    FourSame,
    FullHouse,
    ThreeSame,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct Hand {
    values: [u8; 5],
    hand_type: HandType,
    bid: u64,
}

impl HandType {
    fn of(hand: [u8; 5]) -> Self {
        let mut map: [Option<(u8, u8)>; 5] = [None; 5];

        for card in hand {
            if let Some(existing) = map.iter_mut().find(|c| c.is_some_and(|c| c.0 == card)) {
                existing.as_mut().unwrap().1 += 1;
            } else {
                let idx = map.iter().position(|c| c.is_none()).unwrap();
                map[idx] = Some((card, 1));
            }
        }
        map.sort_by_key(|c| cmp::Reverse(c.map(|c| c.1)));

        let map = map.map(|c| c.map_or(0, |c| c.1));
        if map[0] == 5 {
            Self::FiveSame
        } else if map[0] == 4 {
            Self::FourSame
        } else if map[0] == 3 && map[1] == 2 {
            Self::FullHouse
        } else if map[0] == 3 {
            Self::ThreeSame
        } else if map[0] == 2 && map[1] == 2 {
            Self::TwoPair
        } else if map[0] == 2 {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::HandType;

    #[test]
    fn hand_type() {
        assert_eq!(HandType::of(*b"32T3K"), HandType::OnePair);
        assert_eq!(HandType::of(*b"K6KK6"), HandType::FullHouse);
    }
}

fn parse(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let cards = parts.next().unwrap();
            let bid = parts.next().unwrap().parse().unwrap();

            let values = cards.bytes().collect_array::<5>().unwrap();
            let hand_type = HandType::of(values);

            Hand {
                values,
                hand_type,
                bid,
            }
        })
        .collect()
}

fn part1(input: &str) -> u64 {
    let mut hands = parse(input);

    // Worst hand first, best hand last.
    hands.sort_by(|a, b| {
        let mk_compare = |v| match v {
            b'A' => b'Z',
            b'K' => b'Y',
            b'T' => b'I',
            other => other,
        };

        let types = a.hand_type.cmp(&b.hand_type);
        if types != Ordering::Equal {
            return types.reverse();
        }
        for (a, b) in std::iter::zip(a.values, b.values) {
            if a == b {
                continue;
            }
            let a = mk_compare(a);
            let b = mk_compare(b);

            return a.cmp(&b);
        }
        Ordering::Equal
    });

    hands
        .iter()
        .zip(1_u64..)
        .map(|(hand, i)| hand.bid * i)
        .sum()
}

fn part2(_input: &str) -> u64 {
    0
}

helper::tests! {
    day07 Day07;
    part1 {
        small => 6440;
        default => 248453531;
    }
    part2 {
        small => 0;
        default => 0;
    }
}
helper::benchmarks! {}
