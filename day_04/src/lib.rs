use std::collections::BTreeMap;

use prelude::{anyhow::anyhow, *};

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
        let Some((_, id)) = left.split_once(' ') else {
            anyhow::bail!("could not find the card id");
        };
        let Some((winning, hand)) = right.split_once(" | ") else {
            anyhow::bail!("could not separate winning numbers");
        };

        Ok(Card {
            id: id.trim_start().parse().context("parse card id")?,
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

pub struct Solution(Vec<Card>);

impl Day for Solution {
    fn new(input: &str) -> Solution {
        Solution(
            input
                .lines()
                .map(Card::try_from)
                .collect::<Result<_, _>>()
                .expect("improperly formatted input"),
        )
    }

    fn part1(&self) -> anyhow::Result<u64> {
        Ok(self
            .0
            .iter()
            .map(|card| {
                let count = card
                    .hand
                    .iter()
                    .filter(|x| card.winning.contains(x))
                    .count() as u64;

                if count == 0 {
                    0
                } else {
                    1 << (count - 1)
                }
            })
            .sum())
    }

    fn part2(&self) -> anyhow::Result<u64> {
        let winning: BTreeMap<u64, usize> = self
            .0
            .iter()
            .map(|card| {
                let count = card
                    .hand
                    .iter()
                    .filter(|x| card.winning.contains(x))
                    .count();
                (card.id, count)
            })
            .collect();

        let mut card_counts: HashMap<u64, usize> = self.0.iter().map(|card| (card.id, 1)).collect();

        for card in &self.0 {
            let how_many_of_this_card =
                *card_counts.get(&card.id).expect("I counted all the cards");
            let how_many_follow = *winning.get(&card.id).expect("I counted all the winners") as u64;

            for next in (card.id + 1)..=(card.id + how_many_follow) {
                *card_counts.get_mut(&next).ok_or_else(|| {
                    anyhow!(
                        "card {} sent us to card {}, and we didn't find that one",
                        card.id,
                        next
                    )
                })? += how_many_of_this_card;
            }
        }

        Ok(card_counts.values().sum::<usize>() as u64)
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
