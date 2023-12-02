use std::mem::MaybeUninit;

mod branchless;
mod naive;
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
        _ => {
            eprintln!("error: invalid mode, must be part1,naive,zero_alloc,branchless");
            std::process::exit(1);
        }
    }
}

fn part1(input: &str) {
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

    println!("part1: {sum}");
}
