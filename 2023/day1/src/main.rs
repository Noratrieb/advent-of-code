use std::mem::MaybeUninit;

use helper::{Day, Variant, Variants};

mod branchless;
mod naive;
mod no_lines;
mod vectorized;
mod zero_alloc;

fn main() {
    let kind = std::env::args().nth(1).unwrap_or("naive".into());

    let mut input = std::hint::black_box(include_str!("../input.txt")).to_owned();

    input.reserve(10); // enough to read u64
    unsafe {
        input
            .as_mut_vec()
            .spare_capacity_mut()
            .fill(MaybeUninit::new(0))
    };

    match kind.as_str() {
        "part1" => part1(&input),
        "naive" => naive::part2(&input),
        "zero_alloc" => zero_alloc::part2(&input),
        "branchless" => unsafe { branchless::part2(&input) },
        "no_lines" => unsafe { no_lines::part2(&input) },
        "vectorized" => unsafe { vectorized::part2(&input) },
        _ => {
            eprintln!("error: invalid mode, must be part1,naive,zero_alloc,branchless");
            std::process::exit(1);
        }
    };
}

struct Day1;

impl Day for Day1 {
    fn pad_input(input: &str) -> std::borrow::Cow<str> {
        let mut input = input.to_owned();
        input.reserve(10); // enough to read u64
        unsafe {
            input
                .as_mut_vec()
                .spare_capacity_mut()
                .fill(MaybeUninit::new(0))
        };
        std::borrow::Cow::Owned(input)
    }
    fn part1() -> Variants {
        Variants::basic(part1)
    }

    fn part2() -> Variants {
        Variants {
            variants: vec![
                Variant::new("naive", naive::part2),
                Variant::new("zero_alloc", zero_alloc::part2),
                Variant::new("branchless", |i| unsafe { branchless::part2(i) }),
                Variant::new("no_lines", |i| unsafe { no_lines::part2(i) }),
                Variant::new("vectorized", |i| unsafe { vectorized::part2(i) }),
            ],
        }
    }
}

fn part1(input: &str) -> u64 {
    let sum = input
        .lines()
        .map(|line| {
            let mut chars = line.chars().filter(|c| c.is_ascii_digit());
            let first = chars.next().unwrap();
            let last = chars.next_back().unwrap_or(first);

            [first, last]
                .into_iter()
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .sum::<u64>();

    sum
}

helper::tests! {
    day1 Day1;
    part1 {
        "../input_small1.txt" => 142;
        "../input.txt" => 54632;
    }
    part2 {
        "../input_small2.txt" => 281;
        "../input.txt" => 54019;
    }
}
