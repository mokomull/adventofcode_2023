use std::collections::{BTreeSet, VecDeque};

use prelude::*;

pub struct Solution(Vec<Vec<u8>>);

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

use Direction::*;

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Left => Right,
            Right => Left,
            Up => Down,
            Down => Up,
        }
    }
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        Solution(
            input
                .lines()
                .map(|line| {
                    line.bytes()
                        .map(|b| {
                            if b.is_ascii_digit() {
                                b - b'0'
                            } else {
                                panic! {"{b:?} is not a digit"}
                            }
                        })
                        .collect_vec()
                })
                .collect_vec(),
        )
    }

    fn part1(&self) -> anyhow::Result<u64> {
        let shortest = self.shortest_paths(3, |ds| {
            let mut res: BTreeSet<_> = [Left, Down, Up, Right].into();
            if ds.len() == 3 && ds.iter().all(|&d| d == ds[0]) {
                res.remove(&ds[0]);
            }
            if let Some(last) = ds.last() {
                res.remove(&last.opposite());
            }
            res
        });

        let end = (self.0.len() - 1, self.0[0].len() - 1);

        shortest
            .into_iter()
            .filter_map(
                |((coords, _), cost)| {
                    if coords == end {
                        Some(cost)
                    } else {
                        None
                    }
                },
            )
            .min()
            .ok_or_else(|| anyhow::anyhow!("Did not find a path to the end"))
            .map(|c| c as u64 + self.0[self.0.len() - 1][self.0[0].len() - 1] as u64)
    }

    fn part2(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented")
    }
}

impl Solution {
    fn shortest_paths<F>(
        &self,
        length_limit: usize,
        directions_from_here: F,
    ) -> HashMap<((usize, usize), Vec<Direction>), i64>
    where
        F: Fn(&[Direction]) -> BTreeSet<Direction>,
    {
        struct Visit {
            coords: (usize, usize),
            last_directions: Vec<Direction>,
            cost: i64,
        }
        let mut shortest: HashMap<((usize, usize), Vec<Direction>), i64> = HashMap::new();
        let mut to_visit = VecDeque::from([Visit {
            coords: (0, 0),
            last_directions: vec![],
            cost: -(self.0[0][0] as i64),
        }]);

        while let Some(v) = to_visit.pop_front() {
            let possible_directions = directions_from_here(&v.last_directions);
            let key = (v.coords, v.last_directions.clone());
            if let Some(&c) = shortest.get(&key) {
                if c <= v.cost {
                    continue;
                }
            }
            shortest.insert(key, v.cost);

            let next = adjacent_including_diagonal(&self.0, v.coords.0, v.coords.1)
                .filter(|&(x, y)| x == v.coords.0 || y == v.coords.1)
                .map(|coords| {
                    (
                        coords,
                        match () {
                            () if coords.0 > v.coords.0 => Down,
                            () if coords.0 < v.coords.0 => Up,
                            () if coords.1 > v.coords.1 => Right,
                            () if coords.1 < v.coords.1 => Left,
                            _ => unreachable!(
                                "we shouldn't iterate over the same point: {:?} == {:?}",
                                coords, v.coords
                            ),
                        },
                    )
                });

            let cost = v.cost + self.0[v.coords.0][v.coords.1] as i64;

            for (next, direction) in next {
                if !possible_directions.contains(&direction) {
                    continue;
                }

                let mut next_directions = v.last_directions.clone();
                next_directions.push(direction);
                if next_directions.len() > length_limit {
                    next_directions.remove(0);
                }

                to_visit.push_back(Visit {
                    coords: next,
                    last_directions: next_directions,
                    cost,
                })
            }
        }

        shortest
    }
}
