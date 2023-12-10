use std::collections::VecDeque;

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

struct Candidates<'a> {
    v: VecDeque<Candidate>,
    width: usize,
    len: usize,
    bytes: &'a [u8],
}

#[derive(Clone, Copy)]
struct Candidate {
    count: u64,
    pos: usize,
    came_from: Direction,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Top,
    Bottom,
    None,
}

fn left(pos: usize, width: usize) -> Option<usize> {
    (pos % width > 0).then(|| pos - 1)
}
fn right(pos: usize, width: usize) -> Option<usize> {
    ((pos % width) < (width - 1)).then(|| pos + 1)
}
fn top(pos: usize, width: usize) -> Option<usize> {
    (pos >= width).then(|| pos - width)
}
fn bottom(pos: usize, len: usize, width: usize) -> Option<usize> {
    (pos < (len - width)).then(|| pos + width)
}

impl Candidates<'_> {
    fn push(&mut self, new: Candidate) {
        match (new.came_from, self.bytes[new.pos]) {
            (Direction::Left, VERTICAL | TOP_LEFT | BOTTOM_LEFT) => {}
            (Direction::Right, VERTICAL | TOP_RIGHT | BOTTOM_RIGHT) => {}
            (Direction::Top, HORIZONTAL | TOP_LEFT | TOP_RIGHT) => {}
            (Direction::Bottom, HORIZONTAL | BOTTOM_LEFT | BOTTOM_RIGHT) => {}
            _ => self.v.push_back(new),
        }
    }

    fn push_left(&mut self, from: Candidate) {
        if from.came_from != Direction::Left {
            if let Some(left) = left(from.pos, self.width) {
                self.push(Candidate::new(from.count + 1, left, Direction::Right));
            }
        }
    }
    fn push_right(&mut self, from: Candidate) {
        if from.came_from != Direction::Right {
            if let Some(right) = right(from.pos, self.width) {
                self.push(Candidate::new(from.count + 1, right, Direction::Left));
            }
        }
    }
    fn push_top(&mut self, from: Candidate) {
        if from.came_from != Direction::Top {
            if let Some(top) = top(from.pos, self.width) {
                self.push(Candidate::new(from.count + 1, top, Direction::Bottom));
            }
        }
    }
    fn push_bottom(&mut self, from: Candidate) {
        if from.came_from != Direction::Bottom {
            if let Some(bottom) = bottom(from.pos, self.bytes.len(), self.width) {
                self.push(Candidate::new(from.count + 1, bottom, Direction::Top));
            }
        }
    }
}

impl Candidate {
    fn new(count: u64, pos: usize, skip: Direction) -> Self {
        Self {
            count,
            pos,
            came_from: skip,
        }
    }
}

struct Loop {
    step_map: Vec<(u64, bool)>,
    target: usize,
    highest_value: u64,
}

fn get_loop(input: &str) -> Loop {
    let bytes = input.as_bytes();
    let width = bytes.into_iter().position(|&b| b == b'\n').unwrap();

    let bytes = bytes
        .iter()
        .copied()
        .filter(|&b| b != b'\n')
        .collect::<Vec<_>>();

    let mut step_map = bytes.iter().map(|_| (0, false)).collect::<Vec<_>>();

    let s = bytes.iter().position(|&b| b == S).unwrap();

    let mut cs = Candidates {
        v: VecDeque::new(),
        len: bytes.len(),
        width,
        bytes: bytes.as_slice(),
    };

    cs.v.push_back(Candidate {
        count: 0,
        pos: s,
        came_from: Direction::None,
    });

    let mut highest_value = 0;

    let print = false;

    #[cfg(test)]
    if print {
        panic!("cannot test with print");
    }

    let mut target = usize::MAX;

    while let Some(c) = cs.v.pop_front() {
        if print {
            for (i, _) in bytes.as_slice().iter().enumerate() {
                if (i as usize) % width == 0 {
                    println!();
                }
                if c.pos == i {
                    print!("NOW   ");
                } else if cs.v.iter().any(|c| c.pos == i) {
                    print!("CAND  ");
                } else if step_map[i].1 {
                    print!("{:<5} ", step_map[i].0);
                } else {
                    print!(".     ");
                }
            }
            println!();
        }

        if step_map[c.pos].1 {
            highest_value = highest_value.max(step_map[c.pos].0);
            target = c.pos;
            break;
        }
        step_map[c.pos] = (c.count, true);
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
    if print {
        for (i, _) in bytes.as_slice().iter().enumerate() {
            if (i as usize) % width == 0 {
                println!();
            }
            if step_map[i].1 {
                print!("{:<5} ", step_map[i].0);
            } else {
                print!(".     ");
            }
        }
        println!();
    }

    Loop {
        step_map,
        target,
        highest_value,
    }
}

fn part1(input: &str) -> u64 {
    get_loop(input).highest_value
}

fn part2(input: &str) -> u64 {
    // Step 1: Find the loop
    //         We do this by using the step-map from before, counting backwards from the target basically.
    // Step 2: Cellular-automata-ish, start from the borders and start eating away
    //         everything connected to that, only stopping at the main loop.
    // Open question: How do we squeeze between main loop pipes?
    let the_loop = get_loop(input);
    let mut tiles = vec![0; the_loop.step_map.len()];

    0
}

helper::tests! {
    day10 Day10;
    part1 {
        "../input_small11.txt" => 4;
        "../input_small12.txt" => 8;
        "../input_small13.txt" => 11;
        "../input_small14.txt" => 13;
        "../input.txt" => 6903;
    }
    part2 {
        "../input.txt" => 0;
    }
}
helper::benchmarks! {}
