mod sort_cache;
mod unstable_sort;

use std::cmp::{self, Ordering};

use helper::{Day, IteratorExt, Variants};

pub fn main() {
    helper::main::<Day07>(include_str!("../input.txt"));
}

struct Day07;

helper::define_variants! {
    day => crate::Day07;
    part1 {
        basic => crate::part1, sample_count=10000;
        unstable_sort => crate::unstable_sort::part1, sample_count=10000;
        sort_cache => crate::sort_cache::part1, sample_count=10000;
    }
    part2 {
        basic => crate::part2, sample_count=10000;
        unstable_sort => crate::unstable_sort::part2, sample_count=10000;
        sort_cache => crate::sort_cache::part2, sample_count=10000;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

fn hand_type_of(hand: [u8; 5], has_jokers: bool) -> HandType {
    let mut card_type = [0; 5];
    let mut counts = [0; 5];
    let mut jokers = 0;

    for card in hand {
        if card == b'J' && has_jokers {
            jokers += 1;
            continue;
        }
        if let Some(existing) = card_type.into_iter().position(|c| c == card) {
            counts[existing] += 1;
        } else {
            let idx = card_type.into_iter().position(|c| c == 0).unwrap();
            card_type[idx] = card;
            counts[idx] = 1_u8;
        }
    }
    counts.sort_by_key(|&c| cmp::Reverse(c));

    if counts[0] + jokers == 5 {
        HandType::FiveSame
    } else if counts[0] + jokers == 4 {
        HandType::FourSame
    // If there are only cards of two types + jokers, then we can form a full house
    // no matter how they're set up.
    } else if counts[0] + counts[1] + jokers == 5 {
        HandType::FullHouse
    } else if counts[0] + jokers == 3 {
        HandType::ThreeSame
    // We need four cards for a two pair. Given that the previous constellations
    // are not possible, we are able to build a two pair.
    } else if counts[0] + counts[1] + jokers == 4 {
        HandType::TwoPair
    // Fun fact: This == is fine and needn't be >=, because if map[0]+jokers==3,
    // then we can build two pairs.
    } else if counts[0] + jokers == 2 {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

#[cfg(test)]
mod tests {
    use crate::HandType;

    #[test]
    fn hand_type() {
        assert_eq!(super::hand_type_of(*b"32T3K", false), HandType::OnePair);
        assert_eq!(super::hand_type_of(*b"K6KK6", false), HandType::FullHouse);
        assert_eq!(super::hand_type_of(*b"KTJJT", true), HandType::FourSame);
    }
}

fn parse(input: &str, has_jokers: bool) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let cards = parts.next().unwrap();
            let bid = parts.next().unwrap().parse().unwrap();

            let values = cards.bytes().collect_array::<5>().unwrap();
            let hand_type = hand_type_of(values, has_jokers);

            Hand {
                values,
                hand_type,
                bid,
            }
        })
        .collect()
}

fn evaluate_hands(hands: &mut [Hand], has_jokers: bool) -> u64 {
    // Worst hand first, best hand last.
    hands.sort_by(|a, b| {
        let mk_compare = |v| match v {
            b'A' => b'Z',
            b'K' => b'Y',
            b'T' => b'I',
            b'J' if has_jokers => b'0',
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

fn part1(input: &str) -> u64 {
    let mut hands = parse(input, false);
    evaluate_hands(&mut hands, false)
}

fn part2(input: &str) -> u64 {
    let mut hands = parse(input, true);
    evaluate_hands(&mut hands, true)
}

helper::tests! {
    day07 Day07;
    part1 {
        small => 6440;
        default => 248453531;
    }
    part2 {
        small => 5905;
        default => 248781813;
    }
}
helper::benchmarks! {}
