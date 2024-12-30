mod cmd;
mod ext;
mod hash;

use std::fmt::Display;
use std::{borrow::Cow, fmt::Debug};

pub use self::cmd::main;
pub use self::ext::*;
pub use self::hash::*;

#[derive(PartialEq, Eq)]
pub enum Answer {
    U64(u64),
    String(String),
}
impl Debug for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U64(arg0) => Debug::fmt(arg0, f),
            Self::String(arg0) => Debug::fmt(arg0, f),
        }
    }
}
impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U64(arg0) => Display::fmt(arg0, f),
            Self::String(arg0) => Display::fmt(arg0, f),
        }
    }
}
impl From<u64> for Answer {
    fn from(value: u64) -> Self {
        Answer::U64(value)
    }
}
impl From<String> for Answer {
    fn from(value: String) -> Self {
        Answer::String(value)
    }
}

pub type Solution = fn(&str) -> Answer;

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

#[track_caller]
pub fn test_part1<D: Day>(inputs: &[(&str, &str, Answer)]) {
    for variant in D::part1().variants {
        for input in inputs {
            let (path, input, expected) = input;
            let actual = (variant.f)(input);
            if actual != *expected {
                panic!(
                    "failed: {}: {}: {} != {}",
                    path, variant.name, actual, expected
                );
            }
        }
    }
}

#[track_caller]
pub fn test_part2<D: Day>(inputs: &[(&str, &str, Answer)]) {
    for variant in D::part2().variants {
        for input in inputs {
            let (path, input, expected) = input;
            let actual = (variant.f)(input);
            if actual != *expected {
                panic!(
                    "failed: {}: {}: {} != {}",
                    path, variant.name, actual, expected
                );
            }
        }
    }
}

#[macro_export]
macro_rules! only_x86_64_and {
    ($feature:tt => $input:ident, $fast:ident else $fallback:ident) => {
        #[cfg(not(target_arch = "x86_64"))]
        return $fallback($input);
        #[cfg(target_arch = "x86_64")]
        {
            if !std::arch::is_x86_feature_detected!($feature) {
                return $fallback($input);
            }
            return unsafe { $fast($input) };
        }
    };
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
                $crate::Variant::new(stringify!($name), |input| {
                    let answer = $func(input);
                    $crate::Answer::from(answer)
                })
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
        #[::divan::bench(sample_count = 10000)]
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

#[doc(hidden)]
#[macro_export]
macro_rules! _test_name_to_file {
    (small) => {
        "../input_small.txt"
    };
    (default) => {
        "../input.txt"
    };
    ($path:expr) => {
        $path
    };
}

#[macro_export]
macro_rules! tests {
    (
        $day_small:ident $day:ident;
        part1 {
            $(
                $file1:tt => $p1:expr;
            )*
        }
        part2 {
            $(
                $file2:tt => $p2:expr;
            )*
        }
    ) => {
        #[cfg(test)]
        mod $day_small {
            #[test]
            fn part1() {
                helper::test_part1::<super::$day>(&[
                    $(
                        ($crate::_test_name_to_file!($file1), include_str!($crate::_test_name_to_file!($file1)), $crate::Answer::from($p1)),
                    )*
                ]);
            }

            #[test]
            fn part2() {
                helper::test_part2::<super::$day>(&[
                    $(
                        ($crate::_test_name_to_file!($file2), include_str!($crate::_test_name_to_file!($file2)), $crate::Answer::from($p2)),
                    )*
                ]);
            }
        }
    };
}
