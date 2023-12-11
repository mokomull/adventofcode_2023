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
        let expanded = self.0.iter().flat_map(|row| {
            if row.iter().any(|&x| x) {
                vec![row]
            } else {
                vec![row, row]
            }
        });

        let mut galaxies = vec![];
        let mut sum_distances = 0;

        for (i, row) in expanded.enumerate() {
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
