use std::{
    cmp::{max, min},
    collections::BTreeMap,
};

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

fn get_ranges(map: &Map, mut k: u64, mut target_count: u64) -> Vec<(u64, u64)> {
    let mut res = vec![];

    // Some() if our start actually exists in a range
    let previous = map
        .range(..=k)
        .rev()
        .next()
        .filter(|(&from, &(to, count))| {
            assert!(k >= from);
            k - from <= count
        });
    if let Some((from, (to, count))) = previous {
        let offset = k - from;
        let end_count = min(target_count, count - offset);
        res.push((to + offset, end_count));
        target_count -= end_count;
        k += end_count;
    }

    while target_count != 0 {
        // invariant: k is *outside* any range in map

        let next = map.range(k..).next();
        if let Some((&from, &(next_to, next_count))) = next {
            assert!(from > k);

            // these are the values from k to the next segment
            let identity_count = min(target_count, from - k);
            res.push((k, identity_count));
            k += identity_count;
            target_count -= identity_count;

            if target_count == 0 {
                break;
            }

            // these are the values within the next segment
            let segment_count = min(target_count, next_count);
            res.push((next_to, segment_count));
            k += segment_count;
            target_count -= segment_count;
        } else {
            // there is no next segment in the map, so write out the last segment as mapping target_count values from k to k.
            res.push((k, target_count));
            break;
        }
    }

    res
}

fn get_all_ranges(map: &Map, ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    log::debug!("ranges: {:?}", ranges);

    ranges
        .into_iter()
        .flat_map(|(k, count)| get_ranges(map, k, count).into_iter())
        .collect()
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
        let ranges = self
            .seeds
            .iter()
            .tuples()
            .map(|(&k, &count)| (k, count))
            .collect_vec();
        let ranges = get_all_ranges(&self.seed_to_soil, ranges);
        let ranges = get_all_ranges(&self.soil_to_fertilizer, ranges);
        let ranges = get_all_ranges(&self.fertilizer_to_water, ranges);
        let ranges = get_all_ranges(&self.water_to_light, ranges);
        let ranges = get_all_ranges(&self.light_to_temperature, ranges);
        let ranges = get_all_ranges(&self.temperature_to_humidity, ranges);
        let ranges = get_all_ranges(&self.humidity_to_location, ranges);

        ranges
            .into_iter()
            .map(|(start, _)| start)
            .min()
            .ok_or_else(|| anyhow::anyhow!("seriously, no matches?"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        env_logger::init();

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
