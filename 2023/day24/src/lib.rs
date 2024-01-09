use helper::{Day, IteratorExt, Variants};

pub fn main() {
    helper::main::<Day24>(include_str!("../input.txt"));
}

struct Day24;

helper::define_variants! {
    day => crate::Day24;
    part1 {
        basic => crate::part1;
    }
    part2 {
        basic => crate::part2;
    }
}

impl Day for Day24 {
    fn part1() -> Variants {
        part1_variants!(construct_variants)
    }

    fn part2() -> Variants {
        part2_variants!(construct_variants)
    }
}

struct Vector {
    x: i64,
    y: i64,
    #[allow(dead_code)]
    z: i64,
}

fn parse(line: &str) -> (Vector, Vector) {
    let [pos, vel] = line.split('@').collect_array().unwrap();

    let [x, y, z] = pos
        .split(",")
        .map(|line| line.trim().parse().unwrap())
        .collect_array()
        .unwrap();
    let pos = Vector { x, y, z };
    let [x, y, z] = vel
        .split(",")
        .map(|line| line.trim().parse().unwrap())
        .collect_array()
        .unwrap();
    let vel = Vector { x, y, z };
    (pos, vel)
}

fn intersect(a1: f64, b1: f64, a2: f64, b2: f64) -> Option<(f64, f64)> {
    // f1(x) = f1(x)
    //       a1 * x + b1 = a2 * x + b2            | - b1
    //            a1 * x = a2 * x + b2 - b1       | - (a2 * x)
    // a1 * x - (a2 * x) = b2 - b1                |
    //     x * (a1 - a2) = b2 - b1                | / (a1 - a2)
    //                 x = (b2 - b1) / (a1 - a2)

    let x = (b2 - b1) / (a1 - a2);

    if x.is_nan() {
        // a1=a2, so they're parallel
        return None;
    }

    let y = (a1 * x) + b1;

    Some((x, y))
}

fn part1(input: &str) -> u64 {
    let paths = input
        .lines()
        .map(|line| {
            let (pos, vel) = parse(line);

            let x1 = pos.x;
            let y1 = pos.y;

            let delta_x = vel.x as f64;
            let delta_y = vel.y as f64;

            // f(x) = ax + b

            let a = delta_y / delta_x;

            //      y = ax + b
            // y - ax = b
            let b = (y1 as f64) - (a * (x1 as f64));

            (a, b)
        })
        .collect_vec();

    let mut total = 0;

    let range = if paths.len() > 100 {
        200000000000000.0..400000000000000.0
    } else {
        7.0..27.0
    };

    // TODO: i dont think floats are gonna work....
    for (i, stone) in paths.iter().enumerate() {
        let candidates = &paths[(i + 1)..];
        for (j, candidate) in candidates.iter().enumerate() {
            if let Some((x, y)) = intersect(stone.0, stone.1, candidate.0, candidate.1) {
                // TODO: check for past, only the future matters
                if range.contains(&x) && range.contains(&y) {
                    dbg!((i, j, x, y));
                    total += 1;
                }
            }
        }
    }

    total
}

fn part2(_input: &str) -> u64 {
    0
}

helper::tests! {
    day24 Day24;
    part1 {
        small => 0;
        default => 0;
    }
    part2 {
        small => 0;
        default => 0;
    }
}
helper::benchmarks! {}
