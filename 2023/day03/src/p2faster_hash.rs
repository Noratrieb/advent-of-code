use rustc_hash::FxHashMap;

pub fn part2(input: &str) -> u64 {
    fn contains_gear(s: &str) -> Option<usize> {
        s.chars().position(|c| !c.is_ascii_digit() && c != '.')
    }

    let len = input.lines().next().unwrap().len();
    let empty_border = std::iter::repeat('.').take(len).collect::<String>();

    let mut prev2 = empty_border.as_str();
    let mut prev1 = input.lines().next().unwrap();

    let mut gears: FxHashMap<u64, Vec<u64>> = FxHashMap::default();

    for (line_number, next) in input
        .lines()
        .skip(1)
        .chain(Some(empty_border.as_str()))
        .enumerate()
    {
        assert!(line_number < u32::MAX as _);
        let mut numbers = prev1.char_indices().peekable();
        while let Some((start, c)) = numbers.next() {
            if c.is_ascii_digit() {
                let mut end = start;
                while let Some((idx, '0'..='9')) = numbers.next() {
                    end = idx;
                }

                let box_bounds = (start.saturating_sub(1))..std::cmp::min(end + 2, len);
                let number = prev1[start..=end].parse::<u64>().unwrap();

                let mut push = |line, pos| {
                    gears
                        .entry((((box_bounds.start + pos) as u64) << 32) | (line as u64))
                        .or_default()
                        .push(number)
                };

                if let Some(position) = contains_gear(&prev2[box_bounds.clone()]) {
                    push(line_number - 1, position);
                }

                if let Some(position) = contains_gear(&prev1[box_bounds.clone()]) {
                    push(line_number, position);
                }

                if let Some(position) = contains_gear(&next[box_bounds.clone()]) {
                    push(line_number + 1, position);
                }
            }
        }

        prev2 = prev1;
        prev1 = next;
    }

    gears
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum()
}
