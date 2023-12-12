fn line_match_count(line: &str) -> usize {
    let mut numbers = line.split(':').nth(1).unwrap().split("|");
    let winning = numbers
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<arrayvec::ArrayVec<_, 16>>();

    let you_have = numbers.next().unwrap().split_whitespace();

    you_have
        .filter(|have| winning.iter().any(|w| w == have))
        .count()
}

pub fn part2(input: &str) -> u64 {
    let lines = input.lines().map(line_match_count).collect::<Vec<_>>();

    let mut cache = vec![0; lines.len()];

    let mut processed = 0;

    // By iterating backwards, we ensure the cache is always populated for every subsequent line.
    for (i, result) in lines.iter().copied().enumerate().rev() {
        let before = processed;
        processed += 1; // Every card gives us one card.
                        // Now, let's see how many cards this card will expand to.
        for expand in (i + 1)..((i + 1) + result) {
            #[cfg(debug_assertions)]
            eprintln!(
                "{} expands to {} which is worth {}",
                i + 1,
                expand + 1,
                cache[expand]
            );
            // Since the value is bigger than i, it must be cached!
            processed += cache[expand];
        }
        cache[i] = processed - before;
    }

    processed
}
