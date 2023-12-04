use helper::IteratorExt;
use nom::InputIter;

#[target_feature(enable = "avx2")]
#[cfg(target_arch = "x86_64")]
unsafe fn line_match_count_avx2(line: &str) -> usize {
    let line = line.as_bytes();
    fn pack(chunk: &[u8]) -> u16 {
        ((chunk[1] as u16) << 8) | (chunk[2] as u16)
    }

    let colon = line.position(|b| b == b':').unwrap();
    let numbers = &line[(colon + 1)..];
    let pipe = numbers.position(|b| b == b'|').unwrap();
    let winning = &numbers[..(pipe - 1)];
    let you_have = &numbers[(pipe + 1)..];

    let winning = winning
        .chunks_exact(3)
        .map(pack)
        .collect_array_default::<16>()
        .unwrap();
    let you_have = you_have.chunks_exact(3).map(pack);

    unsafe {
        use std::arch::x86_64;

        let winning = std::mem::transmute::<_, x86_64::__m256i>(winning);

        you_have
            .filter(|&have| {
                let have = x86_64::_mm256_set1_epi16(have as _);
                let eq = x86_64::_mm256_cmpeq_epi16(winning, have);

                x86_64::_mm256_movemask_epi8(eq) > 0
            })
            .count()
    }
}

fn line_match_count(line: &str) -> usize {
    let line = line.as_bytes();
    fn pack(chunk: &[u8]) -> u16 {
        ((chunk[1] as u16) << 8) | (chunk[2] as u16)
    }

    let colon = line.position(|b| b == b':').unwrap();
    let numbers = &line[(colon + 1)..];
    let pipe = numbers.position(|b| b == b'|').unwrap();
    let winning = &numbers[..(pipe - 1)];
    let you_have = &numbers[(pipe + 1)..];

    let winning = winning
        .chunks_exact(3)
        .map(pack)
        .collect_array_default::<16>()
        .unwrap();
    let you_have = you_have.chunks_exact(3).map(pack);

    you_have
        .filter(|have| winning.iter().any(|w| w == have))
        .count()
}

pub fn part2(input: &str) -> u64 {
    let avx2 = std::arch::is_x86_feature_detected!("avx2");
    let lines = if avx2 {
        #[cfg(target_arch = "x86_64")]
        {
            input
                .lines()
                .map(|line| unsafe { line_match_count_avx2(line) })
                .collect::<Vec<_>>()
        }
        #[cfg(not(target_arch = "x86_64"))]
        Vec::new()
    } else {
        input
            .lines()
            .map(|line| line_match_count(line))
            .collect::<Vec<_>>()
    };

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
