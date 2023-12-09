use prelude::*;

pub struct Solution(Vec<Vec<u64>>);

impl Day for Solution {
    fn new(input: &str) -> Self {
        Solution(
            input.lines().map(|line| {
                line.split_whitespace().map(|num| num.parse().expect("can't parse input number")).collect()
            }).collect()
        )
    }

    fn part1(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented")
    }

    fn part2(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented")
    }
}
