use prelude::*;

fn hash(data: &[u8]) -> u8 {
    let mut value = 0u8;

    for &i in data.iter() {
        value = value.wrapping_add(i);
        value = value.wrapping_mul(17);
    }

    value
}

pub struct Solution();

impl Day for Solution {
    fn new(input: &str) -> Self {
        Solution()
    }

    fn part1(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented")
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
