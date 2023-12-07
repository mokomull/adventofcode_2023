use prelude::*;

struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn ways_to_win(&self) -> u64 {
        (0..self.time)
            .filter(|held| held * (self.time - held) > self.record)
            .count() as u64
    }
}

pub struct Solution(Vec<Race>);

impl Solution {
    pub fn new(input: &str) -> Solution {
        let mut lines = input.lines();
        let times = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .trim_start()
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<u64>, _>>()
            .unwrap();
        let records = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .trim_start()
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<u64>, _>>()
            .unwrap();

        Solution(
            times
                .into_iter()
                .zip(records)
                .map(|(time, record)| Race { time, record })
                .collect(),
        )
    }

    pub fn part1(&self) -> anyhow::Result<u64> {
        Ok(self.0.iter().map(Race::ways_to_win).product())
    }

    pub fn part2(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented")
    }
}
