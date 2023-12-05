use std::collections::BTreeMap;

use prelude::*;

type Map = BTreeMap<u64, (u64, u64)>;

pub struct Solution {
    seeds: Vec<u64>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

fn parse_map<'a>(it: impl Iterator<Item = &'a str>) -> Map {
    let mut map = Map::new();

    let triples = it
        .take_while(|x| !x.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().expect("not an integer"))
                .collect_vec()
                .try_into()
                .expect("more or fewer than three in a line")
        })
        .map(|junk: [u64; 3]| (junk[0], junk[1], junk[2]));

    for (to, from, count) in triples {
        map.insert(from, (to, count));
    }

    map
}

fn get(map: &Map, k: u64) -> u64 {
    let previous = map.range(..=k).rev().next();
    if let Some((from, (to, count))) = previous {
        assert!(k >= *from, "looked up {}, found {}", k, from);
        let offset = k - from;
        if offset <= *count {
            return to + offset;
        } else {
            return k;
        }
    } else {
        return k;
    }
}

impl Solution {
    pub fn new(input: &str) -> Solution {
        init();

        let mut lines = input.lines();

        let seeds = lines.next().expect("no seeds");
        let seeds = seeds
            .split_whitespace()
            .skip(1)
            .map(|seed| {
                log::debug!("{:?}", seed);
                seed.parse().expect("seed not an integer?")
            })
            .collect();
        assert_eq!(lines.next().unwrap(), "");

        assert_eq!(lines.next().unwrap(), "seed-to-soil map:");
        let seed_to_soil = parse_map(&mut lines);
        assert_eq!(lines.next().unwrap(), "soil-to-fertilizer map:");
        let soil_to_fertilizer = parse_map(&mut lines);
        assert_eq!(lines.next().unwrap(), "fertilizer-to-water map:");
        let fertilizer_to_water = parse_map(&mut lines);
        assert_eq!(lines.next().unwrap(), "water-to-light map:");
        let water_to_light = parse_map(&mut lines);
        assert_eq!(lines.next().unwrap(), "light-to-temperature map:");
        let light_to_temperature = parse_map(&mut lines);
        assert_eq!(lines.next().unwrap(), "temperature-to-humidity map:");
        let temperature_to_humidity = parse_map(&mut lines);
        assert_eq!(lines.next().unwrap(), "humidity-to-location map:");
        let humidity_to_location = parse_map(&mut lines);

        Solution {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }

    pub fn part1(&self) -> anyhow::Result<u64> {
        let mut result = u64::MAX;

        for &seed in &self.seeds {
            log::debug!("seed {}", seed);
            let idx = get(&self.seed_to_soil, seed);
            log::debug!("idx {}", idx);
            let idx = get(&self.soil_to_fertilizer, idx);
            log::debug!("idx {}", idx);
            let idx = get(&self.fertilizer_to_water, idx);
            log::debug!("idx {}", idx);
            let idx = get(&self.water_to_light, idx);
            log::debug!("idx {}", idx);
            let idx = get(&self.light_to_temperature, idx);
            log::debug!("idx {}", idx);
            let idx = get(&self.temperature_to_humidity, idx);
            log::debug!("idx {}", idx);
            let idx = get(&self.humidity_to_location, idx);
            log::debug!("idx {}", idx);

            result = std::cmp::min(result, idx);
        }

        Ok(result)
    }

    pub fn part2(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example = Solution::new(EXAMPLE);
        assert_eq!(example.part1().unwrap(), 35);
        assert_eq!(example.part2().unwrap(), 46);
    }

    static EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
}
