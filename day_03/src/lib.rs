use std::cmp::min;

use prelude::*;

pub struct Solution(Vec<Vec<u8>>);

impl Solution {
    pub fn new(input: &str) -> Solution {
        init();
        Solution(input.lines().map(|line| line.as_bytes().into()).collect())
    }

    pub fn part1(&self) -> anyhow::Result<u64> {
        let mut visited = HashSet::new();
        let mut sum = 0;

        for row in 0..self.0.len() {
            // assume all rows are the same length
            for col in 0..self.0[0].len() {
                let c = self.0[row][col];
                if c.is_ascii_digit() || c == b'.' {
                    continue;
                }

                log::debug!("symbol at {:?}: {:?}", (row, col), c);

                // we're on a symbol, so check all around it for a digit
                for x in row.saturating_sub(1)..=min(row + 1, self.0.len() - 1) {
                    for y in col.saturating_sub(1)..=min(col + 1, self.0[0].len() - 1) {
                        if (x, y) == (row, col) || visited.contains(&(x, y)) {
                            continue;
                        }

                        if self.0[x][y].is_ascii_digit() {
                            // part numbers only exist within a single row, so just look left and right for more digits
                            let string = &*self.0[x];
                            let first = (0..y)
                                .rev()
                                .find(|&i| !string[i].is_ascii_digit())
                                .map(
                                    // add one, because we found the first position left of (x, y) that is *not* a digit
                                    |i| i + 1,
                                )
                                .unwrap_or(0);
                            let last = (y..string.len())
                                .find(|&i| !string[i].is_ascii_digit())
                                .unwrap_or(string.len());
                            log::debug!("{first} {last}");

                            let part_number = std::str::from_utf8(&string[first..last])
                                .context("digits should be UTF-8")?;
                            log::debug!("found part number: {:?}", part_number);

                            sum += part_number
                                .parse::<u64>()
                                .context("digits should also parse into a u64")?;

                            // and mark that part number as "used" so we don't double-count it
                            for i in first..last {
                                visited.insert((x, i));
                            }
                        }
                    }
                }
            }
        }

        Ok(sum)
    }

    pub fn part2(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented")
    }
}
