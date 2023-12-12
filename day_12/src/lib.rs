use prelude::*;

#[derive(Clone, Debug, PartialEq)]
enum Spring {
    Good,
    Damaged,
    Unknown,
}

use Spring::*;

impl Spring {
    fn parse(input: &str) -> Vec<Spring> {
        input
            .bytes()
            .map(|b| match b {
                b'.' => Good,
                b'#' => Damaged,
                b'?' => Unknown,
                x => panic!("unexpected spring {x:?}"),
            })
            .collect_vec()
    }
}

fn could_possibly_fit(springs: &[Spring], counts: &[u64]) -> bool {
    if springs.is_empty() && !counts.is_empty() {
        return false;
    }

    if counts.is_empty() {
        return springs.iter().all(|s| s != &Damaged);
    }

    let mut counting = false;
    let mut count = 0;

    for (i, spring) in springs.iter().enumerate() {
        match spring {
            Good if counting => {
                // we've found the first sequence of damaged springs
                if count != counts[0] {
                    // we definitely have a mismatch!
                    return false;
                }

                return could_possibly_fit(&springs[(i + 1)..], &counts[1..]);
            }
            Good => (),
            Damaged => {
                counting = true;
                count += 1;
            }
            // it's not *in*consistent yet, so it could *possibly* fit.
            Unknown => return true,
        }
    }

    // we got all the way to the end of springs, so make sure we counted exactly the right number of
    // damaged springs
    count == counts[0] && counts.len() == 1
}

fn count_options(springs: &[Spring], counts: &[u64]) -> u64 {
    log::debug!("count_options: {springs:?} with counts {counts:?}");

    if !could_possibly_fit(springs, counts) {
        log::debug!("ruled it out");
        return 0;
    }

    let first_unknown = springs.iter().position(|s| s == &Unknown);

    if let Some(i) = first_unknown {
        let mut count = 0;

        // try it with a Good spring
        let mut next = springs.to_vec();
        next[i] = Good;
        count += count_options(&next, counts);

        // and try it with a Bad spring
        next[i] = Damaged;
        count += count_options(&next, counts);

        return count;
    } else {
        // we had no unknowns, and we didn't rule it out, so this is the exactly-one way to arrange
        return 1;
    }
}

pub struct Solution(Vec<(Vec<Spring>, Vec<u64>)>);

impl Day for Solution {
    fn new(input: &str) -> Self {
        Solution(
            input
                .lines()
                .map(|line| {
                    let (springs, counts) = line.split_once(" ").expect("missing space");
                    let counts = counts
                        .split(',')
                        .map(|x| x.parse().expect("not an integer"))
                        .collect_vec();
                    (Spring::parse(springs), counts)
                })
                .collect(),
        )
    }

    fn part1(&self) -> anyhow::Result<u64> {
        Ok(self
            .0
            .iter()
            .map(|(springs, counts)| count_options(springs, counts))
            .sum())
    }

    fn part2(&self) -> anyhow::Result<u64> {
        Ok(self
            .0
            .iter()
            .map(|(springs, counts)| {
                let mut unfolded_springs = springs.clone();
                let mut unfolded_counts = counts.to_vec();
                for _ in 0..4 {
                    // add four more copies.
                    unfolded_springs.push(Unknown);
                    unfolded_springs.extend_from_slice(springs);
                    unfolded_counts.extend_from_slice(counts);
                }
                count_options(&unfolded_springs, &unfolded_counts)
            })
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_counts() {
        let _ = env_logger::try_init();

        fn do_line(line: &str) -> u64 {
            let solution = Solution::new(line);
            let (springs, counts) = solution.0.into_iter().next().unwrap();
            count_options(&springs, &counts)
        }

        assert_eq!(1, do_line("#.#.### 1,1,3"));
        assert_eq!(1, do_line("???.### 1,1,3"));
    }

    #[test]
    fn example() {
        let solution = Solution::new(EXAMPLE);
        assert_eq!(21, solution.part1().unwrap());
        assert_eq!(525152, solution.part2().unwrap());
    }

    static EXAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
}
