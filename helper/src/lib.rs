mod cmd;
mod ext;

use std::{borrow::Cow, fmt::Debug};

pub use self::cmd::main;
pub use self::ext::*;

pub type Solution = fn(&str) -> u64;

pub trait Day {
    fn part1() -> Variants;

    fn part2() -> Variants;

    /// Pad or manipulate the input in ways that don't necessarily
    /// change it but do things that may be sound or unsound.
    fn pad_input(input: &str) -> Cow<str> {
        Cow::Borrowed(input)
    }
}

pub struct Variants {
    pub variants: Vec<Variant>,
}

#[derive(Debug)]
pub struct Variant {
    pub name: &'static str,
    pub f: Solution,
}

impl Variants {
    pub fn basic(f: Solution) -> Self {
        Variants {
            variants: vec![Variant { name: "basic", f }],
        }
    }
}

impl Variant {
    pub fn new(name: &'static str, f: Solution) -> Self {
        Self { name, f }
    }
}

pub fn test_part1<D: Day>(inputs: &[(&str, u64)]) {
    for variant in D::part1().variants {
        for input in inputs {
            let actual = (variant.f)(input.0);
            if actual != input.1 {
                panic!("failed: {}: {actual} != {}", variant.name, input.1);
            }
        }
    }
}

pub fn test_part2<D: Day>(inputs: &[(&str, u64)]) {
    for variant in D::part2().variants {
        for input in inputs {
            let actual = (variant.f)(input.0);
            if actual != input.1 {
                panic!("failed: {}: {actual} != {}", variant.name, input.1);
            }
        }
    }
}

#[macro_export]
macro_rules! define_variants {
    (
        day => $day:ty;
        part1 {
            $( $name1:ident => $func1:expr $(, sample_count=$sample_count1:expr)? ; )*
        }
        part2 {
            $( $name2:ident => $func2:expr $(, sample_count=$sample_count2:expr)? ; )*
        }
    ) => {
        macro_rules! part1_variants {
            ($macro:ident) => {
                $crate::$macro! { $day; $( ($name1, $func1, [ $( sample_count=$sample_count1, )? ]) ),* }
            };
        }
        macro_rules! part2_variants {
            ($macro:ident) => {
                $crate::$macro! { $day; $( ($name2, $func2, [ $( sample_count=$sample_count2, )? ]) ),* }
            };
        }
    };
}

#[macro_export]
macro_rules! construct_variants {
    ( $day:ty; $( ($name:ident, $func:expr, [ $($_:tt)* ]) ),*) => {
        $crate::Variants {
            variants: vec![$(
                $crate::Variant::new(stringify!($name), $func)
            ),*]
        }
    };
}

#[macro_export]
macro_rules! benchmarks {
    () => {
        mod bench {
            mod part1 {
                part1_variants! { _define_benchmarks }
            }
            mod part2 {
                part2_variants! { _define_benchmarks }
            }
        }

        pub fn bench() {
            divan::main();
        }
    };
}

#[macro_export]
macro_rules! _bench_sample_count {
    (;$($tt:tt)*) => {
        #[::divan::bench(sample_count = 5000)]
        $($tt)*
    };
    ($sample_count:expr; $($tt:tt)*) => {
        #[::divan::bench(sample_count = $sample_count)]
        $($tt)*
    };
}

#[macro_export]
macro_rules! _define_benchmarks {
    ($day:ty; $( ($name:ident, $func:expr, [ $(sample_count=$sample_count:expr,)? ]) ),*) => {
        $(
            $crate::_bench_sample_count! {
                $($sample_count)?;
                fn $name(bencher: ::divan::Bencher) {
                    let input = include_str!("../input.txt");
                    let input = <$day as $crate::Day>::pad_input(input);

                    bencher.with_inputs(|| input.as_ref()).bench_values($func);
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! tests {
    (
        $day_small:ident $day:ident;
        part1 {
            small => $p1small:expr;
            default => $p1default:expr;
        }
        part2 {
            small => $p2small:expr;
            default => $p2default:expr;
        }
    ) => {
        $crate::tests! {
            $day_small $day;
            part1 {
                "../input_small.txt" => $p1small;
                "../input.txt" => $p1default;
            }
            part2 {
                "../input_small.txt" => $p2small;
                "../input.txt" => $p2default;
            }
        }
    };
    (
        $day_small:ident $day:ident;
        part1 {
            $(
                $file1:literal => $p1:expr;
            )*
        }
        part2 {
            $(
                $file2:literal => $p2:expr;
            )*
        }
    ) => {
        #[cfg(test)]
        mod $day_small {
            #[test]
            fn part1() {
                helper::test_part1::<super::$day>(&[
                    $(
                        (include_str!($file1), $p1),
                    )*
                ]);
            }

            #[test]
            fn part2() {
                helper::test_part2::<super::$day>(&[
                    $(
                        (include_str!($file2), $p2),
                    )*
                ]);
            }
        }
    };
}
