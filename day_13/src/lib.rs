use prelude::*;

#[derive(PartialEq)]
enum Tile {
    Ash,
    Rock,
}

use Tile::*;

struct Pattern(Vec<Vec<Tile>>);

impl Pattern {
    fn part1(&self) -> u64 {
        let mut vertical_reflections = vec![];
        let mut horizontal_reflections = vec![];

        'outer: for i in 1..(self.0[0].len() - 1) {
            for row in &self.0 {
                let left = row[..i].iter();
                let right = row[i..].iter().rev();
                for (a, b) in left.zip(right) {
                    if a != b {
                        // found a mismatch, so this ain't the mirror
                        continue 'outer;
                    }
                }
            }

            vertical_reflections.push(i);
        }

        'outer: for i in 1..(self.0.len() - 1) {
            let top = self.0[..i].iter();
            let bottom = self.0[i..].iter().rev();
            for (a, b) in top.zip(bottom) {
                if a != b {
                    continue 'outer;
                }
            }

            horizontal_reflections.push(i);
        }

        vertical_reflections.into_iter().sum::<usize>() as u64
            + (horizontal_reflections.into_iter().sum::<usize>() as u64 * 100)
    }
}

pub struct Solution(Vec<Pattern>);

impl Day for Solution {
    fn new(input: &str) -> Self {
        Solution(
            input
                .split("\n\n")
                .map(|pattern| {
                    Pattern(
                        pattern
                            .lines()
                            .map(|line| {
                                line.bytes()
                                    .map(|b| match b {
                                        b'#' => Rock,
                                        b'.' => Ash,
                                        _ => panic!("unexpected character {b:?}"),
                                    })
                                    .collect_vec()
                            })
                            .collect_vec(),
                    )
                })
                .collect_vec(),
        )
    }

    fn part1(&self) -> anyhow::Result<u64> {
        Ok(self.0.iter().map(Pattern::part1).sum())
    }

    fn part2(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented");
    }
}
