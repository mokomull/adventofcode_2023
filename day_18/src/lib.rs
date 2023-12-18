use prelude::*;

#[derive(Debug, PartialEq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}
use Direction::*;

#[derive(Debug, PartialEq)]
struct Plan {
    direction: Direction,
    count: usize,
    color: (u8, u8, u8),
}

impl From<&str> for Plan {
    fn from(value: &str) -> Self {
        let (direction, rest) = value.split_once(' ').expect("missing any spaces");
        let (count, rest) = rest.split_once(' ').expect("couldn't find second space");
        let color = rest.strip_prefix("(#").expect("color missing left-paren");
        let color = color.strip_suffix(')').expect("color missing right-paren");

        let direction = match direction {
            "R" => Right,
            "L" => Left,
            "U" => Up,
            "D" => Down,
            _ => panic!("unexpected direction {direction:?}"),
        };

        let count = count.parse().expect("bad integer");

        let color: [u8; 3] = color
            .as_bytes()
            .chunks(2)
            .map(|chunk| {
                let color = std::str::from_utf8(chunk).expect("somehow the color isn't UTF-8");
                u8::from_str_radix(color, 16).expect("invalid hex")
            })
            .collect_vec()
            .try_into()
            .expect("wrong amount of data in color");
        let [r, g, b] = color;

        Plan {
            direction,
            count,
            color: (r, g, b),
        }
    }
}

pub struct Solution();

impl Day for Solution {
    fn new(input: &str) -> Self {
        Solution()
    }

    fn part1(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented")
    }

    fn part2(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parser() {
        assert_eq!(
            Plan::from("R 6 (#70c710)"),
            Plan {
                direction: Right,
                count: 6,
                color: (0x70, 0xc7, 0x10),
            }
        )
    }
}
