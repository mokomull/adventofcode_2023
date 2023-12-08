use prelude::*;

struct Node {
    left: String,
    right: String,
}

pub struct Solution {
    directions: String,
    map: HashMap<String, Node>,
}

impl Solution {
    pub fn new(input: &str) -> Solution {
        init();
        let mut lines = input.lines();

        let directions = lines.next().unwrap().to_owned();
        lines.next().unwrap();

        let mut map = HashMap::new();
        for line in lines {
            let (from, rest) = line.split_once(" = ").unwrap();
            let (left, right) = rest.split_once(", ").unwrap();
            map.insert(
                from.to_owned(),
                Node {
                    left: left[1..].to_owned(),
                    right: right[..3].to_owned(),
                },
            );
        }

        Self { directions, map }
    }

    pub fn part1(&self) -> anyhow::Result<u64> {
        let mut current = "AAA";
        for (i, direction) in self.directions.bytes().cycle().enumerate() {
            log::debug!("at {:?}, step {}, direction {}", current, i, direction);
            if current == "ZZZ" {
                return Ok(i as u64);
            }

            current = match direction {
                b'L' => &self.map[current].left,
                b'R' => &self.map[current].right,
                x => panic!("unexpected direction {:?}", x),
            }
        }
        unreachable!()
    }

    pub fn part2(&self) -> anyhow::Result<u64> {
        let mut currents = self
            .map
            .keys()
            .filter(|name| name.ends_with('A'))
            .collect_vec();

        for (i, direction) in self.directions.bytes().cycle().enumerate() {
            log::debug!("at {:?}, step {}, direction {}", currents, i, direction);
            if currents.iter().all(|&name| name.ends_with('Z')) {
                return Ok(i as u64);
            }

            for name in currents.iter_mut() {
                *name = match direction {
                    b'L' => &self.map[*name].left,
                    b'R' => &self.map[*name].right,
                    x => panic!("unexpected direction {:?}", x),
                }
            }
        }
        unreachable!()
    }
}
