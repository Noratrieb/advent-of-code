use helper::{Day, Variants};

pub fn main() {
    helper::main::<Day15>(include_str!("../input.txt"));
}

struct Day15;

helper::define_variants! {
    day => crate::Day15;
    part1 {
        basic => crate::part1;
    }
    part2 {
        basic => crate::part2;
    }
}

impl Day for Day15 {
    fn part1() -> Variants {
        part1_variants!(construct_variants)
    }

    fn part2() -> Variants {
        part2_variants!(construct_variants)
    }
}

fn part1(input: &str) -> u64 {
    input.trim_end().split(",").map(hash).sum()
}

fn hash(s: &str) -> u64 {
    s.bytes()
        .fold(0, |state, b| ((state + u64::from(b)) * 17) % 256)
}

fn part2(input: &str) -> u64 {
    let mut boxes = vec![Vec::<(&str, u64)>::new(); 256];

    for step in input.trim_end().split(',') {
        if let Some(label) = step.strip_suffix('-') {
            let b = &mut boxes[hash(label) as usize];
            b.retain(|lens| lens.0 != label);
        } else {
            let (label, number) = step.split_once("=").unwrap();
            let number = number.parse().unwrap();
            let b = &mut boxes[hash(label) as usize];
            if let Some(existing) = b.iter().position(|lens| lens.0 == label) {
                b[existing] = (label, number);
            } else {
                b.push((label, number));
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .flat_map(|(box_idx, b)| {
            b.iter().enumerate().map(move |(lens_idx, lens)| {
                (1 + (box_idx as u64)) * ((lens_idx as u64) + 1) * lens.1
            })
        })
        .sum()
}

helper::tests! {
    day15 Day15;
    part1 {
        small => 1320;
        default => 513643;
    }
    part2 {
        small => 145;
        default => 265345;
    }
}
helper::benchmarks! {}
