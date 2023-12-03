use std::hash::BuildHasherDefault;

use rustc_hash::FxHashMap;

#[derive(Debug)]
struct GearRatio(u64, u8);

impl Default for GearRatio {
    fn default() -> Self {
        Self(1, 0)
    }
}

impl GearRatio {
    fn push(&mut self, value: u64) {
        self.0 *= value;
        self.1 += 1;
    }
    fn get(&self) -> Option<u64> {
        (self.1 == 2).then_some(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::GearRatio;

    #[test]
    fn gear_ratio() {
        let mut r = GearRatio::default();
        assert_eq!(r.get(), None);
        r.push(2);
        assert_eq!(r.get(), None);
        r.push(3);
        eprintln!("{:b}", r.0);
        assert_eq!(r.get(), Some(6));
        r.push(1);
        assert_eq!(r.get(), None);
    }
}

pub fn part2(input: &str) -> u64 {
    fn contains_gear(s: &str) -> Option<usize> {
        s.chars().position(|c| !c.is_ascii_digit() && c != '.')
    }

    let len = input.lines().next().unwrap().len();
    let empty_border = std::iter::repeat('.').take(len).collect::<String>();

    let mut prev2 = empty_border.as_str();
    let mut prev1 = input.lines().next().unwrap();

    let mut gears: FxHashMap<u32, GearRatio> =
        FxHashMap::with_capacity_and_hasher(1000, BuildHasherDefault::default());

    for (line_number, next) in input
        .lines()
        .skip(1)
        .chain(Some(empty_border.as_str()))
        .enumerate()
    {
        let mut numbers = prev1.char_indices().peekable();
        while let Some((start, c)) = numbers.next() {
            if c.is_ascii_digit() {
                let mut end = start;
                while let Some((idx, '0'..='9')) = numbers.next() {
                    end = idx;
                }

                let box_bounds = (start.saturating_sub(1))..std::cmp::min(end + 2, len);
                let number = prev1[start..=end].parse::<u64>().unwrap();

                let mut push = |key: (usize, usize)| {
                    gears
                        .entry(((key.1 as u32) << 16) | (key.0 as u32))
                        .or_default()
                        .push(number)
                };

                if let Some(position) = contains_gear(&prev2[box_bounds.clone()]) {
                    let key = (line_number - 1, box_bounds.start + position);
                    push(key);
                }

                if let Some(position) = contains_gear(&prev1[box_bounds.clone()]) {
                    let key = (line_number, box_bounds.start + position);
                    push(key);
                }

                if let Some(position) = contains_gear(&next[box_bounds.clone()]) {
                    let key = (line_number + 1, box_bounds.start + position);
                    push(key);
                }
            }
        }

        prev2 = prev1;
        prev1 = next;
    }

    gears.values().filter_map(|v| v.get()).sum()
}
