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
        let starting = self
            .map
            .keys()
            .filter(|name| name.ends_with('A'))
            .collect_vec();

        #[derive(Debug)]
        struct Data {
            steps_to_cycle_start: usize,
            cycle_length: usize,
            steps_to_end: usize,
        }
        let mut data = HashMap::new();

        for s in starting {
            let mut steps_to_end = None;

            let mut current = s;
            let mut seen = HashMap::new();
            for (i, direction) in self.directions.bytes().cycle().enumerate() {
                if steps_to_end.is_none() && current.ends_with('Z') {
                    steps_to_end = Some(i);
                }

                // because not only do we have to be at the same node, but we
                // have to be at the same node *executing the same steps* for it
                // to count as a cycle.
                let absolute_step = i % self.directions.len();
                if let Some(&steps_to_cycle_start) = seen.get(&(current, absolute_step)) {
                    data.insert(
                        s,
                        Data {
                            steps_to_cycle_start,
                            cycle_length: i - steps_to_cycle_start,
                            steps_to_end: steps_to_end.unwrap(),
                        },
                    );
                    break;
                }
                seen.insert((current, absolute_step), i);

                current = match direction {
                    b'L' => &self.map[current].left,
                    b'R' => &self.map[current].right,
                    x => panic!("unexpected direction {:?}", x),
                }
            }
        }

        log::info!("collected data: {:?}", data);

        // Assuming that we'll end up finding something well beyond every
        // starting position's cycle, we're looking for a position that satifies
        //
        //   x = steps_to_end (mod cycle_length)
        //
        // note that steps_to_cycle_start is not in that equation, since
        // steps_to_end started at *the beginning* rather than the beginning of
        // the cycle.  It will simply be used to verify that we calculated a
        // sufficiently large step count.
        let res = ring_algorithm::chinese_remainder_theorem(
            &data
                .values()
                .map(|d| d.steps_to_end % d.cycle_length)
                .collect_vec(),
            &data.values().map(|d| d.cycle_length).collect_vec(),
        )
        .unwrap();

        assert!(data.values().all(|d| res > d.steps_to_cycle_start));

        Ok(res as u64)
    }
}
