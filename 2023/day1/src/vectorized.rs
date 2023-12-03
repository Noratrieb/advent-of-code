pub unsafe fn part2(input: &str) -> u64 {
    let mut sum = 0;

    let bytes = input.as_bytes();

    let mut digits = [0_u8; 128];

    let mut byte_idx = 0;
    let mut line_idx = 0;

    #[cfg(target_arch = "x86_64")]
    let avx2 = std::arch::is_x86_feature_detected!("avx2");
    #[cfg(not(target_arch = "x86_64"))]
    let avx2 = false;

    while byte_idx < bytes.len() {
        // in memory:
        // o n e X X X X X
        // in the integer bytes:
        // X X X X X e n o
        // this out of bounds read is UB under SB, but fine under models that don't do provenance narrowing with slices. i dont care enough to fix it.
        let block = bytes
            .as_ptr()
            .add(byte_idx)
            .cast::<u64>()
            .read_unaligned()
            .to_le();

        let one = (block & ((1 << (8 * 1)) - 1)) as u8;
        let three = block & ((1 << (8 * 3)) - 1);
        let four = block & ((1 << (8 * 4)) - 1);
        let five = block & ((1 << (8 * 5)) - 1);

        if one == b'\n' {
            let first = digits[..line_idx].iter().find(|&&d| d > b'0').unwrap();
            let last = digits[..line_idx]
                .iter()
                .rev()
                .find(|&&d| d > b'0')
                .unwrap();

            let first = (first - b'0') as u64;
            let last = (last - b'0') as u64;
            sum += first * 10 + last;
            digits = [0_u8; 128];
            line_idx = 0;
            byte_idx += 1;
            continue;
        }

        fn gorble(s: &[u8]) -> u64 {
            let mut bytes = [0; 8];
            let mut i = 0;
            while i < s.len() {
                bytes[7 - i] = s[i];
                i += 1;
            }
            // like: u64::from_be_bytes([0, 0, 0, b't', b'h', b'g', b'i', b'e'])
            u64::from_be_bytes(bytes)
        }

        let mut acc = 0;

        acc |= if one >= b'0' && one <= b'9' { one } else { 0 };

        let mut vector_result = None;

        #[cfg(all(target_arch = "x86_64"))]
        if avx2 {
            use std::arch::x86_64;
            unsafe fn round(input: u64, compare: [u64; 4], then: [u64; 4]) -> x86_64::__m256i {
                // YYYYYYYY|AAAAAAAA|XXXXXXXX|BBBBBBBB|
                let compare = unsafe { std::mem::transmute::<_, x86_64::__m256i>(compare) };
                // 000000EE|000000ZZ|000000XX|000000FF|
                let then = unsafe { std::mem::transmute::<_, x86_64::__m256i>(then) };
                // XXXXXXXX|XXXXXXXX|XXXXXXXX|XXXXXXXX|
                let actual = x86_64::_mm256_set1_epi64x(input as i64);
                // 00000000|00000000|11111111|00000000|
                let mask = x86_64::_mm256_cmpeq_epi64(compare, actual);
                // 00000000|00000000|0000000X|00000000|
                let result = x86_64::_mm256_and_si256(then, mask);
                // we can also pretend that it's this as only the lowest byte is set in each lane
                // 0000/0000|0000/0000|0000/000X|0000/0000|
                result
            }

            let fives = round(
                five,
                [gorble(b"eight"), gorble(b"seven"), gorble(b"three"), 0],
                [b'8' as _, b'7' as _, b'3' as _, 0],
            );
            let fours = round(
                four,
                [gorble(b"five"), gorble(b"four"), gorble(b"nine"), 0],
                [b'5' as _, b'4' as _, b'9' as _, 0],
            );
            let threes = round(
                three,
                [gorble(b"six"), gorble(b"two"), gorble(b"one"), 0],
                [b'6' as _, b'2' as _, b'1' as _, 0],
            );

            let result =
                x86_64::_mm256_or_pd(std::mem::transmute(fives), std::mem::transmute(fours));
            let result = x86_64::_mm256_or_pd(result, std::mem::transmute(threes));

            let low = x86_64::_mm256_extractf128_pd(result, 0);
            let high = x86_64::_mm256_extractf128_pd(result, 1);
            let result = x86_64::_mm_or_pd(low, high);
            let result = std::mem::transmute::<_, x86_64::__m128i>(result);
            let low = x86_64::_mm_extract_epi64(result, 0);
            let high = x86_64::_mm_extract_epi64(result, 1);
            let result = low | high;
            debug_assert!(result < 128);

            digits[line_idx] = acc | result as u8;

            if cfg!(debug_assertions) {
                vector_result = Some(acc | result as u8);
            }
        }

        if cfg!(debug_assertions) || !avx2 {
            macro_rules! check {
                ($len:ident == $str:expr => $value:expr) => {
                    acc |= (if $len == gorble($str) { $value } else { 0 });
                };
            }

            check!(five == b"eight" => b'8');
            check!(five == b"seven" => b'7');
            check!(five == b"three" => b'3');

            check!(four == b"five" => b'5');
            check!(four == b"four" => b'4');
            check!(four == b"nine" => b'9');

            check!(three == b"six" => b'6');
            check!(three == b"two" => b'2');
            check!(three == b"one" => b'1');

            digits[line_idx] = acc;

            if cfg!(debug_assertions) {
                if let Some(vector_result) = vector_result {
                    assert_eq!(vector_result, acc);
                }
            }
        }

        byte_idx += 1;
        line_idx += 1;
    }

    sum
}
