use helper::{Day, Variants};

pub fn main() {
    helper::main::<Day08>(include_str!("../input.txt"));
}

struct Day08;

helper::define_variants! {
    day => crate::Day08;
    part1 {
        basic => crate::part1;
        asciicheck => crate::part1_asciicheck;
        parsing => crate::part1_parsing;
        vec_opts => crate::part1_vec_opts;
        arrayvec => crate::part1_arrayvec;
    }
    part2 {
        basic => crate::part2;
    }
}

impl Day for Day08 {
    fn part1() -> Variants {
        part1_variants!(construct_variants)
    }

    fn part2() -> Variants {
        part2_variants!(construct_variants)
    }
}

fn part1(input: &str) -> u64 {
    let mut all_antennas = vec![vec![]; 128];

    let mut height = 0;
    let mut width = 0;

    for (row, line) in input.lines().enumerate() {
        for (col, cell) in line.bytes().enumerate() {
            if cell.is_ascii_alphanumeric() {
                all_antennas[cell as usize].push((col, row));
            }
        }
        width = line.len();
        height += 1;
    }

    let mut is_antinode = vec![false; width * height];

    for antennas in all_antennas {
        for &a in &antennas {
            for &b in &antennas {
                if a == b {
                    continue;
                }

                let diff = (a.0 as i64 - b.0 as i64, a.1 as i64 - b.1 as i64);
                let antinode = (a.0 as i64 + diff.0, a.1 as i64 + diff.1);

                if !(0..(width as i64)).contains(&antinode.0) {
                    continue;
                }
                if !(0..(height as i64)).contains(&antinode.1) {
                    continue;
                }

                is_antinode[(antinode.0 as usize * width) + antinode.1 as usize] = true;
            }
        }
    }

    is_antinode
        .iter()
        .filter(|is_antinode| **is_antinode)
        .count() as u64
}

fn part1_asciicheck(input: &str) -> u64 {
    let mut all_antennas = vec![vec![]; 128];

    let mut height = 0;
    let mut width = 0;

    for (row, line) in input.lines().enumerate() {
        for (col, cell) in line.bytes().enumerate() {
            if cell != b'.' {
                all_antennas[cell as usize].push((col, row));
            }
        }
        width = line.len();
        height += 1;
    }

    let mut is_antinode = vec![false; width * height];

    for antennas in all_antennas {
        for &a in &antennas {
            for &b in &antennas {
                if a == b {
                    continue;
                }

                let diff = (a.0 as i64 - b.0 as i64, a.1 as i64 - b.1 as i64);
                let antinode = (a.0 as i64 + diff.0, a.1 as i64 + diff.1);

                if !(0..(width as i64)).contains(&antinode.0) {
                    continue;
                }
                if !(0..(height as i64)).contains(&antinode.1) {
                    continue;
                }

                is_antinode[(antinode.0 as usize * width) + antinode.1 as usize] = true;
            }
        }
    }

    is_antinode
        .iter()
        .filter(|is_antinode| **is_antinode)
        .count() as u64
}

fn part1_parsing(input: &str) -> u64 {
    let mut all_antennas = vec![vec![]; 128];

    let mut width = 0;

    let mut row = 0;
    let mut col = 0;
    for cell in input.bytes() {
        if cell == b'\n' {
            row += 1;
            width = col;
            col = 0;
        } else if cell != b'.' {
            all_antennas[cell as usize].push((col, row));
            col += 1;
        } else {
            col += 1;
        }
    }

    let height = row;

    let mut is_antinode = vec![false; width * height];

    for antennas in all_antennas {
        for &a in &antennas {
            for &b in &antennas {
                if a == b {
                    continue;
                }

                let diff = (a.0 as i64 - b.0 as i64, a.1 as i64 - b.1 as i64);
                let antinode = (a.0 as i64 + diff.0, a.1 as i64 + diff.1);

                if !(0..(width as i64)).contains(&antinode.0) {
                    continue;
                }
                if !(0..(height as i64)).contains(&antinode.1) {
                    continue;
                }

                is_antinode[(antinode.0 as usize * width) + antinode.1 as usize] = true;
            }
        }
    }

    is_antinode
        .iter()
        .filter(|is_antinode| **is_antinode)
        .count() as u64
}

fn part1_vec_opts(input: &str) -> u64 {
    let mut all_antennas = [const { vec![] }; 256];

    let mut width = 0;

    let mut row = 0;
    let mut col = 0;
    for cell in input.bytes() {
        if cell == b'\n' {
            row += 1;
            width = col;
            col = 0;
        } else if cell != b'.' {
            all_antennas[cell as usize].push((col, row));
            col += 1;
        } else {
            col += 1;
        }
    }

    let height = row;

    let mut is_antinode = vec![false; width * height];
    let all_antennas = &all_antennas[b'0' as usize..b'z' as usize];

    for antennas in all_antennas {
        for &a in antennas {
            for &b in antennas {
                if a == b {
                    continue;
                }

                let diff = (a.0 as i64 - b.0 as i64, a.1 as i64 - b.1 as i64);
                let antinode = (a.0 as i64 + diff.0, a.1 as i64 + diff.1);

                if !(0..(width as i64)).contains(&antinode.0) {
                    continue;
                }
                if !(0..(height as i64)).contains(&antinode.1) {
                    continue;
                }

                unsafe {
                    *is_antinode
                        .get_unchecked_mut((antinode.0 as usize * width) + antinode.1 as usize) =
                        true;
                }
            }
        }
    }

    is_antinode
        .iter()
        .filter(|is_antinode| **is_antinode)
        .count() as u64
}

fn part1_arrayvec(input: &str) -> u64 {
    type Coods = (usize, usize);
    struct Arrayvec {
        elems: [Coods; 8],
        len: usize,
    }
    impl Arrayvec {
        fn push(&mut self, elem: Coods) {
            self.elems[self.len] = elem;
            self.len += 1;
        }
    }
    impl<'a> IntoIterator for &'a Arrayvec {
        type IntoIter = std::slice::Iter<'a,Coods>;
        type Item = &'a Coods;
        fn into_iter(self) -> Self::IntoIter {
            unsafe { self.elems.get_unchecked(0..self.len).iter() }
        }
    }

    let mut all_antennas = [const {
        Arrayvec {
            elems: [(0, 0); 8],
            len: 0,
        }
    }; 256];

    let mut width = 0;

    let mut row = 0;
    let mut col = 0;
    for cell in input.bytes() {
        if cell == b'\n' {
            row += 1;
            width = col;
            col = 0;
        } else if cell != b'.' {
            all_antennas[cell as usize].push((col, row));
            col += 1;
        } else {
            col += 1;
        }
    }

    let height = row;

    let mut is_antinode = vec![false; width * height];
    let all_antennas = &all_antennas[b'0' as usize..b'z' as usize];

    for antennas in all_antennas {
        for &a in antennas {
            for &b in antennas {
                if a == b {
                    continue;
                }

                let diff = (a.0 as i64 - b.0 as i64, a.1 as i64 - b.1 as i64);
                let antinode = (a.0 as i64 + diff.0, a.1 as i64 + diff.1);

                if !(0..(width as i64)).contains(&antinode.0) {
                    continue;
                }
                if !(0..(height as i64)).contains(&antinode.1) {
                    continue;
                }

                unsafe {
                    *is_antinode
                        .get_unchecked_mut((antinode.0 as usize * width) + antinode.1 as usize) =
                        true;
                }
            }
        }
    }

    is_antinode
        .iter()
        .filter(|is_antinode| **is_antinode)
        .count() as u64
}

fn part2(_input: &str) -> u64 {
    0
}

helper::tests! {
    day08 Day08;
    part1 {
        small => 14;
        default => 269;
    }
    part2 {
        small => 34;
        default => 0;
    }
}
helper::benchmarks! {}
