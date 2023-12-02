use prelude::*;

pub struct Solution {
    passwords: Vec<String>,
}

impl Solution {
    pub fn new(input: &str) -> Solution {
        init();

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
        let re = Regex::new("one|two|three|four|five|six|seven|eight|nine|zero").unwrap();
        let modified_passwords = self
            .passwords
            .iter()
            .map(|p| {
                re.replace(p, |capture: &regex::Captures<'_>| match capture.get(0) {
                    Some(m) => match m.as_str() {
                        "one" => "1",
                        "two" => "2",
                        "three" => "3",
                        "four" => "4",
                        "five" => "5",
                        "six" => "6",
                        "seven" => "7",
                        "eight" => "8",
                        "nine" => "9",
                        "ten" => "10",
                        x => panic!("ran on unexpected group: {:?}", x),
                    },
                    None => panic!("zeroth capture is guaranteed to exist"),
                })
                .into_owned()
            })
            .collect_vec();
        
        log::debug!("{:?}", modified_passwords);

        Solution {
            passwords: modified_passwords,
        }
        .part1()
    }
}
