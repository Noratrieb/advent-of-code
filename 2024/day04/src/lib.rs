use helper::{Day, Variants};

pub fn main() {
    helper::main::<Day04>(include_str!("../input.txt"));
}

struct Day04;

helper::define_variants! {
    day => crate::Day04;
    part1 {
        basic => crate::part1;
    }
    part2 {
        basic => crate::part2;
        prepare_better => crate::part2_prepare_better;
        u64 => crate::part2_u64;
        simd => crate::part2_simd;
        reading => crate::part2_reading;
    }
}

impl Day for Day04 {
    fn part1() -> Variants {
        part1_variants!(construct_variants)
    }

    fn part2() -> Variants {
        part2_variants!(construct_variants)
    }
}

fn part1(input: &str) -> u64 {
    const XMAS: &[u8] = b"XMAS";
    const SAMX: &[u8] = b"SAMX";
    let mut count = 0;

    let lines = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    // Horizontal rows
    for line in &lines {
        for i in 0..line.len() {
            if line[i..].starts_with(XMAS) || line[i..].starts_with(SAMX) {
                count += 1;
            }
        }
    }

    let line_length = lines[0].len();

    let cols = (0..line_length).map(|i| lines.iter().map(|line| line[i]).collect::<Vec<_>>());

    for col in cols {
        // dbg!(std::str::from_utf8(&col));
        for i in 0..col.len() {
            if col[i..].starts_with(XMAS) || col[i..].starts_with(SAMX) {
                count += 1;
            }
        }
    }

    let all = lines
        .iter()
        .flat_map(|line| *line)
        .copied()
        .collect::<Vec<_>>();

    for i in 0..all.len() {
        if i % line_length >= (line_length - 3) {
            continue;
        }
        (|| {
            let offset = line_length + 1;
            let a = *all.get(i)?;
            let b = *all.get(i + offset * 1)?;
            let c = *all.get(i + offset * 2)?;
            let d = *all.get(i + offset * 3)?;
            let chunk = [a, b, c, d];
            if chunk == XMAS || chunk == SAMX {
                count += 1;
            }

            Some(())
        })();
    }

    for i in 0..all.len() {
        if i % line_length < 3 {
            continue;
        }
        (|| {
            let offset = line_length - 1;
            let a = *all.get(i)?;
            let b = *all.get(i + offset * 1)?;
            let c = *all.get(i + offset * 2)?;
            let d = *all.get(i + offset * 3)?;
            let chunk = [a, b, c, d];
            if chunk == XMAS || chunk == SAMX {
                count += 1;
            }

            Some(())
        })();
    }

    count
}

#[allow(dead_code)]
fn print_chunk(chunk: u128) -> String {
    use std::fmt::Write;
    let mut s = String::new();
    let c = |c: u128| {
        let c = c as u8 as char;
        if c == '\0' {
            '.'
        } else {
            c
        }
    };
    write!(
        s,
        "{}{}{}|{}{}{}|{}{}{}",
        c((chunk >> 64) & 0xFF),
        c((chunk >> 56) & 0xFF),
        c((chunk >> 48) & 0xFF),
        c((chunk >> 40) & 0xFF),
        c((chunk >> 32) & 0xFF),
        c((chunk >> 24) & 0xFF),
        c((chunk >> 16) & 0xFF),
        c((chunk >> 8) & 0xFF),
        c(chunk & 0xFF),
    )
    .unwrap();
    s
}

fn part2(input: &str) -> u64 {
    #[rustfmt::skip]
    const XMAS_COMBINATIONS: &[u128] = &[
        u128::from_be_bytes([
            0,0,0,0,0,0,0,
            b'M',   0, b'S',
               0, b'A',  0,
            b'M',   0, b'S',
        ]),
        u128::from_be_bytes([
            0,0,0,0,0,0,0,
            b'M',   0, b'M',
               0, b'A',  0,
            b'S',   0, b'S',
        ]),
        u128::from_be_bytes([
            0,0,0,0,0,0,0,
            b'S',   0, b'S',
               0, b'A',  0,
            b'M',   0, b'M',
        ]),
        u128::from_be_bytes([
            0,0,0,0,0,0,0,
            b'S',   0, b'M',
               0, b'A',  0,
            b'S',   0, b'M',
        ]),
    ];

    let all = input
        .lines()
        .flat_map(|line| line.as_bytes())
        .copied()
        .collect::<Vec<_>>();
    let line_length = input.lines().next().unwrap().len();

    let chunk_count = line_length - 2;

    let end = all.len() - line_length * 2;

    let mut count = 0;

    let mut i = 0;
    while i < end {
        for _ in 0..chunk_count {
            let chunk_top = &all[i..][..3];
            let chunk_mid = &all[(i + line_length)..][..3];
            let chunk_bot = &all[(i + line_length * 2)..][..3];

            #[rustfmt::skip]
            let full_chunk = [
                chunk_top[0], chunk_top[1], chunk_top[2],
                chunk_mid[0], chunk_mid[1], chunk_mid[2],
                chunk_bot[0], chunk_bot[1], chunk_bot[2],
            ];
            let mut be = [0; 16];
            be[(16 - 9)..].copy_from_slice(&full_chunk);
            let int = u128::from_be_bytes(be);

            const XMAS_MASK: u128 = 0xFF00FF_00FF00_FF00FF;

            let int_relevant = int & XMAS_MASK;

            if XMAS_COMBINATIONS.contains(&int_relevant) {
                count += 1;
            }

            i += 1;
        }
        // skip end
        i += 2;
    }

    count
}

fn part2_prepare_better(input: &str) -> u64 {
    #[rustfmt::skip]
    const XMAS_COMBINATIONS: &[u128] = &[
        u128::from_be_bytes([
            0,0,0,0,0,0,0,
            b'M',   0, b'S',
               0, b'A',  0,
            b'M',   0, b'S',
        ]),
        u128::from_be_bytes([
            0,0,0,0,0,0,0,
            b'M',   0, b'M',
               0, b'A',  0,
            b'S',   0, b'S',
        ]),
        u128::from_be_bytes([
            0,0,0,0,0,0,0,
            b'S',   0, b'S',
               0, b'A',  0,
            b'M',   0, b'M',
        ]),
        u128::from_be_bytes([
            0,0,0,0,0,0,0,
            b'S',   0, b'M',
               0, b'A',  0,
            b'S',   0, b'M',
        ]),
    ];

    let mut all = Vec::with_capacity(input.len());
    let line_length = input.lines().next().unwrap().len();
    let input = input.as_bytes();
    let mut i = 0;
    while i < input.len() {
        let next = &input[i..][..line_length];
        all.extend_from_slice(next);
        i += line_length + 1;
    }

    let chunk_count = line_length - 2;

    let end = all.len() - line_length * 2;

    let mut count = 0;

    let mut i = 0;
    while i < end {
        for _ in 0..chunk_count {
            let chunk_top = &all[i..][..3];
            let chunk_mid = &all[(i + line_length)..][..3];
            let chunk_bot = &all[(i + line_length * 2)..][..3];

            #[rustfmt::skip]
            let full_chunk = [
                chunk_top[0], chunk_top[1], chunk_top[2],
                chunk_mid[0], chunk_mid[1], chunk_mid[2],
                chunk_bot[0], chunk_bot[1], chunk_bot[2],
            ];
            let mut be = [0; 16];
            be[(16 - 9)..].copy_from_slice(&full_chunk);
            let int = u128::from_be_bytes(be);

            const XMAS_MASK: u128 = 0xFF00FF_00FF00_FF00FF;

            let int_relevant = int & XMAS_MASK;

            if XMAS_COMBINATIONS.contains(&int_relevant) {
                count += 1;
            }

            i += 1;
        }
        // skip end
        i += 2;
    }

    count
}

fn part2_u64(input: &str) -> u64 {
    #[rustfmt::skip]
    const XMAS_COMBINATIONS: &[(u64, u8)] = &[
        (u64::from_be_bytes([
            b'M',   0, b'S',
               0, b'A',  0,
            b'M',   0,
        ]), b'S'),
        (u64::from_be_bytes([
            b'M',   0, b'M',
               0, b'A',  0,
            b'S',   0,
        ]), b'S'),
        (u64::from_be_bytes([
            b'S',   0, b'S',
               0, b'A',  0,
            b'M',   0
        ]), b'M'),
        (u64::from_be_bytes([
            b'S',   0, b'M',
               0, b'A',  0,
            b'S',   0
        ]), b'M'),
    ];

    let mut all = Vec::with_capacity(input.len());
    let line_length = input.lines().next().unwrap().len();
    let input = input.as_bytes();
    let mut i = 0;
    while i < input.len() {
        let next = &input[i..][..line_length];
        all.extend_from_slice(next);
        i += line_length + 1;
    }

    let chunk_count = line_length - 2;

    let end = all.len() - line_length * 2;

    let mut count = 0;

    let mut i = 0;
    while i < end {
        for _ in 0..chunk_count {
            let chunk_top = &all[i..][..3];
            let chunk_mid = &all[(i + line_length)..][..3];
            let chunk_bot = &all[(i + line_length * 2)..][..3];

            #[rustfmt::skip]
            let full_chunk = [
                chunk_top[0], chunk_top[1], chunk_top[2],
                chunk_mid[0], chunk_mid[1], chunk_mid[2],
                chunk_bot[0], chunk_bot[1],
            ];
            let int = u64::from_be_bytes(full_chunk);

            const XMAS_MASK: u64 = 0xFF00FF_00FF00_FF00;

            let int_relevant = int & XMAS_MASK;

            for &(most, rest) in XMAS_COMBINATIONS {
                if most == int_relevant && chunk_bot[2] == rest {
                    count += 1;
                    break;
                }
            }

            i += 1;
        }
        // skip end
        i += 2;
    }

    count
}

fn part2_simd(input: &str) -> u64 {
    helper::only_x86_64_and! { "avx2" =>
        input, do_avx else part2_u64
    }

    #[target_feature(enable = "avx2")]
    unsafe fn do_avx(input: &str) -> u64 {
        use std::arch::x86_64;

        #[rustfmt::skip]
        const XMAS_COMBINATIONS: &[(u64, u8)] = &[
            (u64::from_le_bytes([
                b'M',   0, b'S',
                0, b'A',  0,
                b'M',   0,
            ]), b'S'),
            (u64::from_le_bytes([
                b'M',   0, b'M',
                0, b'A',  0,
                b'S',   0,
            ]), b'S'),
            (u64::from_le_bytes([
                b'S',   0, b'S',
                0, b'A',  0,
                b'M',   0
            ]), b'M'),
            (u64::from_le_bytes([
                b'S',   0, b'M',
                0, b'A',  0,
                b'S',   0
            ]), b'M'),
        ];

        let all = input.as_bytes();
        let filled_line_len = input.lines().next().unwrap().len();
        let full_line_len = filled_line_len + 1;

        let chunk_count = filled_line_len - 2;

        let end = all.len() - full_line_len * 2;

        let mut count = 0;

        let mut i = 0;

        let combinations = x86_64::_mm256_set_epi64x(
            XMAS_COMBINATIONS[0].0 as i64,
            XMAS_COMBINATIONS[1].0 as i64,
            XMAS_COMBINATIONS[2].0 as i64,
            XMAS_COMBINATIONS[3].0 as i64,
        );

        while i < end {
            for _ in 0..chunk_count {
                let chunk_top = &all[i..][..3];
                let chunk_mid = &all[(i + full_line_len)..][..3];
                let chunk_bot = &all[(i + full_line_len * 2)..][..3];

                #[rustfmt::skip]
                let full_chunk = [
                    chunk_top[0], chunk_top[1], chunk_top[2],
                    chunk_mid[0], chunk_mid[1], chunk_mid[2],
                    chunk_bot[0], chunk_bot[1],
                ];
                let int = u64::from_le_bytes(full_chunk);

                let to_test = x86_64::_mm256_set1_epi64x(
                    (int & 0xFF00FF_00FF00_FF00_u64.swap_bytes()) as i64,
                );

                let eq = x86_64::_mm256_cmpeq_epi64(to_test, combinations);

                let movmask = x86_64::_mm256_movemask_epi8(eq);
                if movmask != 0 {
                    let check = match movmask as u32 {
                        0xFF000000 => XMAS_COMBINATIONS[0].1,
                        0x00FF0000 => XMAS_COMBINATIONS[1].1,
                        0x0000FF00 => XMAS_COMBINATIONS[2].1,
                        0x000000FF => XMAS_COMBINATIONS[3].1,
                        _ => unreachable!(),
                    };
                    if check == chunk_bot[2] {
                        count += 1;
                    }
                }

                i += 1;
            }
            // skip end
            i += 3;
        }

        count
    }
}

fn part2_reading(input: &str) -> u64 {
    helper::only_x86_64_and! { "avx2" =>
        input, do_avx else part2_u64
    }

    #[target_feature(enable = "avx2")]
    unsafe fn do_avx(input: &str) -> u64 {
        use std::arch::x86_64;

        #[rustfmt::skip]
        const XMAS_COMBINATIONS: &[(u64, u8)] = &[
            (u64::from_le_bytes([
                b'S',   0, b'M',
                0, b'A',  0,
                b'S',   0
            ]), b'M'),
            (u64::from_le_bytes([
                b'S',   0, b'S',
                0, b'A',  0,
                b'M',   0
            ]), b'M'),
            (u64::from_le_bytes([
                b'M',   0, b'M',
                0, b'A',  0,
                b'S',   0,
            ]), b'S'),
            (u64::from_le_bytes([
                b'M',   0, b'S',
                0, b'A',  0,
                b'M',   0,
            ]), b'S'),
        ];

        let all = input.as_bytes();
        let filled_line_len = input.lines().next().unwrap().len();
        let full_line_len = filled_line_len + 1;

        let chunk_count = filled_line_len - 2;

        let end = all.len() - full_line_len * 2;

        let mut count = 0;

        let mut i = 0;

        let combinations = x86_64::_mm256_set_epi64x(
            XMAS_COMBINATIONS[3].0 as i64,
            XMAS_COMBINATIONS[2].0 as i64,
            XMAS_COMBINATIONS[1].0 as i64,
            XMAS_COMBINATIONS[0].0 as i64,
        );

        while i < end {
            for _ in 0..chunk_count {
                let chunk_top = all.as_ptr().add(i).cast::<u32>().read_unaligned() as u64;
                let chunk_mid = all
                    .as_ptr()
                    .add(i + full_line_len)
                    .cast::<u32>()
                    .read_unaligned() as u64;
                let chunk_bot = all
                    .as_ptr()
                    .add(i + full_line_len * 2)
                    .cast::<u32>()
                    .read_unaligned() as u64;

                let int = (chunk_top & 0xFF00FF)
                    | ((chunk_mid & 0x00FF00) << 24)
                    | ((chunk_bot & 0xFF00FF) << (24 * 2));

                let to_test = x86_64::_mm256_set1_epi64x(int as i64);

                let eq = x86_64::_mm256_cmpeq_epi64(to_test, combinations);

                let movmask = x86_64::_mm256_movemask_epi8(eq);
                if movmask != 0 {
                    let check =
                        XMAS_COMBINATIONS[((movmask as u32).trailing_zeros() / 8) as usize].1;
                    if check == ((chunk_bot >> 16) & 0xFF) as u8 {
                        count += 1;
                    }
                }

                i += 1;
            }
            // skip end
            i += 3;
        }

        count
    }
}

helper::tests! {
    day04 Day04;
    part1 {
        small => 18;
        default => 2562;
    }
    part2 {
        small => 9;
        default => 1902;
    }
}
helper::benchmarks! {}
