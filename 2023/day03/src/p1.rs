pub fn basic(input: &str) -> u64 {
    fn contains_symbol(s: &str) -> bool {
        s.chars().any(|c| !c.is_ascii_digit() && c != '.')
    }

    let len = input.lines().next().unwrap().len();
    let empty_border = ".".repeat(len);

    let mut prev2 = empty_border.as_str();
    let mut prev1 = input.lines().next().unwrap();

    let mut acc = 0;

    for next in input.lines().skip(1).chain(Some(empty_border.as_str())) {
        let mut numbers = prev1.char_indices().peekable();
        while let Some((start, c)) = numbers.next() {
            if c.is_ascii_digit() {
                let mut end = start;
                while let Some((idx, '0'..='9')) = numbers.next() {
                    end = idx;
                }

                let box_bounds = (start.saturating_sub(1))..std::cmp::min(end + 2, len);
                let number = prev1[start..=end].parse::<u64>().unwrap();

                if contains_symbol(&prev2[box_bounds.clone()])
                    || contains_symbol(&prev1[box_bounds.clone()])
                    || contains_symbol(&next[box_bounds])
                {
                    acc += number;
                }
            }
        }

        prev2 = prev1;
        prev1 = next;
    }

    acc
}

pub fn bytes(input: &str) -> u64 {
    fn contains_symbol(s: &str) -> bool {
        s.bytes().any(|c| !c.is_ascii_digit() && c != b'.')
    }

    let len = input.lines().next().unwrap().len();
    let empty_border = ".".repeat(len);

    let mut prev2 = empty_border.as_str();
    let mut prev1 = input.lines().next().unwrap();

    let mut acc = 0;

    for next in input.lines().skip(1).chain(Some(empty_border.as_str())) {
        let mut numbers = prev1.bytes().enumerate().peekable();
        while let Some((start, c)) = numbers.next() {
            if c.is_ascii_digit() {
                let mut end = start;
                while let Some((idx, b'0'..=b'9')) = numbers.next() {
                    end = idx;
                }

                let box_bounds = (start.saturating_sub(1))..std::cmp::min(end + 2, len);
                let number = prev1[start..=end].parse::<u64>().unwrap();

                if contains_symbol(&prev2[box_bounds.clone()])
                    || contains_symbol(&prev1[box_bounds.clone()])
                    || contains_symbol(&next[box_bounds])
                {
                    acc += number;
                }
            }
        }

        prev2 = prev1;
        prev1 = next;
    }

    acc
}
