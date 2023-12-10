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
        // let mut seen = HashSet::new();
        // let mut to_visit = VecDeque::new();

        let starting = self
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

        anyhow::bail!("unimplemented");
    }

    fn part2(&self) -> anyhow::Result<u64> {
        todo!()
    }
}
