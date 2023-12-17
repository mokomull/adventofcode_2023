use std::collections::VecDeque;

use prelude::*;

pub struct Solution(Vec<Vec<u8>>);

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

use Direction::*;

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
        struct Visit {
            coords: (usize, usize),
            last_directions: Vec<Direction>,
            cost: u64,
        }
        let mut shortest: HashMap<(usize, usize), u64> = HashMap::new();
        let mut to_visit = VecDeque::from([Visit {
            coords: (0, 0),
            last_directions: vec![],
            cost: 0,
        }]);

        while let Some(v) = to_visit.pop_front() {
            if let Some(&c) = shortest.get(&v.coords) {
                if c >= v.cost {
                    // TODO: maybe I need to actually consider the directions we could go from here,
                    // too; perhaps getting to coords from a different direction would end up
                    // cheaper overall?
                    continue;
                }
            }
            shortest.insert(v.coords, v.cost);

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

            let cost = v.cost + self.0[v.coords.0][v.coords.1] as u64;

            for (next, direction) in next {
                if v.last_directions.len() == 3 && v.last_directions.iter().all(|&d| d == direction)
                {
                    // we can't keep going this way
                    continue;
                }

                let mut next_directions = v.last_directions.clone();
                next_directions.push(direction);
                if next_directions.len() == 3 {
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
            .get(&(self.0.len() - 1, self.0[0].len() - 1))
            .ok_or_else(|| anyhow::anyhow!("Did not find a path to the end"))
            .map(|&c| c + self.0[self.0.len() - 1][self.0[0].len() - 1] as u64)
    }

    fn part2(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented")
    }
}
