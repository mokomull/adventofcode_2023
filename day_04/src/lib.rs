use prelude::*;

#[derive(Debug, PartialEq)]
struct Card {
    id: u64,
    winning: Vec<u64>,
    hand: Vec<u64>,
}

impl TryFrom<&str> for Card {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let Some((left, right)) = value.split_once(": ") else {
            anyhow::bail!("did not find a colon")
        };
        let Some((_, id)) = left.split_once(" ") else {
            anyhow::bail!("could not find the card id");
        };
        let Some((winning, hand)) = right.split_once(" | ") else {
            anyhow::bail!("could not separate winning numbers");
        };

        Ok(Card {
            id: id.parse().context("parse card id")?,
            winning: winning
                .split_whitespace()
                .map(str::parse)
                .collect::<Result<_, _>>()
                .context("parsing winning numbers")?,
            hand: hand
                .split_whitespace()
                .map(str::parse)
                .collect::<Result<_, _>>()
                .context("parsing winning numbers")?,
        })
    }
}

pub struct Solution();

impl Solution {
    pub fn new(input: &str) -> Solution {
        Solution()
    }

    pub fn part1(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented")
    }

    pub fn part2(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser() {
        assert_eq!(
            Card::try_from("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1").unwrap(),
            Card {
                id: 3,
                winning: vec![1, 21, 53, 59, 44],
                hand: vec![69, 82, 63, 72, 16, 21, 14, 1],
            }
        );
    }
}
