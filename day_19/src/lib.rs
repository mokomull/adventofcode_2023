use std::ops::RangeInclusive;

use prelude::*;

use range_set::RangeSet;

#[derive(Debug)]
enum Category {
    X,
    M,
    A,
    S,
}
use Category::*;

impl From<&str> for Category {
    fn from(value: &str) -> Self {
        match value {
            "x" => X,
            "m" => M,
            "a" => A,
            "s" => S,
            _ => panic!("invalid category {value:?}"),
        }
    }
}

#[derive(Debug)]
enum Criterion {
    Unconditional,
    Gt(Category, u64),
    Lt(Category, u64),
}
use Criterion::*;

#[derive(Debug)]
enum Disposition {
    Next(String),
    Accept,
    Reject,
}
use Disposition::*;

impl From<&str> for Disposition {
    fn from(value: &str) -> Self {
        match value {
            "A" => Accept,
            "R" => Reject,
            _ => Next(value.to_owned()),
        }
    }
}

type Rule = (Criterion, Disposition);

#[derive(Clone, Debug)]
struct Fourple<T> {
    x: T,
    m: T,
    a: T,
    s: T,
}

impl<T: Clone> Fourple<T> {
    fn get(&self, category: &Category) -> &T {
        match category {
            X => &self.x,
            M => &self.m,
            A => &self.a,
            S => &self.s,
        }
    }

    fn get_mut(&mut self, category: &Category) -> &mut T {
        match category {
            X => &mut self.x,
            M => &mut self.m,
            A => &mut self.a,
            S => &mut self.s,
        }
    }
}

type Rating = Fourple<u64>;
type AttributeRange = RangeSet<[RangeInclusive<u64>; 10]>;

pub struct Solution {
    rules: HashMap<String, Vec<Rule>>,
    ratings: Vec<Rating>,
}

lazy_static::lazy_static! {
    static ref RATING: Regex = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").unwrap();
}

impl Day for Solution {
    fn new(input: &str) -> Self {
        let (top, bottom) = input
            .split_once("\n\n")
            .expect("there must be a blank line in your input");

        let rules = top
            .lines()
            .map(|line| {
                let (name, rest) = line.split_once('{').expect("could not find a {");
                let rest = rest.strip_suffix('}').expect("could not find a }");
                let rules = rest
                    .split(',')
                    .map(|rule| -> Rule {
                        if let Some((l, r)) = rule.split_once(':') {
                            if let Some((category, count)) = l.split_once('<') {
                                (
                                    Lt(category.into(), count.parse().expect("bad integer")),
                                    r.into(),
                                )
                            } else if let Some((category, count)) = l.split_once('>') {
                                (
                                    Gt(category.into(), count.parse().expect("bad integer")),
                                    r.into(),
                                )
                            } else {
                                panic!("input was neither > nor <");
                            }
                        } else {
                            (Unconditional, rule.into())
                        }
                    })
                    .collect();

                (name.to_owned(), rules)
            })
            .collect();

        let ratings = bottom
            .lines()
            .map(|line| {
                let captures = RATING.captures(line).expect("ratings don't match regex");
                let x = captures
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("bad integer");
                let m = captures
                    .get(2)
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("bad integer");
                let a = captures
                    .get(3)
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("bad integer");
                let s = captures
                    .get(4)
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("bad integer");

                Rating { x, m, a, s }
            })
            .collect();

        Solution { rules, ratings }
    }

    fn part1(&self) -> anyhow::Result<u64> {
        Ok(self
            .ratings
            .iter()
            .filter_map(|r| match self.is_accepted(r) {
                Err(e) => Some(Err(e)),
                Ok(false) => {
                    log::debug!("rejected {r:?}");
                    None
                }
                Ok(true) => {
                    log::debug!("accepted {r:?}");
                    Some(Ok(r.x + r.m + r.a + r.s))
                }
            })
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .sum())
    }

    fn part2(&self) -> anyhow::Result<u64> {
        self.count_accepted(
            "in",
            Fourple {
                x: RangeSet::from_ranges(&[1..=4000]),
                m: RangeSet::from_ranges(&[1..=4000]),
                a: RangeSet::from_ranges(&[1..=4000]),
                s: RangeSet::from_ranges(&[1..=4000]),
            },
        )
    }
}

impl Solution {
    fn is_accepted(&self, rating: &Rating) -> anyhow::Result<bool> {
        let mut wf_name = "in";
        log::debug!("evaluating {rating:?}");

        loop {
            log::debug!("looking at workflow {wf_name:?}");
            let workflow = self
                .rules
                .get(wf_name)
                .ok_or_else(|| anyhow::anyhow!("could not find workflow {wf_name:?}"))?;
            log::debug!("workflow is {workflow:?}");

            let mut chosen = None;
            for (criterion, disposition) in workflow {
                match criterion {
                    Unconditional => {
                        chosen = Some(disposition);
                        break;
                    }
                    Gt(c, target) => {
                        if rating.get(c) > target {
                            chosen = Some(disposition);
                            break;
                        }
                    }
                    Lt(c, target) => {
                        if rating.get(c) < target {
                            chosen = Some(disposition);
                            break;
                        }
                    }
                }
            }

            match chosen {
                Some(Accept) => return Ok(true),
                Some(Reject) => return Ok(false),
                Some(Next(name)) => wf_name = name,
                None => anyhow::bail!("none of the rules matched"),
            }
        }
    }

    fn count_accepted(
        &self,
        state: &str,
        mut possibilities: Fourple<AttributeRange>,
    ) -> anyhow::Result<u64> {
        let mut res = 0;

        let next = self
            .rules
            .get(state)
            .ok_or_else(|| anyhow::anyhow!("could not find state {state:?}"))?;

        for (criterion, disposition) in next {
            if [
                &possibilities.x,
                &possibilities.m,
                &possibilities.a,
                &possibilities.s,
            ]
            .into_iter()
            .any(|r| r.is_empty())
            {
                // no amount of set-intersections is ever going to bring this back...
                break;
            }

            let mut recur_possibilities = possibilities.clone();

            match criterion {
                Lt(cat, target) => {
                    let recur_ranges = recur_possibilities.get_mut(cat);
                    let continue_ranges = possibilities.get_mut(cat);
                    let inverse = shrink_range(recur_ranges, 0..=(target - 1));
                    *continue_ranges = inverse;
                }
                Gt(cat, target) => {
                    let recur_ranges = recur_possibilities.get_mut(cat);
                    let continue_ranges = possibilities.get_mut(cat);
                    let inverse = shrink_range(recur_ranges, (target + 1)..=u64::MAX);
                    *continue_ranges = inverse;
                }
                Unconditional => {
                    // we will recur on everything that's possible so far, and then we're done.  Set
                    // one (any one!) of the attribute ranges to a completely empty one.
                    let mut empty = AttributeRange::new();
                    std::mem::swap(&mut possibilities.x, &mut empty);
                }
            }

            match disposition {
                Accept => {
                    res += count(&recur_possibilities);
                }
                Reject => {
                    // do nothing, but let the shrunken range continue down the list of rules
                }
                Next(s) => {
                    res += self.count_accepted(&s, recur_possibilities)?;
                }
            }
        }

        Ok(res)
    }
}

// Intersects ranges with intersect_with, and returns ranges & !intersect_with
fn shrink_range(
    ranges: &mut AttributeRange,
    intersect_with: RangeInclusive<u64>,
) -> AttributeRange {
    // since there's no intersect() function in RangeSet, but doing inserts and removals will *return* the ranges
    let intersected = ranges.remove_range(intersect_with);
    if let Some(mut r) = intersected {
        // put the intersection where we expected, and return the inverse of the intersection
        std::mem::swap(&mut r, ranges);
        return r;
    } else {
        // there was no intersection, so ranges becomes empty and everything that *was* in ranges should be returned
        let mut empty = AttributeRange::new();
        std::mem::swap(&mut empty, ranges);
        return empty; // which is not empty anymore
    }
}

fn count(ranges: &Fourple<AttributeRange>) -> u64 {
    [&ranges.x, &ranges.m, &ranges.a, &ranges.s]
        .into_iter()
        .map(|r| {
            r.as_ref()
                .iter()
                .map(|std_range| std_range.end() - std_range.start() + 1)
                .sum::<u64>()
        })
        .product()
}
