fn main() {
    let kind = std::env::args().nth(1).unwrap_or("naive".into());

    let input = std::hint::black_box(include_str!("../input.txt")).to_owned();

    let result = match kind.as_str() {
        "part1" => part1(&input),
        "part2" => part2(&input),
        _ => {
            eprintln!("error: invalid mode, must be part1");
            std::process::exit(1);
        }
    };

    println!("result: {result}");
}

fn part1(input: &str) -> u64 {
    fn contains_symbol(s: &str) -> bool {
        s.chars().any(|c| !c.is_ascii_digit() && c != '.')
    }

    let len = input.lines().next().unwrap().len();
    let empty_border = std::iter::repeat('.').take(len).collect::<String>();

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

fn part2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_small() {
        assert_eq!(super::part1(include_str!("../input_small.txt")), 8);
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(include_str!("../input.txt")), 1931);
    }

    #[test]
    fn part2_small() {
        assert_eq!(super::part2(include_str!("../input_small.txt")), 2286);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(include_str!("../input.txt")), 83105);
    }
}
