use prelude::*;

fn hash(data: &[u8]) -> u8 {
    let mut value = 0u8;

    for &i in data.iter() {
        value = value.wrapping_add(i);
        value = value.wrapping_mul(17);
    }

    value
}

enum Operation<'a> {
    Set { label: &'a str, focal_length: u64 },
    Remove(&'a str),
}

impl<'a> TryFrom<&'a str> for Operation<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if let Some(x) = value.strip_suffix('-') {
            return Ok(Operation::Remove(x));
        }

        if let Some((label, r)) = value.split_once('=') {
            let focal_length = r.parse()?;
            return Ok(Operation::Set {
                label,
                focal_length,
            });
        }

        anyhow::bail!("found neither a = nor a - in {}", value)
    }
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
        const EMPTY: Vec<(&str, u64)> = vec![]; // workaround, because [vec![]; 256] complains that Vec is not Copy
        let mut boxes: [Vec<(&str, u64)>; 256] = [EMPTY; 256];

        for step in &self.0 {
            let step = Operation::try_from(step.as_str())?;

            match step {
                Operation::Remove(label) => {
                    let this_box = &mut boxes[hash(label.as_bytes()) as usize];
                    let loc = this_box.iter().position(|x| x.0 == label);
                    if let Some(i) = loc {
                        this_box.remove(i);
                    }
                }
                Operation::Set {
                    label,
                    focal_length,
                } => {
                    let this_box = &mut boxes[hash(label.as_bytes()) as usize];
                    let loc = this_box.iter().position(|x| x.0 == label);
                    if let Some(i) = loc {
                        this_box[i].1 = focal_length
                    } else {
                        this_box.push((label, focal_length))
                    }
                }
            }
        }

        Ok(boxes
            .iter()
            .enumerate()
            .flat_map(|(box_number, this_box)| {
                this_box
                    .iter()
                    .enumerate()
                    .map(move |(slot, &(_label, focal_length))| {
                        (box_number as u64 + 1) * (slot as u64 + 1) * focal_length
                    })
            })
            .sum())
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
