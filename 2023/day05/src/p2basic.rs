use std::{cmp, ops::Range};

pub fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let seeds = seeds.chunks(2);

    let maps = super::parse_maps(lines);

    let mut min_loc = u64::MAX;

    let mut current_seeds: Vec<(usize, usize, Range<u64>)> = seeds
        .map(|seed| (0, 0, seed[0]..(seed[0] + seed[1])))
        .collect();

    'queue: while let Some((stage, range_idx, numbers)) = current_seeds.pop() {
        if stage == maps.len() {
            // Range is done.
            min_loc = cmp::min(min_loc, numbers.start);
            continue;
        }
        let ranges = &maps[stage];
        for range in ranges.iter().skip(range_idx) {
            if range.source_end() <= numbers.start || range.source_start >= numbers.end {
                // Completely out of range.
                continue;
            }

            // Part that falls before this redirection, needs to be queued again.
            let pre = numbers.start..range.source_start;
            let in_ = cmp::max(numbers.start, range.source_start)
                ..cmp::min(numbers.end, range.source_end());
            let post = range.source_end()..(numbers.end);

            if !pre.is_empty() {
                current_seeds.push((stage, range_idx + 1, pre));
            }
            if !post.is_empty() {
                current_seeds.push((stage, range_idx + 1, post));
            }
            let offset = in_.start - range.source_start;
            let new =
                (range.dest_start + offset)..(range.dest_start + offset + (in_.end - in_.start));

            if !new.is_empty() {
                current_seeds.push((stage + 1, 0, new));
            }
            continue 'queue;
        }
        // No change, pass unaffected
        current_seeds.push((stage + 1, 0, numbers));
    }

    min_loc
}
