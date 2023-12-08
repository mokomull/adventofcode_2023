use std::cmp::Ordering::*;

use prelude::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Card(u8);

impl std::cmp::Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // 2 through 9 will sort normally, but let's assign TJQKA to arbitrarily
        // high values.
        fn map(x: u8) -> u8 {
            match x {
                b'2'..=b'9' => x,
                b'T' => 210,
                b'J' => 211,
                b'Q' => 212,
                b'K' => 213,
                b'A' => 214,
                _ => panic!("invalid card"),
            }
        }

        map(self.0).cmp(&map(other.0))
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
use Type::*;

#[derive(Clone, Eq)]
struct Hand {
    cards: [Card; 5],
    bet: u64,
}

impl Hand {
    fn get_type(&self) -> Type {
        let mut counts = HashMap::new();
        for x in self.cards {
            *counts.entry(x).or_default() += 1
        }

        let mut counts = counts.values().collect_vec();
        counts.sort();

        match &counts[..] {
            [5] => FiveOfAKind,
            [4, 1] => FourOfAKind,
            [3, 2] => FullHouse,
            [3, 1, 1] => ThreeOfAKind,
            [2, 2, 1] => TwoPair,
            [2, 1, 1, 1] => OnePair,
            [1, 1, 1, 1, 1] => HighCard,
            _ => panic!("unexpected counts {:?}", counts),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.get_type().cmp(&other.get_type()) {
            core::cmp::Ordering::Equal => self.cards.cmp(&other.cards),
            ord => return ord,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Equal
    }
}

pub struct Solution(Vec<Hand>);

impl Solution {
    pub fn new(input: &str) -> Solution {
        init();

        Solution(
            input
                .lines()
                .map(|line| Hand {
                    cards: line[..5]
                        .bytes()
                        .map(Card)
                        .collect_vec()
                        .try_into()
                        .expect("five cards makes a hand"),
                    bet: line[6..].parse().expect("could not parse bet"),
                })
                .collect(),
        )
    }

    pub fn part1(&self) -> anyhow::Result<u64> {
        let mut hands = self.0.clone();
        hands.sort();
        Ok(hands
            .into_iter()
            .enumerate()
            .map(|(rank, hand)| hand.bet * (rank as u64 + 1))
            .sum())
    }

    pub fn part2(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented");
    }
}
