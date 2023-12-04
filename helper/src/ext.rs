use std::mem::MaybeUninit;

use nom::{character::complete::digit1, combinator::map, IResult};

pub fn integer(input: &str) -> IResult<&str, u64> {
    map(digit1, |d: &str| d.parse::<u64>().unwrap())(input)
}

#[derive(Debug)]
pub struct CollectArrayError;

/// i will not use itertools i will not use itertools i will not use itertools i will not use itertools
pub trait IteratorExt: Iterator {
    /// Collect an iterator into an array.
    /// If `next` panics, collected items are leaked. Too bad!
    fn collect_array<const N: usize>(&mut self) -> Result<[Self::Item; N], CollectArrayError> {
        // SAFETY: Uninit is valid for MaybeUninit
        let mut array: [MaybeUninit<Self::Item>; N] =
            unsafe { MaybeUninit::uninit().assume_init() };

        for i in 0..array.len() {
            array[i].write(self.next().ok_or(CollectArrayError)?);
        }

        if self.next().is_some() {
            return Err(CollectArrayError);
        }

        // SAFETY: All elements have been initialized
        Ok(array.map(|elem| unsafe { elem.assume_init() }))
    }

    /// Collect an iterator into an array.
    /// If `next` panics, collected items are leaked. Too bad!
    fn collect_array_default<const N: usize>(
        &mut self,
    ) -> Result<[Self::Item; N], CollectArrayError>
    where
        Self::Item: Default + Copy,
    {
        // SAFETY: Uninit is valid for MaybeUninit
        let mut array: [Self::Item; N] = [Default::default(); N];

        for i in 0..array.len() {
            let Some(elem) = self.next() else {
                break;
            };
            array[i] = elem;
        }

        if self.next().is_some() {
            return Err(CollectArrayError);
        }

        // SAFETY: All elements have been initialized
        Ok(array)
    }
}

impl<I: Iterator> IteratorExt for I {}

#[cfg(test)]
mod tests {
    use crate::IteratorExt;

    #[test]
    fn collect_array() {
        assert!([0, 1].into_iter().collect_array::<3>().is_err());
        assert!([0, 1].into_iter().collect_array::<1>().is_err());
        assert_eq!([0, 1].into_iter().collect_array().unwrap(), [0, 1]);
    }
}
