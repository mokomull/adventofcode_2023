use std::cmp::min;

pub use prelude_2022::*;

pub extern crate anyhow;
pub use anyhow::Context;

pub trait Day {
    fn new(input: &str) -> Self;
    fn part1(&self) -> anyhow::Result<u64>;
    fn part2(&self) -> anyhow::Result<u64>;
}

/// Iterate through the (up to) eight locations that are adjacent to (x, y),
/// where `data` is accessed via `data[x][y]`.
///
/// # Panics
///
/// Panics if `data` is empty or non-square.
pub fn adjacent_including_diagonal<T, C>(
    data: &[C],
    x: usize,
    y: usize,
) -> impl Iterator<Item = (usize, usize)>
where
    C: AsRef<[T]>,
{
    let first_index = x.saturating_sub(1)..=min(x + 1, data.len() - 1);
    // just use data[0], which assumes that the input is
    // (a) non-empty, and
    // (b) square.
    let second_index = y.saturating_sub(1)..=min(y + 1, data[0].as_ref().len() - 1);

    first_index
        .cartesian_product(second_index)
        .filter(move |&(i, j)| (x, y) != (i, j))
}
