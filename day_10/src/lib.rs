use std::{cmp::min, collections::VecDeque};

use prelude::*;

#[derive(Debug, PartialEq)]
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
                    Ground => panic!("should not have reached ground at {:?}", (next_x, next_y)),
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

fn adjacent_including_diagonal_tripled_coordinate(
    data: &Vec<Vec<Tile>>,
    x: usize,
    y: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let first_index = x.saturating_sub(1)..=min(x + 1, data.len() * 3 - 1);
    // just use data[0], which assumes that the input is
    // (a) non-empty, and
    // (b) square.
    let second_index = y.saturating_sub(1)..=min(y + 1, data[0].len() * 3 - 1);

    first_index
        .cartesian_product(second_index)
        .filter(move |&(i, j)| (x, y) != (i, j))
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
        self.get_distances()?
            .into_values()
            .max()
            .ok_or_else(|| anyhow::anyhow!("somehow managed to visit no nodes"))
    }

    fn part2(&self) -> anyhow::Result<u64> {
        // Treat all of the pipes in the loop as 3x3 grids of
        //    _X_  _X_  ___
        //    _XX  _X_  XX_
        //    ___  _X_  _X_   , etc.
        // to allow the outside to flood-fill from (0,0).
        // And treat all pipes *not* in the loop as empty ground.

        let in_loop = self.get_distances()?;

        let mut seen = HashSet::new();
        // to_visit is in the *tripled* coordinate space.
        let mut to_visit = VecDeque::from(vec![(0, 0)]);

        // reverse-engineer what the starting tile must have been
        let starting_tile;
        {
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

            let (mut north, mut east, mut south, mut west) = (false, false, false, false);

            for (x, y) in adjacent_including_diagonal(&self.0, starting_x, starting_y) {
                if x != starting_x && y != starting_y {
                    // diagonal, so skip
                    continue;
                }

                if self.0[x][y] == Ground {
                    // only start with the two connected pipes
                    continue;
                }

                // and only start with pipes that flow to/from S; exclude pipes that make a glancing blow
                if !self.0[x][y]
                    .adjacent(&self.0, x, y)
                    .contains(&(starting_x, starting_y))
                {
                    continue;
                }

                if x < starting_x {
                    north = true;
                } else if y > starting_y {
                    east = true;
                } else if x > starting_x {
                    south = true;
                } else if y < starting_y {
                    west = true;
                }
            }

            starting_tile = match (north, east, south, west) {
                (true, false, true, false) => Vertical,
                (true, true, false, false) => NE,
                (true, false, false, true) => NW,
                (false, true, false, true) => Horizontal,
                (false, true, true, false) => SE,
                (false, false, true, true) => SW,
                _ => anyhow::bail!("found more than two pipes adjacent to the starting pipe"),
            };
        }

        while let Some((x, y)) = to_visit.pop_front() {
            if !seen.insert((x, y)) {
                // seen already contained (x, y)
                continue;
            }

            for (next_x, next_y) in adjacent_including_diagonal_tripled_coordinate(&self.0, x, y) {
                let tile_x = next_x / 3;
                let tile_y = next_y / 3;
                let mut tile = &self.0[tile_x][tile_y];

                if tile == &Starting {
                    tile = &starting_tile;
                }

                let expanded_tile = match tile {
                    _ if !in_loop.contains_key(&(tile_x, tile_y)) => [[false, false, false]; 3],
                    Starting => {
                        unreachable!("we replaced the starting tile with what it really is")
                    }
                    Vertical => [[false, true, false]; 3],
                    Horizontal => [
                        [false, false, false],
                        [true, true, true],
                        [false, false, false],
                    ],
                    NE => [
                        [false, true, false],
                        [false, true, true],
                        [false, false, false],
                    ],
                    NW => [
                        [false, true, false],
                        [true, true, false],
                        [false, false, false],
                    ],
                    SE => [
                        [false, false, false],
                        [false, true, true],
                        [false, true, false],
                    ],
                    SW => [
                        [false, false, false],
                        [true, true, false],
                        [false, true, false],
                    ],
                    Ground => [[false; 3]; 3],
                };

                if !expanded_tile[next_x % 3][next_y % 3] {
                    to_visit.push_back((next_x, next_y));
                }
            }
        }

        let mut res = 0;
        for x in 0..self.0.len() {
            for y in 0..self.0[x].len() {
                // check the center of each square that isn't part of the loop
                if !in_loop.contains_key(&(x, y)) && !seen.contains(&(x * 3 + 1, y * 3 + 1)) {
                    res += 1;
                }
            }
        }

        Ok(res)
    }
}

impl Solution {
    pub fn get_distances(&self) -> anyhow::Result<HashMap<(usize, usize), u64>> {
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
        log::debug!("starting at {:?}", (starting_x, starting_y));

        seen.insert((starting_x, starting_y), 0);

        for (x, y) in adjacent_including_diagonal(&self.0, starting_x, starting_y) {
            if x != starting_x && y != starting_y {
                // diagonal, so skip
                continue;
            }

            if self.0[x][y] == Ground {
                // only start with the two connected pipes
                continue;
            }

            // and only start with pipes that flow to/from S; exclude pipes that make a glancing blow
            if !self.0[x][y]
                .adjacent(&self.0, x, y)
                .contains(&(starting_x, starting_y))
            {
                continue;
            }

            to_visit.push_back(ToVisit {
                distance: 1,
                coord: (x, y),
            });
        }

        while let Some(i) = to_visit.pop_front() {
            log::debug!(
                "visiting {i:?}, which is a {:?}",
                self.0[i.coord.0][i.coord.1]
            );
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

        Ok(seen)
    }
}
