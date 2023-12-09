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

impl Day for Solution {
    fn new(input: &str) -> Solution {
        let mut lines = input.lines();
        let times = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
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

    fn part1(&self) -> anyhow::Result<u64> {
        Ok(self.0.iter().map(Race::ways_to_win).product())
    }

    fn part2(&self) -> anyhow::Result<u64> {
        let times = self.0.iter().map(|x| format!("{}", x.time)).join("");
        let records = self.0.iter().map(|x| format!("{}", x.record)).join("");

        Solution::new(&format!("Time: {}\nDistance: {}\n", times, records)).part1()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let solution = Solution::new(EXAMPLE);
        assert_eq!(solution.part1().unwrap(), 288);
        assert_eq!(solution.part2().unwrap(), 71503);
    }

    #[test]
    fn personal_input() {
        let solution = Solution::new(INPUT);
        assert_eq!(solution.part1().unwrap(), 608902);
        assert_eq!(solution.part2().unwrap(), 46173809);
    }

    static EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    static INPUT: &str = "Time:        55     82     64     90
Distance:   246   1441   1012   1111";
}
