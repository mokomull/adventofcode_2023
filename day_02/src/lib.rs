use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space1, u64},
    multi::separated_list1,
    IResult,
};
use prelude::*;

#[derive(Debug, PartialEq)]
struct Game {
    id: u64,
    grabs: Vec<Handful>,
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Game> {
        let (input, _) = tag("Game ")(input)?;
        let (input, id) = u64(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, grabs) = separated_list1(tag("; "), Handful::parse)(input)?;
        Ok((input, Game { id, grabs }))
    }
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

pub struct Solution {
    games: Vec<Game>,
}

impl Solution {
    pub fn new(input: &str) -> Solution {
        let games = input
            .lines()
            .map(|line| Game::parse(line).unwrap().1)
            .collect();
        Solution { games }
    }

    pub fn part1(&self) -> anyhow::Result<u64> {
        let res =
            self.games
                .iter()
                .filter_map(|game| {
                    if game.grabs.iter().all(|handful| {
                        handful.red <= 12 && handful.green <= 13 && handful.blue <= 14
                    }) {
                        Some(game.id)
                    } else {
                        None
                    }
                })
                .sum();
        Ok(res)
    }

    pub fn part2(&self) -> anyhow::Result<u64> {
        anyhow::bail!("unimplemented");
    }
}

#[cfg(test)]
mod test {
    use super::*;

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

    #[test]
    fn parse_game() {
        assert_eq!(
            Game::parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
                .unwrap()
                .1,
            Game {
                id: 1,
                grabs: vec![
                    Handful {
                        red: 4,
                        green: 0,
                        blue: 3
                    },
                    Handful {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    Handful {
                        red: 0,
                        green: 2,
                        blue: 0
                    },
                ]
            }
        )
    }
}
