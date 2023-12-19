use prelude::*;

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

enum Criterion {
    Unconditional,
    Gt(Category, u64),
    Lt(Category, u64),
}
use Criterion::*;

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

#[derive(Debug)]
struct Rating {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Rating {
    fn get(&self, category: &Category) -> u64 {
        match category {
            X => self.x,
            M => self.m,
            A => self.a,
            S => self.a,
        }
    }
}

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
                let rest = rest.strip_suffix("}").expect("could not find a }");
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
        anyhow::bail!("unimplemented");
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

            let mut chosen = None;
            for (criterion, disposition) in workflow {
                match criterion {
                    Unconditional => {
                        chosen = Some(disposition);
                        break;
                    }
                    Gt(c, target) => {
                        if rating.get(c) > *target {
                            chosen = Some(disposition);
                            break;
                        }
                    }
                    Lt(c, target) => {
                        if rating.get(c) < *target {
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
}
