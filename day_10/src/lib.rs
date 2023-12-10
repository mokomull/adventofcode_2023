use std::collections::VecDeque;

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
        #[derive(Hash, Eq, PartialEq)]
        enum Side {
            Left,
            Right,
            Top,
            Bottom,
        }
        use Side::*;

        let mut seen: HashMap<(usize, usize), HashSet<Side>> = HashMap::new();
        // assume all four sides of (0,0) are on the "outside"
        let mut to_visit: VecDeque<_> = vec![
            ((0, 0), Left),
            ((0, 0), Right),
            ((0, 0), Top),
            ((0, 0), Bottom),
        ]
        .into();

        while let Some(((x, y), side)) = to_visit.pop_front() {
            if !seen.entry((x, y)).or_default().insert(side) {
                // we've already seen this side of this tile, so don't worry about it
                continue;
            }

            match side {
                Left => {
                    match self.0[x][y] {
                        Ground => 
                    }
                }
            }
        }

        anyhow::bail!("unimplemented");
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
