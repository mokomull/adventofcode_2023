use prelude::*;

pub struct Solution(Vec<Vec<i64>>);

fn next_value(values: &[i64]) -> i64 {
    let mut diffs = vec![values.to_vec()];

    log::debug!("Calculating interpolated value for {values:?}");

    while diffs.last().unwrap().iter().any(|&i| i != 0) {
        let next_diff = diffs
            .last()
            .unwrap()
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect_vec();
        diffs.push(next_diff);
    }

    log::debug!("found diffs {diffs:?}");

    let res = diffs.iter().map(|d| d.last().unwrap()).sum::<i64>();
    log::debug!("returning {res:?}");
    res
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        Solution(
            input
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|num| num.parse().expect("can't parse input number"))
                        .collect()
                })
                .collect(),
        )
    }

    fn part1(&self) -> anyhow::Result<u64> {
        Ok(self
            .0
            .iter()
            .map(|history| next_value(&history))
            .sum::<i64>() as u64)
    }

    fn part2(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented")
    }
}
