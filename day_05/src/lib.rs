use prelude::*;

type Map = HashMap<u64, u64>;

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

    for (from, to, count) in triples {
        for offset in 0..count {
            map.insert(from + offset, to + offset);
        }
    }

    map
}

impl Solution {
    pub fn new(input: &str) -> Solution {
        init();

        let mut lines = input.lines();

        let seeds = lines.next().expect("no seeds");
        let seeds = seeds
            .split_whitespace()
            .map(|seed| seed.parse().expect("seed not an integer?"))
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
        anyhow::bail!("unimplemented")
    }

    pub fn part2(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented")
    }
}
