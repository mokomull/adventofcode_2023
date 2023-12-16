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
        let mut new_round = self.round.clone();

        'outer: loop {
            for &(x, y) in new_round.iter() {
                if x > 0 && !new_round.contains(&(x - 1, y)) && !self.cube.contains(&(x, y)) {
                    // nothing above this rock, so move it up and try again
                    new_round.remove(&(x, y));
                    new_round.insert((x - 1, y));
                    continue 'outer;
                }
            }

            // nothing moved, so we're done
            break;
        }

        Ok(new_round
            .into_iter()
            .map(|(x, _y)| self.max_x as u64 - x as u64 + 1)
            .sum())
    }

    fn part2(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented")
    }
}
