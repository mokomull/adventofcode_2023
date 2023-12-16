use prelude::*;

fn hash(data: &[u8]) -> u8 {
    let mut value = 0u8;

    for &i in data.iter() {
        value = value.wrapping_add(i);
        value = value.wrapping_mul(17);
    }

    value
}

pub struct Solution(Vec<String>);

impl Day for Solution {
    fn new(input: &str) -> Self {
        Solution(input.trim().split(',').map(|s| s.to_owned()).collect())
    }

    fn part1(&self) -> anyhow::Result<u64> {
        Ok(self.0.iter().map(|x| hash(x.as_bytes()) as u64).sum())
    }

    fn part2(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(30, hash(b"rn=1"));
        assert_eq!(253, hash(b"cm-"));
    }
}
