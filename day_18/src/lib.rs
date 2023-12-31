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
    color: [u8; 6],
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

        let color = color
            .as_bytes()
            .try_into()
            .expect("wrong number of hex digits");

        Plan {
            direction,
            count,
            color,
        }
    }
}

fn det(a: (i64, i64), b: (i64, i64)) -> i64 {
    a.0 * b.1 - a.1 * b.0
}

pub struct Solution(Vec<Plan>);

impl Day for Solution {
    fn new(input: &str) -> Self {
        Solution(input.lines().map(Plan::from).collect_vec())
    }

    fn part1(&self) -> anyhow::Result<u64> {
        let mut current = (0, 0);
        let mut vertices = vec![current];

        for i in &self.0 {
            // Unlike pretty much all the rest of my solutions this year, this will use +y as the "up" axis, and +x as the "right" axis.
            match i.direction {
                Up => current.1 += i.count as i64,
                Down => current.1 -= i.count as i64,
                Left => current.0 -= i.count as i64,
                Right => current.0 += i.count as i64,
            }

            vertices.push(current);
        }

        // Implement the "Shoelace Formula"
        let two_a = vertices
            .iter()
            .chain([&vertices[0]])
            .tuple_windows()
            .map(|(&a, &b)| det(a, b))
            .sum::<i64>();
        assert_eq!(0, two_a.abs() % 2);

        // Use Pick's theorem to determine the number of lattice points that are inside or on the
        // boundary.
        let area = two_a.abs() as u64 / 2;
        let perimiter = self.0.iter().map(|p| p.count as u64).sum::<u64>();
        // it doesn't necessarily follow that this must be true, but it certainly is for the
        // example, and we're about to integer-divide-by-2.
        assert_eq!(0, perimiter % 2);

        // interior plus boundary, in Pick's formula
        Ok(area + 1 + (perimiter / 2))
    }

    fn part2(&self) -> anyhow::Result<u64> {
        let plans = self
            .0
            .iter()
            .map(|p| {
                let direction = match p.color[5] {
                    b'0' => Right,
                    b'1' => Down,
                    b'2' => Left,
                    b'3' => Up,
                    _ => anyhow::bail!("unexpected direction hex digit in the bagging area"),
                };
                let count = usize::from_str_radix(std::str::from_utf8(&p.color[..5])?, 16)?;

                Ok(Plan {
                    direction,
                    count,
                    color: [0; 6],
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Solution(plans).part1()
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
                color: *b"70c710",
            }
        );
    }
}
