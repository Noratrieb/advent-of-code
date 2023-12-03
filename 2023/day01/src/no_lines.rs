pub unsafe fn part2(input: &str) -> u64 {
    let mut sum = 0;

    let bytes = input.as_bytes();

    let mut digits = [0_u8; 128];

    let mut byte_idx = 0;
    let mut line_idx = 0;

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

        let one = (block & ((1 << 8) - 1)) as u8;
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

        const fn gorble(s: &[u8]) -> u64 {
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

        macro_rules! check {
            ($const:ident $len:ident == $str:expr => $value:expr) => {
                const $const: u64 = gorble($str);
                acc |= (if $len == $const { $value } else { 0 });
            };
        }

        acc |= if one.is_ascii_digit() { one } else { 0 };

        check!(EIGHT five == b"eight" => b'8');
        check!(SEVEN five == b"seven" => b'7');
        check!(THREE five == b"three" => b'3');

        check!(FIVE four == b"five" => b'5');
        check!(FOUR four == b"four" => b'4');
        check!(NINE four == b"nine" => b'9');

        check!(SIX three == b"six" => b'6');
        check!(TWO three == b"two" => b'2');
        check!(ONE three == b"one" => b'1');

        digits[line_idx] = acc;

        byte_idx += 1;
        line_idx += 1;
    }

    sum
}
