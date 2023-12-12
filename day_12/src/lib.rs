use prelude::*;

enum Spring {
    Good,
    Damaged,
    Unknown,
}

use Spring::*;

impl Spring {
    fn parse(input: &str) -> Vec<Spring> {
        input
            .bytes()
            .map(|b| match b {
                b'.' => Good,
                b'#' => Damaged,
                b'?' => Unknown,
                x => panic!("unexpected spring {x:?}"),
            })
            .collect_vec()
    }
}

pub struct Solution(Vec<(Vec<Spring>, Vec<u64>)>);

impl Day for Solution {
    fn new(input: &str) -> Self {
        Solution(
            input
                .lines()
                .map(|line| {
                    let (springs, counts) = line.split_once(" ").expect("missing space");
                    let counts = counts
                        .split(',')
                        .map(|x| x.parse().expect("not an integer"))
                        .collect_vec();
                    (Spring::parse(springs), counts)
                })
                .collect(),
        )
    }

    fn part1(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented")
    }

    fn part2(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented")
    }
}
