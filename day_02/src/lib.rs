use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space1, u64},
    multi::separated_list1,
    IResult,
};
use prelude::*;

struct Game {
    id: u64,
    grabs: Vec<Handful>,
}

#[derive(Debug, Default, PartialEq)]
struct Handful {
    red: u64,
    green: u64,
    blue: u64,
}

impl Handful {
    fn parse(input: &str) -> IResult<&str, Handful> {
        fn parse_single(input: &str) -> IResult<&str, Handful> {
            let (input, count) = u64(input)?;
            let (input, _) = space1(input)?;
            let (input, res) = alt((
                |input| -> IResult<&str, Handful> {
                    let (input, _) = tag("red")(input)?;
                    Ok((
                        input,
                        Handful {
                            red: count,
                            green: 0,
                            blue: 0,
                        },
                    ))
                },
                |input| -> IResult<&str, Handful> {
                    let (input, _) = tag("green")(input)?;
                    Ok((
                        input,
                        Handful {
                            red: 0,
                            green: count,
                            blue: 0,
                        },
                    ))
                },
                |input| -> IResult<&str, Handful> {
                    let (input, _) = tag("blue")(input)?;
                    Ok((
                        input,
                        Handful {
                            red: 0,
                            green: 0,
                            blue: count,
                        },
                    ))
                },
            ))(input)?;

            Ok((input, res))
        }

        let (input, singles) = separated_list1(tag(", "), parse_single)(input)?;
        Ok((
            input,
            singles
                .iter()
                .fold(Handful::default(), |one, other| Handful {
                    red: one.red + other.red,
                    green: one.green + other.green,
                    blue: one.blue + other.blue,
                }),
        ))
    }
}

pub struct Solution {}

impl Solution {
    pub fn new(input: &str) -> Solution {
        Solution {}
    }

    pub fn part1(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented");
    }

    pub fn part2(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented");
    }
}

#[cfg(test)]
mod test {
    use crate::Handful;

    #[test]
    fn parse_handful() {
        assert_eq!(
            Handful::parse("3 blue, 4 red").unwrap().1,
            Handful {
                red: 4,
                blue: 3,
                green: 0
            }
        );
    }
}
