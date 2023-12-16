use std::cmp::max;

use prelude::*;

pub struct Solution {
    round: HashSet<(usize, usize)>,
    cube: HashSet<(usize, usize)>,
    max_x: usize,
    max_y: usize,
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        let mut round = HashSet::new();
        let mut cube = HashSet::new();
        let mut max_x = 0;
        let mut max_y = 0;

        for (x, line) in input.lines().enumerate() {
            for (y, c) in line.bytes().enumerate() {
                if c == b'O' {
                    round.insert((x, y));
                } else if c == b'#' {
                    cube.insert((x, y));
                }
                max_y = max(max_y, y);
            }
            max_x = max(max_x, x);
        }

        Solution {
            round,
            cube,
            max_x,
            max_y,
        }
    }

    fn part1(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented")
    }

    fn part2(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented")
    }
}
