use prelude::*;

pub struct Solution {
    passwords: Vec<String>,
}

impl Solution {
    pub fn new(input: &str) -> Solution {
        Solution {
            passwords: input.lines().map(str::to_owned).collect_vec(),
        }
    }

    pub fn part1(&self) -> u64 {
        self.passwords
            .iter()
            .map(|p| {
                let p = p.as_bytes();
                let first = *p.iter().find(|b| b.is_ascii_digit()).unwrap();
                let last = *p.iter().rev().find(|b| b.is_ascii_digit()).unwrap();
                let number = [first, last];
                let number =
                    std::str::from_utf8(&number).expect("two ASCII digits must be valid UTF-8");
                number
                    .parse::<u64>()
                    .expect("two ASCII digits should parse as a u64 successfully;e")
            })
            .sum()
    }

    pub fn part2(&self) -> u64 {
        42
    }
}
