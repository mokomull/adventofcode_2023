use std::cmp::{max, min};

use prelude::*;

pub struct Solution(Vec<Vec<bool>>);

impl Day for Solution {
    fn new(input: &str) -> Self {
        Solution(
            input
                .lines()
                .map(|l| {
                    l.bytes()
                        .map(|b| match b {
                            b'#' => true,
                            _ => false,
                        })
                        .collect_vec()
                })
                .collect_vec(),
        )
    }

    fn part1(&self) -> anyhow::Result<u64> {
        let mut expanded = self.0.clone();
        // find all the empty columns first - from right to left so we never have to worry about what we've already inserted
        for c in (0..self.0[0].len()).rev() {
            if self.0.iter().all(|row| !row[c]) {
                // insert a column here
                for row in expanded.iter_mut() {
                    row.insert(c, false);
                }
            }
        }

        // now find all the empty rows
        for r in (0..self.0.len()).rev() {
            if self.0[r].iter().all(|&x| !x) {
                expanded.insert(r, expanded[r].clone());
            }
        }

        let mut galaxies = vec![];
        let mut sum_distances = 0;

        for (i, row) in expanded.into_iter().enumerate() {
            for j in row.iter().positions(|&x| x) {
                for &(other_i, other_j) in &galaxies {
                    // Manhattan distance, but written so I don't have to use signed arithmetic and abs().
                    let length =
                        max(other_i, i) - min(other_i, i) + max(other_j, j) - min(other_j, j);
                    log::debug!("found length {length}");
                    sum_distances += length;
                }
                galaxies.push((i, j));
            }
        }

        Ok(sum_distances as u64)
    }

    fn part2(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented")
    }
}
