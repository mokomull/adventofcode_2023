use std::cmp::min;

use prelude::*;

#[derive(Debug, PartialEq)]
enum Tile {
    Ash,
    Rock,
}

use Tile::*;

struct Pattern(Vec<Vec<Tile>>);

impl Pattern {
    fn part1(&self) -> u64 {
        self.reflection_differences(0)
    }

    fn reflection_differences(&self, needle: usize) -> u64 {
        let mut vertical_reflections = vec![];
        let mut horizontal_reflections = vec![];

        'outer: for i in 1..self.0[0].len() {
            let mut differences = 0;
            for row in &self.0 {
                let count = min(i, row.len() - i);
                assert_ne!(0, count, "we can't process a reflection of nothing");
                let left = row[(i - count)..i].iter();
                let right = row[i..(i + count)].iter().rev();
                for (a, b) in left.zip(right) {
                    if a != b {
                        // found a mismatch
                        differences += 1;
                        if differences > needle {
                            continue 'outer;
                        }
                    }
                }
            }

            if differences == needle {
                vertical_reflections.push(i);
            }
        }

        'outer: for i in 1..self.0.len() {
            let mut differences = 0;
            let count = min(i, self.0.len() - i);
            assert_ne!(0, count, "we can't process a reflection of nothing");
            let top = self.0[(i - count)..i].iter();
            let bottom = self.0[i..(i + count)].iter().rev();
            for (a, b) in top.zip(bottom) {
                let this_differences = a.iter().zip(b).filter(|(x, y)| x != y).count();
                differences += this_differences;
                if differences > needle {
                    continue 'outer;
                }
            }

            if differences == needle {
                horizontal_reflections.push(i);
            }
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
        Ok(self
            .0
            .iter()
            .enumerate()
            .map(|(i, pattern)| {
                let res = pattern.part1();
                log::debug!(
                    "pattern {i} -> {res}\n{}",
                    pattern
                        .0
                        .iter()
                        .map(|row| {
                            row.iter()
                                .map(|tile| match tile {
                                    Ash => '.',
                                    Rock => '#',
                                })
                                .join("")
                        })
                        .join("\n")
                );
                res
            })
            .sum())
    }

    fn part2(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn manual_bad_case() {
        let solution = Solution::new(
            ".#...#..#
##...#...
....#..#.
....#..#.
##...#...
.#..##..#
####...##
##.#..#.#
#.......#
.#.....#.
..#.##.##
.#.#.....
..###.#.#
#..###.##
.##....##
..#######
..#######",
        );

        assert_eq!(1600, solution.part1().unwrap());
    }
}
