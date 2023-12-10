use std::collections::VecDeque;

use prelude::*;

#[derive(PartialEq)]
enum Tile {
    Ground,
    NE,
    SE,
    NW,
    SW,
    Vertical,
    Horizontal,
    Starting,
}

use Tile::*;

impl Tile {
    fn adjacent<'a>(
        &'a self,
        data: &Vec<Vec<Tile>>,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize)> + 'a {
        adjacent_including_diagonal(data, x, y).filter(move |&(next_x, next_y)| {
            (next_x == x || next_y == y) // filter out diagonals
                && match self {
                    Ground => false,
                    NE => next_x < x || next_y > y,
                    SE => next_x > x || next_y > y,
                    NW => next_x < x || next_y < y,
                    SW => next_x > x || next_y < y,
                    Vertical => next_y == y,
                    Horizontal => next_x == x,
                    Starting => false,
                }
        })
    }
}

pub struct Solution(Vec<Vec<Tile>>);

impl Day for Solution {
    fn new(input: &str) -> Self {
        Solution(
            input
                .lines()
                .map(|line| {
                    line.bytes()
                        .map(|b| match b {
                            b'.' => Ground,
                            b'L' => NE,
                            b'F' => SE,
                            b'J' => NW,
                            b'7' => SW,
                            b'-' => Horizontal,
                            b'|' => Vertical,
                            b'S' => Starting,
                            x => panic!("unexpected tile: {x:?}"),
                        })
                        .collect_vec()
                })
                .collect_vec(),
        )
    }

    fn part1(&self) -> anyhow::Result<u64> {
        #[derive(Debug)]
        struct ToVisit {
            distance: u64,
            coord: (usize, usize),
        }

        let mut seen = HashMap::new();
        let mut to_visit = VecDeque::new();

        let (starting_x, starting_y) = self
            .0
            .iter()
            .enumerate()
            .find_map(|(i, row)| {
                row.iter().enumerate().find_map(|(j, tile)| {
                    if tile == &Starting {
                        Some((i, j))
                    } else {
                        None
                    }
                })
            })
            .ok_or_else(|| anyhow::anyhow!("there was no starting tile"))?;

        for (x, y) in adjacent_including_diagonal(&self.0, starting_x, starting_y) {
            if x != starting_x && y != starting_y {
                // diagonal, so skip
                continue;
            }

            to_visit.push_back(ToVisit {
                distance: 1,
                coord: (x, y),
            });
        }

        while let Some(i) = to_visit.pop_front() {
            log::debug!("visiting {i:?}");
            if let Some(existing) = seen.get(&i.coord) {
                if *existing <= i.distance {
                    continue;
                }
            }
            seen.insert(i.coord, i.distance);

            let distance = i.distance + 1;

            let (x, y) = i.coord;
            for (next_x, next_y) in self.0[x][y].adjacent(&self.0, x, y) {
                log::debug!("neighbor is {:?}", (next_x, next_y));
                to_visit.push_back(ToVisit {
                    distance,
                    coord: (next_x, next_y),
                });
            }
        }

        seen.into_values()
            .max()
            .ok_or_else(|| anyhow::anyhow!("somehow managed to visit no nodes"))
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}
