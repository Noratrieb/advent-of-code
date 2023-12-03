pub fn part2(input: &str) -> u64 {
    let sum = input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();

            let mut i = 0;
            let mut first = None;
            let mut last = b'_';

            let mut insert = |byte| {
                if first.is_none() {
                    first = Some(byte);
                }
                last = byte;
            };

            while i < bytes.len() {
                match bytes[i] {
                    b @ b'0'..=b'9' => insert(b),
                    b'o' if bytes.get(i..(i + 3)) == Some(b"one") => insert(b'1'),
                    b't' if bytes.get(i..(i + 3)) == Some(b"two") => insert(b'2'),
                    b't' if bytes.get(i..(i + 5)) == Some(b"three") => insert(b'3'),
                    b'f' if bytes.get(i..(i + 4)) == Some(b"four") => insert(b'4'),
                    b'f' if bytes.get(i..(i + 4)) == Some(b"five") => insert(b'5'),
                    b's' if bytes.get(i..(i + 3)) == Some(b"six") => insert(b'6'),
                    b's' if bytes.get(i..(i + 5)) == Some(b"seven") => insert(b'7'),
                    b'e' if bytes.get(i..(i + 5)) == Some(b"eight") => insert(b'8'),
                    b'n' if bytes.get(i..(i + 4)) == Some(b"nine") => insert(b'9'),
                    _ => {}
                }
                i += 1;
            }

            let first = (first.unwrap() - b'0') as u64;
            let last = (last - b'0') as u64;

            first * 10 + last
        })
        .sum::<u64>();

        sum
    }
