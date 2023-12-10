use helper::{Day, Variants};

pub fn main() {
    helper::main::<Day10>(include_str!("../input.txt"));
}

struct Day10;

helper::define_variants! {
    day => crate::Day10;
    part1 {
        basic => crate::part1;
    }
    part2 {
        basic => crate::part2;
    }
}

impl Day for Day10 {
    fn part1() -> Variants {
        part1_variants!(construct_variants)
    }

    fn part2() -> Variants {
        part2_variants!(construct_variants)
    }
}

const S: u8 = b'S';
const VERTICAL: u8 = b'|';
const HORIZONTAL: u8 = b'-';
const TOP_LEFT: u8 = b'F';
const TOP_RIGHT: u8 = b'7';
const BOTTOM_LEFT: u8 = b'L';
const BOTTOM_RIGHT: u8 = b'J';

struct Candidates {
    v: Vec<Candidate>,
    width: usize,
    len: usize,
}

#[derive(Clone, Copy)]
struct Candidate {
    count: u64,
    pos: usize,
    skip: Skip,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Skip {
    Left,
    Right,
    Top,
    Bottom,
    None,
}

impl Candidates {
    fn push_left(&mut self, from: Candidate) {
        if from.skip != Skip::Left && from.pos % self.width > 0 {
            self.v
                .push(Candidate::new(from.count + 1, from.pos - 1, Skip::Right));
        }
    }
    fn push_right(&mut self, from: Candidate) {
        if from.skip != Skip::Right && (from.pos % self.width) < (self.width - 1) {
            self.v
                .push(Candidate::new(from.count + 1, from.pos + 1, Skip::Left));
        }
    }
    fn push_top(&mut self, from: Candidate) {
        if from.skip != Skip::Top && from.pos >= self.width {
            self.v.push(Candidate::new(
                from.count + 1,
                from.pos - self.width,
                Skip::Bottom,
            ));
        }
    }
    fn push_bottom(&mut self, from: Candidate) {
        if from.skip != Skip::Bottom && from.pos < (self.len - self.width) {
            self.v.push(Candidate::new(
                from.count + 1,
                from.pos + self.width,
                Skip::Top,
            )); // BOTTOM
        }
    }
}

impl Candidate {
    fn new(count: u64, pos: usize, skip: Skip) -> Self {
        Self { count, pos, skip }
    }
}

fn part1(input: &str) -> u64 {
    let bytes = input.as_bytes();
    let width = bytes.into_iter().position(|&b| b == b'\n').unwrap();

    let bytes = bytes
        .iter()
        .copied()
        .filter(|&b| b != b'\n')
        .collect::<Vec<_>>();

    let mut seen = bytes.iter().map(|_| false).collect::<Vec<_>>();

    let s = bytes.iter().position(|&b| b == S).unwrap();

    let mut cs = Candidates {
        v: Vec::new(),
        len: bytes.len(),
        width,
    };

    cs.v.push(Candidate {
        count: 0,
        pos: s,
        skip: Skip::None,
    });

    let mut highest = 0;

    while let Some(c) = cs.v.pop() {
        if seen[c.pos] {
            highest = highest.max(c.count - 1);
            break;
        }
        seen[c.pos] = true;
        match bytes[c.pos] {
            S => {
                cs.push_left(c);
                cs.push_right(c);
                cs.push_top(c);
                cs.push_bottom(c);
            }
            HORIZONTAL => {
                cs.push_left(c);
                cs.push_right(c);
            }
            VERTICAL => {
                cs.push_top(c);
                cs.push_bottom(c);
            }
            TOP_LEFT => {
                cs.push_right(c);
                cs.push_bottom(c);
            }
            TOP_RIGHT => {
                cs.push_left(c);
                cs.push_bottom(c);
            }
            BOTTOM_LEFT => {
                cs.push_right(c);
                cs.push_top(c);
            }
            BOTTOM_RIGHT => {
                cs.push_left(c);
                cs.push_top(c);
            }
            b'.' => {}
            _ => panic!(),
        }
    }

    highest
}

fn part2(_input: &str) -> u64 {
    0
}

helper::tests! {
    day10 Day10;
    part1 {
        "../input_small11.txt" => 4;
        "../input_small12.txt" => 8;
        "../input.txt" => 0;
    }
    part2 {
        "../input.txt" => 0;
    }
}
helper::benchmarks! {}
