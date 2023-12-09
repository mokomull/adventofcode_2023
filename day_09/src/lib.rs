use prelude::*;

pub struct Solution(Vec<Vec<u64>>);

fn next_value(values: &[u64]) -> u64 {
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

    for i in (1..diffs.len()).rev() {
        let (lower, upper) = diffs.split_at_mut(i);
        let lower: &mut Vec<u64> = lower.last_mut().unwrap();
        let upper = &mut upper[0];
        upper.push(upper.last().unwrap() + lower.last().unwrap())
    }

let res =     *diffs[0].last().unwrap();
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
        Ok(self.0.iter().map(|history| next_value(&history)).sum())
    }

    fn part2(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented")
    }
}
