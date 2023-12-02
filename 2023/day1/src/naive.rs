pub fn part2(input: &str) {
    let sum = input
        .lines()
        .map(|line| {
            let line = line
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine");
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

    println!("part2: {sum}");
}
