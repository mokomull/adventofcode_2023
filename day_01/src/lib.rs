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
                let first = *p.iter().find(|b| b.is_ascii_digit()).unwrap_or(&b'0');
                let last = *p.iter().rev().find(|b| b.is_ascii_digit()).unwrap_or(&b'0');
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
        self.passwords
            .iter()
            .map(|p_str| {
                let p = p_str.as_bytes();
                let rev_p = p.iter().cloned().rev().collect_vec();

                let first = p
                    .iter()
                    .enumerate()
                    .filter_map(|(i, c)| {
                        if c.is_ascii_digit() {
                            return Some(*c);
                        }

                        match &p[i..] {
                            x if x.starts_with(b"zero") => Some(b'0'),
                            x if x.starts_with(b"one") => Some(b'1'),
                            x if x.starts_with(b"two") => Some(b'2'),
                            x if x.starts_with(b"three") => Some(b'3'),
                            x if x.starts_with(b"four") => Some(b'4'),
                            x if x.starts_with(b"five") => Some(b'5'),
                            x if x.starts_with(b"six") => Some(b'6'),
                            x if x.starts_with(b"seven") => Some(b'7'),
                            x if x.starts_with(b"eight") => Some(b'8'),
                            x if x.starts_with(b"nine") => Some(b'9'),
                            _ => None,
                        }
                    })
                    .next()
                    .unwrap_or(b'0');

                let last = rev_p
                    .iter()
                    .enumerate()
                    .filter_map(|(i, c)| {
                        if c.is_ascii_digit() {
                            return Some(*c);
                        }

                        match &rev_p[i..] {
                            x if x.starts_with(b"orez") => Some(b'0'),
                            x if x.starts_with(b"eno") => Some(b'1'),
                            x if x.starts_with(b"owt") => Some(b'2'),
                            x if x.starts_with(b"eerht") => Some(b'3'),
                            x if x.starts_with(b"ruof") => Some(b'4'),
                            x if x.starts_with(b"evif") => Some(b'5'),
                            x if x.starts_with(b"xis") => Some(b'6'),
                            x if x.starts_with(b"neves") => Some(b'7'),
                            x if x.starts_with(b"thgie") => Some(b'8'),
                            x if x.starts_with(b"enin") => Some(b'9'),
                            _ => None,
                        }
                    })
                    .next()
                    .unwrap_or(b'0');

                let number = [first, last];
                let number =
                    std::str::from_utf8(&number).expect("two ASCII digits must be valid UTF-8");
                log::debug!("password {} -> digits {}", p_str, number);
                number
                    .parse::<u64>()
                    .expect("two ASCII digits should parse as a u64 successfully")
            })
            .sum()
    }
}
