use std::collections::VecDeque;

use prelude::*;

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    NS,
    EW,
    NeSw,
    NwSe,
}

use Tile::*;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

use Direction::*;

impl Direction {
    fn go(&self, board: &[Vec<Tile>], x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let cardinal = adjacent_including_diagonal(board, x, y)
            .filter(|&(next_x, next_y)| next_x == x || next_y == y);

        cardinal
            .filter(|&(next_x, next_y)| match self {
                Up => next_x < x,
                Down => next_x > x,
                Left => next_y < y,
                Right => next_y > y,
            })
            .collect_vec()
            .into_iter()
    }
}

pub struct Solution(Vec<Vec<Tile>>);

impl Solution {
    fn count_from(&self, d: Direction, pos: (usize, usize)) -> u64 {
        let mut seen = HashSet::new();
        let mut to_visit = VecDeque::from([(d, pos)]);

        while let Some((d, (x, y))) = to_visit.pop_front() {
            if !seen.insert((d, (x, y))) {
                // we've already dealt with a beam in this direction at this spot.
                continue;
            }

            let tile = self.0[x][y];
            match (d, tile) {
                (d, Empty) | (d @ Left, EW) | (d @ Right, EW) | (d @ Up, NS) | (d @ Down, NS) => {
                    // proceed straight through
                    for (x, y) in d.go(&self.0, x, y) {
                        to_visit.push_back((d, (x, y)));
                    }
                }

                (Up, EW) | (Down, EW) => {
                    // split into left and right
                    to_visit.push_back((Left, (x, y)));
                    to_visit.push_back((Right, (x, y)));
                }

                (Left, NS) | (Right, NS) => {
                    // split into up and down
                    to_visit.push_back((Up, (x, y)));
                    to_visit.push_back((Down, (x, y)));
                }

                (Right, NwSe) | (Left, NeSw) => {
                    for pos in Down.go(&self.0, x, y) {
                        to_visit.push_back((Down, pos));
                    }
                }

                (Left, NwSe) | (Right, NeSw) => {
                    for pos in Up.go(&self.0, x, y) {
                        to_visit.push_back((Up, pos));
                    }
                }

                (Up, NwSe) | (Down, NeSw) => {
                    for pos in Left.go(&self.0, x, y) {
                        to_visit.push_back((Left, pos));
                    }
                }

                (Down, NwSe) | (Up, NeSw) => {
                    for pos in Right.go(&self.0, x, y) {
                        to_visit.push_back((Right, pos));
                    }
                }
            };
        }

        seen.into_iter()
            .map(|(_dir, pos)| pos)
            .collect::<HashSet<_>>()
            .len() as u64
    }
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        Solution(
            input
                .lines()
                .map(|line| {
                    line.bytes()
                        .map(|c| match c {
                            b'.' => Empty,
                            b'|' => NS,
                            b'-' => EW,
                            b'\\' => NwSe,
                            b'/' => NeSw,
                            x => panic!("unexpected character {x:?}"),
                        })
                        .collect_vec()
                })
                .collect_vec(),
        )
    }

    fn part1(&self) -> anyhow::Result<u64> {
        Ok(self.count_from(Right, (0, 0)))
    }

    fn part2(&self) -> anyhow::Result<u64> {
        let mut to_trace = vec![];

        // top edge
        for y in 0..self.0[0].len() {
            to_trace.push((Down, (0, y)));
        }
        // left edge
        for x in 0..self.0.len() {
            to_trace.push((Right, (x, 0)));
        }
        // right edge
        for x in 0..self.0.len() {
            to_trace.push((Left, (x, self.0[0].len() - 1)))
        }
        // bottom edge
        for y in 0..self.0[0].len() {
            to_trace.push((Up, (self.0.len() - 1, y)))
        }

        to_trace
            .into_iter()
            .map(|(d, pos)| self.count_from(d, pos))
            .max()
            .ok_or_else(|| anyhow::anyhow!("we somehow found no edges to count at all"))
    }
}
