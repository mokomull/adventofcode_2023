use prelude::*;

enum Tile {
    Empty,
    NS,
    EW,
    NeSw,
    NwSe,
}

use Tile::*;

enum Direction {
    Up,
    Left,
    Down,
    Right,
}

use Direction::*;

pub struct Solution(Vec<Vec<Tile>>);

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
                            x => panic!("unexpected character {x:?}")
                        })
                        .collect_vec()
                })
                .collect_vec(),
        )
    }

    fn part1(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented")
    }

    fn part2(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented")
    }
}
