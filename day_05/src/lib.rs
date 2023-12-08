use std::{cmp::min, collections::BTreeMap};

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
    let previous = map.range(..=k).next_back();
    if let Some((from, (to, count))) = previous {
        assert!(k >= *from, "looked up {}, found {}", k, from);
        let offset = k - from;
        if offset <= *count {
            to + offset
        } else {
            k
        }
    } else {
        k
    }
}

fn get_ranges(map: &Map, mut k: u64, mut target_count: u64) -> Vec<(u64, u64)> {
    let mut res = vec![];

    // Some() if our start actually exists in a range
    let previous = map
        .range(..=k)
        .next_back()
        .filter(|(&from, &(_to, count))| {
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
        // invariant: k is either exactly the beginning of a mapped range, or
        // fully outside one.  We must have fully exhausted any partial range
        // before here.
        log::debug!("starting on k = {}, count = {}", k, target_count);

        let next = map.range(k..).next();
        if let Some((&from, &(next_to, next_count))) = next {
            if from > k {
                // k is outside any mapped range
                // these are the values from k to the next segment
                let identity_count = min(target_count, from - k);
                res.push((k, identity_count));
                k += identity_count;
                target_count -= identity_count;
            } else {
                // these are the values within the this segment
                let segment_count = min(target_count, next_count);
                res.push((next_to, segment_count));
                k += segment_count;
                target_count -= segment_count;
            }
        } else {
            // there is no next segment in the map, so write out the last
            // segment as mapping target_count values from k to k.
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

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn example() {
        init();

        let example = Solution::new(EXAMPLE);
        assert_eq!(example.part1().unwrap(), 35);
        assert_eq!(example.part2().unwrap(), 46);
    }

    #[test]
    fn personal_input() {
        init();

        let solution = Solution::new(INPUT);
        assert_eq!(solution.part1().unwrap(), 111627841);
        assert_eq!(solution.part2().unwrap(), 69323688);
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

    static INPUT: &str = "seeds: 3136945476 509728956 1904897211 495273540 1186343315 66026055 1381149926 11379441 4060485949 190301545 444541979 351779229 1076140984 104902451 264807001 60556152 3676523418 44140882 3895155702 111080695

seed-to-soil map:
2122609492 2788703865 117293332
751770532 1940296486 410787026
2652142963 2905997197 464992562
3442443139 3721315963 573651333
3117135525 2356966701 133002244
742051533 3370989759 9718999
2239902824 720032349 393589935
1162557558 58715335 661317014
1823874572 2489968945 298734920
2633492759 2351083512 5883189
4016094472 3442443139 278872824
58715335 1256960288 683336198
3250137769 1126389299 130570989
2639375948 1113622284 12767015

soil-to-fertilizer map:
1839905294 2992775329 34548650
266781855 3027323979 163164353
1874453944 1681649719 388228515
671359508 847997583 161400465
0 2547368955 266781855
3203005759 3207454799 385245393
832759973 813886658 34110925
3671840261 3598096086 304246395
1377823717 351805081 65559824
1443383541 417364905 396521753
3985931590 3902342481 309035706
2954410911 2069878234 236077421
2262682459 2814150810 19476781
429946208 2305955655 241413300
3981482550 3203005759 4449040
3976086656 3592700192 5395894
1026018636 0 351805081
2282159240 1009398048 672251671
866870898 2833627591 159147738
3588251152 4211378187 83589109

fertilizer-to-water map:
2408925737 4214441342 80525954
1405678964 176700146 234310964
2103148712 0 114754845
289811242 411011110 140151672
19025194 1085844786 2698717
2799367899 3104502118 25793488
3686730518 3130295606 74796096
628099810 1273935988 19704264
1112219380 661022482 39037869
467844674 876473087 160255136
1356562401 1036728223 49116563
0 2195479548 19025194
1778400105 1801270370 10408868
3201290155 3772851666 86331548
1795901930 1713284032 67001953
1151257249 1907547144 205305152
4156855349 4121566803 26796069
160254887 2112852296 19696655
1788808973 114754845 7092957
1639989928 1780285985 20984385
3990635607 2938282376 166219742
2864292987 3316407580 74613579
3761526614 4148362872 26946870
2048296368 121847802 54852344
3054903767 3975180415 146386388
21723911 737942111 138530976
647804074 1811679238 95867906
3580274539 3391021159 106455979
2489451691 2628366168 309916208
1049288783 2132548951 62930597
4183651418 3205091702 111315878
3360834108 2408925737 219440431
743671980 1293640252 305616803
3287621703 3699639261 73212405
2938906566 3859183214 115997201
429962914 700060351 37881760
1775001290 2214504742 3398815
3788473484 3497477138 202162123
2825161387 4175309742 39131600
1660974313 1599257055 114026977
1862903883 1088543503 185392485
179951542 551162782 109859700

water-to-light map:
2182426048 2230942562 75597875
3871195410 3518497934 102047632
2258023923 1068663414 38503018
1376483871 1748535271 120845081
3267723970 3442138354 23090394
3687599530 4146748183 69194895
1687874044 1117179928 229976398
1024656105 0 9213951
2036492700 1503963494 145933348
1339060619 127856209 37423252
3600067928 3079307704 87531602
3778928632 3171781100 38997592
485121354 165279461 136305184
1266011073 1649896842 73049546
1917850442 9213951 118642258
2296526941 1107166432 10013496
3973243042 2847069441 75895604
3177917323 2989501057 89806647
136410837 1347156326 156807168
4049138646 3166839306 4941794
3817926224 3465228748 53269186
2631950731 2587548926 259520515
478937106 2064525337 6184248
3756794425 2922965045 22134207
621426538 487303746 242996590
3299937194 3372641330 69497024
1240422190 1722946388 25588883
1497328952 1873980245 190545092
2587548926 2945099252 44401805
864423128 2070709585 160232977
1033870056 1869380352 4599893
2891471246 3629668396 286446077
1158126414 866711173 82295776
4054080440 3210778692 62083789
1038469949 949006949 119656465
0 730300336 136410837
4116164229 3272862481 99778849
3290814364 3620545566 9122830
3369434218 3916114473 230633710
293218005 301584645 185719101

light-to-temperature map:
2047881931 2133163196 61773729
2768352591 2658645540 147815435
663892205 789634091 62723057
962320572 2260439344 13990400
4174477469 3668516433 35699725
3949428604 3175207948 225048865
2918680909 3704216158 118365848
976310972 47881164 71382440
736841121 215530327 84565848
1481137268 593966122 45807826
3406939292 2891251077 174249820
2916168026 3907788997 2512883
854252812 1457767969 108067760
2658645540 3065500897 109707051
0 1842095595 291067601
821406969 119263604 32845843
2182554848 897903253 121629531
1047693412 1565835729 166620621
3847603798 3400256813 16617815
1259860138 152109447 53195021
3581189112 3424865230 243651203
1470195525 1364566318 10941743
653613314 30205504 10278891
1430142037 1324512830 40053488
291067601 1773419829 68675766
2109655660 2194936925 65502419
1856219803 2274429744 191662128
359743367 300096175 293869947
3398948690 3416874628 7990602
3864221613 3822582006 85206991
2609164425 1750403060 23016769
1707010741 2466091872 49002444
1756013185 1732456350 17946710
1214314033 852357148 45546105
1313055159 2515094316 117086878
2175158079 40484395 7396769
3824840315 3910301880 22763483
1526945094 639773948 149860143
2304184379 1019532784 304980046
3037046757 3933065363 361901933
1773959895 1375508061 82259908
726615262 205304468 10225859
1676805237 0 30205504
4210177194 2806460975 84790102

temperature-to-humidity map:
2704404081 3383155981 190240562
3678765766 3078657339 304498642
2894644643 2740175301 304339717
3375985319 2704404081 35771220
3198984360 3840405770 142858638
3341842998 3044515018 34142321
3411756539 3573396543 267009227

humidity-to-location map:
3843755612 3461421206 53203349
3797023193 2837279508 46732419
1328442859 1820435049 165058603
2358032500 2069263501 110029787
776537476 129658954 20092280
2041812400 3514624555 271685061
4004991793 2179293288 239800545
1813819309 1372459189 171674343
890079235 609776153 19282104
129748073 629058257 107944033
2472688430 2519585064 209661612
1039983756 866750363 166281406
3896958961 2729246676 108032832
1004757286 149751234 35226470
850718450 90298169 39360785
3150066893 4231246292 63721004
0 737002290 129748073
909361339 1154885545 95395947
2682350042 2911243637 44822034
318816223 1586615093 233819956
796629756 531466240 54088694
705961941 460890705 70575535
3669300252 2884011927 27231710
4244792338 3015820858 50174958
3352063456 3786309616 317236796
552636179 0 75876832
3213787897 4103546412 55695414
294595004 585554934 24221219
2799176542 3110530855 350890351
1691965533 1033031769 121853776
2468062287 2956065671 4626143
2727172076 4159241826 72004466
1493501462 262426634 198464071
280173667 75876832 14421337
1986683356 2960691814 55129044
628513011 184977704 77448930
2313497461 3065995816 44535039
1206265162 1250281492 122177697
3269483311 1986683356 82580145
237692106 1544133532 42481561
3696531962 2419093833 100491231
";
}
