use prelude::*;

use rayon::prelude::*;

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
            .par_iter()
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

    #[test]
    fn personal_input() {
        let solution = Solution::new(INPUT);
        assert_eq!(8270, solution.part1().unwrap());
        assert_eq!(3, solution.part2().unwrap());
    }

    static EXAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    static INPUT: &str = ".#??..#??.?. 3,1,1,1
???.##??#??????#???# 1,13,1
.??#.?#??#????#?? 2,4,1,1,1
???#??#.???#.?#??#?# 4,2,1,2,1,4
????#?##???#.??.? 2,2,6,1
.?????#???.###.##? 7,3,2
.###?..#??????#???? 4,8,1
??????????????? 1,1,1,2,1
???.????????#? 1,3,1,1
???.#..?#.?#?#??#?. 2,1,1,5,1
.?..??#.????. 1,3,1,2
.#?.??#?????##.# 1,2,1,3,1
?###????.#?? 5,1,2
?#??.????? 2,4
?.#????#?????#??#?? 1,6,1,4,1
#????????#?#?? 1,7,1,1
?#??.?????? 3,1,1
?..??#?###?? 1,6
???.?.??#??##?? 1,1,1,5,1
#???#.?????? 2,1,3,1
?###??.??.?#???..? 4,1,2,1,1,1
?.#???#?.?? 2,1,1
#??##????##??????#? 7,5,2
????#????????????# 1,6,1,1,2
#?.?.##???? 1,1,5
.?????.?#???????? 1,2,2,2,1
#??.???#?.????? 2,5,1,1
?????????.#???. 6,2
?????????? 1,1,4
?#??#??#?## 1,7
????#?##???#? 1,6,1,1
??#??#.???#?????? 1,1,1,7,1
?????##?.?#??. 1,2,4
???##????###????. 4,7
?.?.?.?#???? 1,1,6
???..??.????.###??. 2,2,2,5
????##??#????? 1,7,1,1
???..????#??? 2,6
.??.????????. 1,2
?.#?.????? 1,2
??????.#???#? 2,5
?????#.###???#.???# 6,7,1,1
##.????##..#??#?. 2,6,1,1
?????.??.?????????. 1,6
??????..#?# 4,3
#??????#??..?#? 10,2
?.?????.?#??? 1,2
???#??????.. 2,4
.#?#???####??.?#?##? 12,5
?.?????##??? 3,4
?###?###????????# 10,3
.?????????#? 1,5
#?##????#???? 1,2,2,1
.??????.??#???. 1,2,1,1,1
??#.?#.????# 1,2,1,1
???????.?#??????.?? 2,1,1,8,1
???#??.?#???? 5,2,2
?#??????###???. 2,8
.??????#???????#? 1,12
??..???#?#??? 1,6
?.??????.. 1,1,3
??#??.?#??#. 1,1,1,1
?#?????.??????. 1,1,6
.#?#?????#?#???#?.?? 5,9
???#???#.#?##????? 5,1,7,1
?#.??#??#?..?? 1,5,1
#???#????#?.? 1,1,6,1
????????.#? 4,1
????#?#?#??.??.?? 8,1
???#?????#?#?..#? 4,1,4,1
??.??????#??.?#??#. 1,3,1,1
.????????????? 1,2,2,1
#.?#?#?#?.??? 1,6,1
?#??#??..?#?? 3,2,3
..#?.?#??#?# 2,6
???#?.?..???## 3,1,2
?????.?#?.???## 3,3,3
??.?????..????? 3,5
?.?????##? 1,1,2
?.??#.#????#???.. 1,3,1,1,4
??#??#?#?##????.# 3,3,6,1
?.?.?.?####???.#. 1,8,1
?##?????#?.?#.. 3,1,1,2,1
???#???????.#. 2,4,1
????.??.?#???? 1,3
.?#?#?#??????##??? 2,11,2
??.?#?.#?. 2,2,2
????#...?#???# 4,5
.??..???#??#?#? 2,1,1,3
????#.?????? 3,6
????.?????? 1,1,2
????#?????.?# 1,2,2,2
.?#?????.??.? 3,1,1,1
???#??.?#.??. 1,3,2,2
?##???????#???????# 2,9,1
?##?????????.?? 8,1
.#???????.??#???? 1,4,4
.?#?.#??.? 1,1,1
??.?#??????#? 4,3
??#?#??#??.?? 3,2,1
.#?.???##? 1,5
?????.#?.???#? 2,1,1,4
##????#??? 4,1,1
?????????# 1,2,3
??#?#?????##? 4,3
?#.???##?#????#?##. 1,12
#.?##???#???..??.?? 1,6,1,2,1
?#????#??#.#?.##?# 5,1,1,1,4
?#????#.#??#..? 1,3,1,1,1
??###?.?.????. 5,2
.????????#.#? 5,2,1
.?.#.???..??#.?? 1,1,1,3,1
#????????.??????. 4,4,2,1,1
?.??????#? 3,3
...?#.#.???##? 2,1,1,2
??????????#.???.?#?? 7,1,1,1
.????##?#??.???#???? 7,5
??.#?.???? 1,2,2
?.#??#??#?? 1,4,2
????#?????? 1,4,2
??.???????? 1,1,2
??#????????#??#???? 3,3,7
?????????##?.??#?. 1,1,2,1,3,2
??#????????????.??? 2,11
??#?.?#???#?##????. 4,3,4,1
????.???????.?? 1,3
????#.##?? 1,1,4
#?????#?????.##?## 2,7,5
#??.??..?#?#.? 2,1,4
.??.??.?.?????##? 1,1,1,4
??.???????#??? 1,7,1
.#.??.?##???#.? 1,1,3,2,1
#??.??##??.?#?.#?#?? 3,4,1,4
???..?????????.# 2,1,3,1,1
??#.?????#?##.?#.# 2,6,2,1,1
?#?.?.#??#?? 1,6
?###????#?.? 6,2
?.?????????????.?? 1,1,1,1,4,1
????.?#?##??#??. 1,1,5,2
???????.????# 1,1,1,3
##???.????#???# 2,1,1,1,5
.#?##???####?.??#.?? 4,5,2
????????#?#?.?#?? 1,1,3,1,3
?.?????#?. 2,2
?#???????????#.????? 7,2,1,1,1
????????#..##. 1,2,2,2
#?#?##?#??. 3,5
???#????###????????. 17,1
.????#???## 1,3,4
??####??#??#?#?#?#?? 6,6,4
??????#????.?.?? 1,3,1,1,1
..?##?.?.?. 3,1
.???####?????#? 8,1
.???????#?..??.?# 1,3,1,2,1
??.?#?????? 1,2,2
??#???#..?#?????#??? 4,1,5,1,2
??#?...#?? 4,1
??????.#?#?##??.?? 2,7
.???????##???. 1,9
?..#.????#??#???#?? 1,3,9
?#??????????#???.#?? 4,3,1,1,3
.?????#???.??#?.?. 4,1
??#.?##?#??##??#?#? 1,1,12,1
?##??#??##???#.?.. 2,9
#??????#??? 3,4
##???.?#..#.?# 5,2,1,2
???.?.???# 2,1,3
?.?????.?? 2,2
???.?????#?.?????? 1,2,2
.#????#????.? 1,6
????.#.?#??????#???? 1,1,1,1,2,7
?.???.#.??##???#?? 1,1,1,1,8
??????.?#? 4,2
#??..?????#? 2,1,2,1
?##.#.????##?# 2,1,2,4
????????#???. 9,1
??..????.? 1,4
????#???#?.? 9,1
#???.??????? 3,1,2,1
.??????.?##??#. 6,3,1
#.?.?#????#??#? 1,2,1,2,2
##??##?.??? 6,1
?????.????#???? 1,1,6
#??.????????????#?#. 2,1,1,4,3
???#??..?????#?## 5,3,2
?##??????.??.? 4,4,1
???#?????#?.? 2,1,3
.?#?#????#?#?#? 5,6
?..?#??###???? 1,7
??#.???##.#? 2,4,1
#??????#.?##?#?##??? 1,2,3,8,1
?.?????????#?.? 2,6
??.????##??? 1,3,2,2
?#?#??#???#?#??? 10,4
#.?.???#.#??#?? 1,1,1,1,6
??##??.??? 3,1
?????.??#..?.#? 1,1,3,1,2
####???#????? 8,1,1
?###???#?.. 4,1,1
????.?????#?.???? 1,1,1,5,2
#????#.??? 4,1,1
??#??????.??#?. 7,1,1
??#?????##??? 1,8
..?#.?#???????. 2,2,2,1
??.???#???.? 1,1,2,1
.?.#?.?.??? 2,1,1
????##??.????##?##?? 5,9
?#???????..?? 2,3,2
?#???????.??#????#?? 1,1,1,1,8
#.?????#?.? 1,2,1
#?.??????????????? 1,1,1,4,1,1
??.#???#????#? 1,2,4,3
.#???.???.#???#.? 4,1,1,2,1
??????#???#?# 1,2,1,1
????##??###??????.?? 1,8,4,1
??#?????#?????????? 9,1,3
.##???.???????? 2,1,1,1
?#.???.??#????# 2,1,1,7
.?..?..??? 1,1,2
???.????.??#?? 3,2
.?#.???#????????? 2,6
????.?#?????????# 1,6,1,1,1
???#?.?#???????? 1,2,4,2,1
??##???#??#? 1,2,3,2
#?.?.#.???? 1,1,3
?#??#.?????? 1,2,4
??????.##?????.? 1,4,1
#????.???.#.???????. 1,1,3,1,3,1
.#?##?..?##?.??.?? 4,2
.#?###??#???#?? 9,3
#??.##?.?.?#????#. 3,3,1,3,1,1
?????##????? 3,2,1
??#??????#????. 3,7
????#.?#?? 1,1,1
.??.#?##?.?? 1,5
?#??.#..?????#??#??? 2,1,1,1,6,1
?#?.??#???#??? 1,3,3
????.?#.?????????? 4,1,1,1,1,1
?.#.??????.?.##. 1,1,6,1,2
????????#???#???? 1,5,3,1
.?.?#??.?#?? 1,3,2
??????????#??####?? 1,1,4,8
?#???..????# 1,1,4
.??###?.#..#???.?.? 5,1,1
??.?.?????.???????.. 1,3,4
..??????.???. 4,1
#????????####??????# 1,11,2
?#?.?.????? 1,1,2
.?.?#??###?????.?#?? 1,2,7,1,1,1
???#?.#???#????..? 1,3,2,6,1
?..???????.? 1,1,4,1
.??????.#?????????? 1,1,1,1,1,4
?.?#??#?#???? 1,5
???.????#????#### 1,1,11
.??#.?????#??#??? 1,1,2,6,1
?#.?????.#?? 2,1,2,3
???.???#.#. 2,3,1
?????.????.# 3,1,1
.?????..?????# 4,1,2
.??##??#?#?????#. 6,1,3
??#?#???.??????#### 5,1,7
???#??#??#??.? 1,1,6
??????.???? 1,1,3
???###?###?.#??#? 1,9,2,1
##??.###..??? 4,3,1
??.??.?????#? 1,1,4
?#????????? 2,1,2
#?.?????.#?.??#?. 1,2,2,1,2
?#???..#????????#? 4,11
?????????####?? 1,1,1,5
??.?????.?#??????. 3,3,2
????.?#?#????#??.?? 2,11
##.#.?#.???.?.? 2,1,2,1,1
??.??????.????? 2,2,1,1,3
#?#???.#??#? 1,2,5
??#?.????? 3,2,2
??.???#??. 1,2
????#??#?. 1,1,1
#?.???.????? 1,1,1,5
???????.#?#??#?#??? 2,1,1,1,1,4
??#????##.? 5,2
??#???#?..#???. 6,1,1
?.#?????#?.. 1,3
.??.??#????..???#? 2,5
?#?#????#?????#? 3,6,1
?###????.????##. 4,1,3
?.???????# 1,2,1
.??.??????#?..?## 1,6,2
#???#??#????..???.?? 9,1,3
#.???????????.???? 1,1,1,3,1,1
..??##??????##????# 3,11
?????.????????.?? 4,2,2,1
#..??.?#?? 1,1,2
????.???##???.#???? 1,1,1,6,1,2
??#?#??#??.???????# 1,8,2,1,1
??#.???#.?? 2,1
???#?#?.#????????##? 3,2,8
???#?.????#????? 4,6,2
??.??..????? 1,2,1,1
?.??#?#?????. 7,1
?.????#??#??#?#? 2,7
.?????????????#???? 3,1,4,1,1
????.????.? 1,3
??#..?????? 3,1,1
.?#...??????. 2,1,3
?????????????# 2,1,8
?????????? 1,1,5
??#?##??#?#?#?????? 6,2,1,1,1,1
.???#?????#. 4,1,1
?#.???#????###?#? 1,1,3,6
????#?#?#???.?.#?? 1,4,1,1,1,1
?#???###.??#?##? 2,1,3,6
?????????.#? 6,1
?????#???#????..# 4,1,4,2,1
.?.????.?????????? 1,2,7
#???????????? 1,1,2,5
???????????#.??.? 5,4
???##?#?#####???? 13,1
??.?..???.?? 1,1
???#?#?????? 8,1
?.??#?.#??? 2,3
#.?.??..#? 1,1,2
????????.??#???? 2,1,1,1,4
??#.??????????????? 1,3
.??##?.??.???##??#?. 4,8
?.???#????..???? 4,1
?#??????#?#. 1,3,1,1
#??####????.??? 11,2
.?#??????????????? 2,3,6
?#???#?#????#?.? 8,1,2
??.????.?.??#?#? 2,1,1,1,5
???#??.????#.. 4,2,2
???.????????. 3,1,2,1
#?#??.??#.????? 1,2,3,1,3
????#.???#?. 1,1,1,5
?????#???.?????. 8,2
??##??##??##??##? 11,4
??#.????#?? 2,5
??????????? 2,3
?????.#..? 1,1,1
.???#????#?????? 10,1
#???#?#???#????#???? 1,15,1
??????????##? 1,1,1,5
???.??##???.#..??#. 4,1,1
.?????.??##.? 1,1,1,4
??????##???..???#?? 3,5,1,2,1
.?#?#????.?.#?. 6,1,1,1
?????#?#?.#?? 1,4,1
???.??##?.?????? 3,5,1,1,1
?.????.????? 3,2,1
..??.?..#??##????#? 2,10
???..?????#?.??????? 1,2,5
?#???????????#?.? 9,2
..?##????##..# 2,1,2,1
???#??????#?.# 9,1,1
#?#???.??? 5,2
???..???.?.??????. 1,2,3,1
.#???.?.#??#?..?. 3,5
?#?#??###?. 4,4
????.?#???#? 2,3,2
?#?.#?.?..#? 3,1,1,1
???.??????.#??? 2,4,3
.##??????##?#????.#? 10,2,2
..??##??..??. 4,1
#.##????....##.??? 1,2,3,2,3
?.???#.?#.????? 2,2,4
???#???.??###? 3,6
?#..????..?##??.? 2,3,4
??#.???????????. 1,1,4,2,1
.##?????#???##?.???? 5,7,1
??.?????#???#?#?? 2,1,1,9
#?.??.?#.? 1,1,1
???..??????#?? 1,2,4
.#?#?#????.??#?.##?? 8,1,3
??????????.?? 6,1
?#????.?#.#??#?#.. 4,2,6
.????????.??.?? 4,1
???.#?#???????#... 6,1
.????.????? 1,5
?????????#. 1,5
.?..??##?.##?. 3,3
??.#?#?#?#????#???? 1,1,3,7,1
..??#?#???#??. 5,3
?..##????????? 1,5,1,1
?#?????#?? 1,4
#.???#??????#?. 1,4,5
?????##????.?.?? 1,8,1
???###?#??.??.?# 1,7,1,2
?#.??#??#????? 1,5
?#.???#????.???#?? 2,1,3,1,3
??#?.???##.??.????.? 4,2,3,1
??.??????#????#? 1,8
?#???.?????#?? 3,2,2
.#???????? 2,2
??.##.????????????? 2,2,3,1
#???????#?. 5,1
##.??.#?.??#?? 2,1,2,2
??????##??. 1,4
??.??##????? 1,7,1
???.?#??#??? 1,5
?#???.?????##?#?#?.# 3,3,2,3,1
?#?.????#? 2,1,1
.?#?#??.??#???? 4,6
?#..?######???? 1,10
??##??????? 2,5
???.????##..?..????? 2,2
??...##.????? 2,2,1,1
?#????#..#????#???? 6,10
.?##?????#??# 2,7
?????????#??#?#??#?? 2,10
.??##????#??.???.#?? 11,1,1,1
?#???.????# 2,5
???#?.??..???##???.? 5,1,2,2,2
???.??????????#? 1,1,1,4,2
????###????.??? 8,2
???.?#.??#??.??????. 1,3,2
??#???.#????#?? 2,2,3
#?#???#??#????.# 3,9,1
.??#????????#.?? 1,1,1,4,1
??#.????????#???#??? 1,3,1,3,1,1
??.?#??##?#???#??## 1,15
?????.?????#???#.??? 5,8,3
.?##????..?#?. 6,2
#????.??##????.# 1,1,6,1
?##???#??.??#. 3,4,1,1
???#.???#?.??????#.? 3,4,1,1,3
????????.# 7,1
?#???###?? 1,4,1
??.??#????#? 1,1,7
?#??????#..??.. 2,5,2
.??.?#????#?#.. 1,9
???..???#.??? 2,4
#??##??#.??? 1,2,2,2
..???.????? 2,1,2
???????.????##.##.? 4,1,4,2
??.??.?#.. 1,2,1
??#??#????##? 1,1,6
???##?###???# 1,8,1
?#????????.?.#?? 3,1,1,3
.?#?#??#?#.??.?? 9,1
.?...???.????# 1,1,5
...#?.#???#?#????? 1,8
#...?????#??#?#???#? 1,1,1,1,8
.??.??#?????????? 1,12
#????.?###?##???. 5,7,1
?#?.?#??????? 2,6
?#?#??????? 6,1
..???..?#??????.#? 3,3,2,1
????.???#?? 1,2,2
???##?##???#????? 5,3
.???..#?#? 1,1,1
?????#???#? 5,4
#?.?##?#???#??#??.# 2,2,1,7,1
?#?????.??#??#.?. 6,4
???#.???.????.?? 3,2,2,1,2
.??#?#??#??? 1,1,1,3
?.??#.?.??? 3,2
.?.#??????#????##?## 1,1,2,1,1,7
.????#???#???? 2,6,3
??##????????## 2,1,1,3
.????..??. 3,1
???#.?????###?.??#?? 2,1,1,3,4
?..???..?.????????#? 1,2,1,7,2
????????#?#? 1,5,3
.????????????? 6,2
???????##?#????.?#?? 1,5,2,1,1,1
??#?#.????#?? 5,2,1,1
??..#?????????. 1,4
??#.#???#???#? 2,7,2
.?.?.?????##?.?? 1,1,1,4,1
.?#???.?#. 4,1
??????#?.#??.?? 2,2,1,1,1
.??#?.?#??? 3,2,1
?##????##.??.??? 8,1,1
???#..??.##??????#? 2,1,1,9
.#???#??#? 2,5
???#.???#???##???? 2,1,5,2,1
?..#?????#????. 1,4
.??????#??. 1,1,1
?.??.?????#??#? 1,1,2,4
???.?#.#??????.??? 3,1,2,2,1,2
?????##?#??#????#.?? 1,13
..#????????.?? 2,4
?#??.?#?.?#??. 2,2,2
???##.????##? 4,7
?##?#?###?#.???.#??? 11,2,2
?.???????#? 1,5
??.?.???## 1,1,2
..#??#?#??##???#???# 1,4,9,1
?#??#????????## 3,1,1,5
.#?.??#????????# 1,8,1
#??????#???#?. 1,1,3,1
.?.???.?#??## 1,6
#?#?????#?#??#?..??? 1,1,3,1,3,1
##??#???.?#?? 7,1
..??#????.??##?#???. 5,8
??..#??????? 1,1,1,2
?????##?#.? 6,1,1
???#???#?#?.# 4,1,1,1
?.#???#.?#?#??.? 3,1,4
?.?#??.???.#..? 2,1,1,1,1
???#?#????.#????#? 1,2,5,1,1,1
.#.??##??#?## 1,9
???.????.???.? 4,3
.???#???##.?#? 1,3,2,2
...#??#?#?# 1,5
???#?#???.. 1,3
#?#???#??###. 1,5,4
#?????????##?????## 1,1,1,1,6,3
?.?#????#????###.? 8,3
.??#???????????? 6,1,3
.?...#.??#.#. 1,1,1,1
.#????????? 4,2
?????????##?.????? 8,3
?????#?.?.???.??? 2,1,3,2
.???#????#### 1,1,1,4
.?.#??#??##?##???.?? 1,1,9,1,2
?..#???#?#?.???.??. 1,8,1,1
??.??.????????? 1,1,4,1,1
.???#.#??.#.? 4,3,1
???#???.??? 1,2,1
?#???????..??#?#???? 5,7
??#???????#?#????#? 1,6,5,3
#???????????#.###? 1,2,1,1,2,4
?#???#?.#??.?# 6,1,2
#??????????.#?##?? 6,2,4
???????.?#????? 2,1,1,2,4
??#?##?#..?????#. 2,4,6
??????.??#??#? 3,4
.??...?????.? 1,2
??#????##????????# 1,1,8,1,1
.?#?????.? 1,1
???#?#????.???? 4,1,1
?.???.##?????# 1,8
??.#??#.???? 1,4,3
?.?#?????#. 1,5,1
.???.?.????? 3,1,2,1
???#?.#????..?#? 3,1,3,2
??????#??#?### 1,9
??????#??#???.#? 1,11,1
?#????#?#?#?.? 3,2,4
??#.????#??..?????.? 1,1,4,1,2,1
?.???.???#??###?.? 1,1,2,7
????..#?#???.?.#? 3,5,1,2
???.?#####.##???? 1,1,5,2,2
?#???##??#???? 1,1,8
???.?##?????.?#?#??? 6,3
????#..????.#?#.?#?? 4,4,1,1,1
?#?.#???.????????? 3,1,1,7,1
#??#??#?#..?????? 1,6,1,3
???#???.?#? 1,3,1
?#.?.?#???? 1,1,2
.#???#?.?.????#.??? 1,1,2,1,3,1
??.??####?.##???. 4,4
?#?.??????? 1,3
.???#?#?###????.## 11,1,2
?#???###????? 2,3,1,1
.#?#??##??#?.?????? 1,5,2,2,3
.##?#??#??????#??? 8,3
##?#?#????? 4,1,3
??.????.??#?#?#?#??? 1,9
????.#?.#??##.?#..#? 2,1,1,5,1,1
??.#?#???###. 3,5
#????#?#???.?? 3,6,2
.#??..??#. 3,1,1
???????.?#? 1,4,1
#???????#???#. 1,3,1,5
??###??#???? 7,1
.??#?###??.?#?##?? 9,1,3
#??#???#?????###??# 4,1,1,1,5,1
?#.?#.??#????#. 1,1,3,1
.##.????.??#?. 2,1,4
##.??#?????????..## 2,8,1,2
?.??#?.????. 2,3
?#????##?#.???##?? 2,5,1,2
#????#??#???#.# 1,10,1
???????..? 1,1,1
.??#??#??#?? 4,1,2,1
?????#???..?? 2,1
.#?..???###. 1,6
???#?.??.???. 3,1,1
?#????.#?.??.??.#. 2,2,2,2,1
?.??#.??????? 2,4
??#??????? 2,1,2
???.?##?.??????? 2,2,1,1,1
??.??#??#? 1,5
#????????#?.##? 1,1,2,1,2
#??.?.?#?????????? 1,1,1,4,1,1
???.?#?.?#?? 3,1,1
..??#??..?#?.??..?.? 1,3,1,1,1,1
??#??#??##??#???.??? 6,5,3
??????##????????#.# 2,7,2,1,1
?#?.????.???#?? 2,2,4
?#?#??????#????#?? 1,6,1,1,2
???.?????? 1,2,1
?????????###?#???? 1,9,5
??????##?#????# 1,5,1,4
?.?#?##?...#?? 5,1,1
.???#...?????#?? 2,1,7
???.??.?????.??? 3,1,2,1,1
.?#??????. 1,3
??????????#. 5,2
????#???##????#??#? 2,2,3,8
???#.????????##?# 1,1,1,5,1
??????#?##?? 2,7
?.?##???#??## 1,3,2,2
?.????.?#.. 1,2
?#.????.????. 2,3,3
.?##????????????. 3,1,2
.???.??#?.???#?# 2,2,1,3
?#?.?#??????????? 2,2,1,1,5
????#???#??????###?? 2,3,1,1,6,1
?#?##???#??#??. 4,2,3
??????.?#??.?? 1,1,3,1
.?????#####??###? 9,3
.?.???#?#.????.?## 1,4,3,2
???###.??????? 5,1,2
??#??.???. 4,1
.?.?.?##.??#? 1,3,2
???..?#??#?. 2,6
??.#??#????????.?? 1,5
??#.??##.?? 1,3
?#.?.?#?.?#####????. 2,1,2,7
???.????#?#??? 2,9
??????#?.# 1,1,1
???????????????????# 1,2,1,2,3,1
?.??.?????#??#??#?. 1,2,4,8
??#??????#?#??? 3,2,1,4
.????????.???????? 4,1,1,2,1
??????##?????????.?? 14,1
.????.???????? 1,7
??..?????##?.??#?? 2,7,4
?..#?..???? 2,3
.?.???.#???#?#????. 1,1,8
?????.?.????????? 1,1,8
??.??.??.#??? 1,2,1,4
???.?###???? 2,3,1
?#?????#???.????. 1,5,2
.????#??.?????#?. 4,4
???..?#?????.?. 2,3,3
?##?.??.?#?????#? 4,1,4,3
?????..#????##?? 1,1,1,6
?.??##???..?#?.???? 5,3
.????????#??????.?? 1,10,1
.#?.?????#??# 1,7
?##?##???.???#. 6,1,1,1
?#??.#.?????#?#?. 1,1,1,1,6
?#?###??.#??##?? 7,5,1
.??#??????#?.?#????? 10,5
?#.#??????????#???# 2,2,1,3,3,1
???????????? 2,1,1,4
.#?????##??#???# 1,5,1,3
??..?.??????.?????. 2,3
?#??.??????.#?#????? 2,3,3,1
???#.????##???? 1,8
???????#???#?#??. 1,1,8,1,1
??????.??? 5,1
?.?..??.#??#?#?? 1,1,1,7
.#??##??#??? 6,1,1
.?#.?...?? 2,1,1
??#?##?#?????#???? 14,1
..?#?.?#.????#?? 2,2,4
.???.????.????#?? 2,1,1,1,3
#.???????#??##???# 1,1,1,9,1
??.???#?.. 2,5
?#???#?.??????.???? 3,1,1,1,1,2
.??#.???.#?#? 1,1,2,3
.?#?.#???#. 2,1,1
.?##?##?.?#..???.#?? 7,2,3,1,1
?#????#??#??#.#.?. 1,9,1,1
??.????????????? 2,1,1,2,4
?.????????? 1,1,6
????.#?????#??? 1,1,1,2,2
.??#??.#??.??.?? 1,1,1,2,2
#????????#??# 1,1,3,1
???.?#??.##?? 3,3
?##?#####???. 8,2
????????????##???.?# 10,2,2,2
??.?#.??????.? 1,1,1
?????#.##???.?#??? 3,2,5,3
???.?#????#?.? 1,8
?.##?#???.#??.?.? 6,1,1
???.??#?#???##.?.?? 2,4,4,1
.#??##???#???? 6,4
??....???. 1,3
#??#??.?????? 5,1,2
??????.?#.??..??? 1,1,2,1,2
#??.????????.#?. 3,1,1,4,2
#?.??.????? 1,1,4
?##?.?#.?? 2,1
?.?.##????.?. 1,1,6,1
?????????? 1,1,1
#???.?#?#????????..? 4,9,1,1
????#???##??.? 6,2
?##?????.#???# 3,2,5
???#???#.? 2,1,1
?.??????##.???.. 3,2
?#?.?????? 1,1,2
???.?#?##???#???? 1,6,1,1
#?#.#.#..??.# 3,1,1,1
.??????????? 5,4
??#.?????##??? 3,2,2
????#?#???##???.??? 14,3
#.##????#?.#??.???? 1,7,1,2
???#????#??????? 4,6,2
?.##?#?.?????##? 2,1,7
??.????#?#?????.? 1,1,4,3
#.????.?#? 1,1,3
.?#????##???.???? 1,7,2
.#.?????#??##?# 1,11
??????#??.#? 1,1,1,2
??..#???.?? 2,1,1
??###??#???????.???# 12,1,3
.?#??#.#??????#??#?? 4,1,1,3,1,1
??#??.????? 3,1,1
.?..?????#????#?##?? 2,10
?????????##??. 1,1,5
?????#?#??#?????? 4,1,2,3
#????.???? 1,3,3
?.??#?????#?? 1,9
?...#.??##???? 1,1,4
?..?#.#????????.? 2,1,6,1
#?#??#?.??????#??# 1,1,2,4,3,1
????#?????.?.?? 1,5,1,1,1
????.??#?.#?.?? 4,1,1,2,1
?????????.?###? 7,4
.#?#?#.?.?.????? 3,1,1,1,2
???##.?.##? 2,2
?#.?#.?#???##? 1,1,2,3
?.????#???? 1,2,5
?.?##.??#???.???? 1,3,5,1,1
???..??##?? 2,4,1
.????.????. 2,4
????####..#???.??? 7,3,3
???#???.##?.??#?#? 1,1,1,3,5
??#??##?.? 1,3
??#??????? 2,2
#??..??????? 2,4
?.??.?.???#??#?##?#? 1,1,1,10
????.?#?.?#??#? 2,3,5
?????????????#.?? 5,1,1,4,1
?#?..?#.?.?? 2,2,2
..#.?????? 1,4
?.??##??????..??? 6,2
??????#???? 3,1,1
.?????????#???# 10,1,1
????#?.#.?.???? 1,2,1,1,1
?.#??.????#. 1,2,3,1
??.#????.??#?????#?? 1,1,1,9
???.????#..???. 1,1,5,1
#???????#. 1,2,1
??????#???#?##?#? 1,1,1,7
??.?###???.?? 1,4,1,2
.??#?#?..???#.??### 3,1,4,4
.?###???.??#... 6,3
?#?.??????#?###?#?? 2,7,7
?#??????.??????.. 7,1,1,1
.??????.??.???#?.# 1,1,1,2,2,1
???.??.#???? 1,1,1,1
???#???#??.?????? 1,1,4,2,1
#?#.??#?#???#??? 3,1,1,1,3
?#????????.#..????? 4,1,1,1,4
??..???.?.? 1,3,1
.????...??#????#.?. 2,1,1
?????#??????#???? 6,7
?.??#?#???? 1,5,2
?..??#.???? 3,2
???##???##??#? 1,2,5
?.????????.#??#?.? 1,8,1,1,1
#??#????###?.???.? 5,6,1,1
????#?#?#?#???.??# 13,2
???#?#??#?? 3,4
??..?#???#????..? 1,4
??#..?.????.???? 1,1,3,1,1
?#??..??.???????.? 3,1,2,2,1
?????#???? 2,3,1
????#?#???.?#.#?#??? 1,6,1,1,6
#?##?#?#??????. 4,1,1,4
#?#.???#??? 3,2
.??.??.???#?#.?.? 1,3
???#?????#?? 1,3,1,1
.???????#? 4,2
???#?????.?????.. 1,4,1,1,2
???????????#?# 2,1,5
?.???.??..????.?.. 2,2
???.?????#?.????? 1,1,1,4,3
??.??????#???.?? 2,1,1,4,1
??#??.#.?.?##?#??? 4,1,1,7
?#???#?.???#??.#??#? 6,3,4
??#?.?????????. 4,2,2,2
??#????...##? 5,2
?#???##.??.???#?#. 6,1,5
????????#???#??.? 3,10
???.?#??.???##? 2,2,4
?..?????## 1,6
#??#.#?##???#?? 1,2,1,3,3
?##.????##?.?##. 3,3,3,3
???.???..#??..?.?#? 2,1,1,1,1,2
?#?##????#? 1,8
.??#??#?##?## 1,1,8
.#??..????..#.????? 1,1,1,1,1,5
???????.?#??# 5,3,1
#???#...?? 5,1
.???###??????? 5,1
?#?.?#?..#?#?#???? 1,2,1,1,2,1
#??#???.?????.??? 6,3,1,1,1
????#??.?. 1,2,1
????.??#.?.???#??? 1,1,2,1,1,3
#.???????##?????? 1,10,1,1
?.??.???##???#.? 2,8
?.#???.??#?? 3,2,1
?#?.?#???. 2,4
.???.#?..?#???##???? 1,2,2,7
?#???????? 3,4
?#???.?#?? 1,2,4
?????#.??? 6,2
??.???.?#? 1,2,2
..#?????????#.?#?#?# 5,4,5
#????#??#???? 2,1,4
..??.???.???#??? 2,7
??#??..??.???.?? 5,1,1,1
.?#??#?#.?..#?#.? 7,3
...?#?#?????#.?#? 2,7,2
??..???.?????????.? 2,2,1,4
???.?#???.? 1,3
?#?#?.???##???# 3,5,2
##?#.?##?.#? 4,3,2
.?????##?# 1,5,1
?.???#??#?#?? 1,7
.????##??????.? 12,1
???????.?.? 1,3
?#.???##????..???#?. 2,6,1,4
#???##?#..?????. 1,5,1,1
#??##.???.????.? 1,3,1,1
?.????#..??#??? 3,2
.??????#??... 4,1
.???#.?.??? 2,1
?#??#??#?#????##?? 2,1,11
.?#???..?.??.... 3,1,2
???#??#??????#??? 1,2,2,3,4
.###?????###.???? 11,4
#?#?..#?#???#??????? 4,7,1,1
????????????#?.. 6,3
.#.??..????. 1,1,2,1
?.#??#?#?#?#.?? 1,8,1
.???#??##?????. 1,9
#???????#? 2,1,3
???#?#.?.?.???? 3,1,1,2
.##???#??.??.#???# 8,5
#?##??????????? 1,7,1
??????????.??.?#? 7,1,1,3
#????#????##?????? 13,1,1
#??#????.?.?.?? 1,6,1,1
?#??###???#? 1,8
?????#..?? 2,3,1
.??????.??????. 1,1,5
???.?..???. 2,1
???????#???##?#?? 3,2,5
??????##??# 1,3,1
#??..?#???. 3,5
??#??#??#?#???#????? 15,1
??.???.???###? 2,3
?????????.???#. 2,5,3
.???????..??#? 1,1,1,2
????#?#.?.???.?? 6,1,1,1,1
?.??????.?.## 1,4,1,2
?#??#?#?.##??#?##??? 1,3,11
????##???.. 1,2,1
????.?.????#????#?#? 3,1,1,1,1,3
???.?.????#??.? 1,6
??##??????# 6,1,1
??.?.????##?????. 2,1,2,2
???...??????? 2,1
.?????###???.????.. 9,1
??.?#?.?.?##. 1,3
??#???#.??.# 1,5,1
??.????????? 1,5
???#???.?.#??????.?# 6,1,1,3,1,2
?????#.???##??.?. 1,1,1,7,1
?.??#?#??#?..?#??. 1,4,2,2,1
????????#????#??.?. 2,10
????##???#???.???? 5,3,3
#.#.?##??#?????#? 1,1,7,1,1
#?????##??????? 1,1,6,1
#.??#?.??..#.??#..?? 1,4,1,1,3,1
#???#?#??#.?##? 1,5,1,2
#.?##???#.?.??#. 1,6,1,1,1
???.?.???? 1,1,1
?#?#???.?? 4,1,1
###?##?..???##?? 6,2
?.#.?#??####???#?. 1,8,2
#??.????#??? 1,1,3,2
#?#??..#????#?? 1,1,1,1,3
.???#?#???# 2,5,1
?#?##???.???? 4,1,1
?#?.???##??#??????? 2,8,1,1
?????.???????.??##?? 2,1,6,3
..????.#?#?? 2,4
.?#???????#.??? 1,1,2,1,1
??#?..#?#??..????? 2,4,4
??.?.?..?? 2,1
#??#???##??.??#? 5,5,2
???#????.???? 5,1,1
#???.??#?##?#.#..? 2,1,1,6,1,1
.?#?##????????.? 1,3,2,1
.#?#??.?.? 4,1
??.???#?????????? 1,11
.###????###????. 3,6,1
??#????.?##? 4,3
???.?###???# 1,8
???.???.#???#?.? 1,1,5,1
??##????????#?? 5,2,3
#?#?#????#?##..#???? 1,1,9,1,2
#??.##???.??? 1,2,1,1
?..#?????. 1,3,1
.???#?#??##????#??? 12,2
#??????.??#? 7,3
??#.?#?.#????#?.# 2,1,2,3,1
?#?.?.???????? 2,7
??.??#?#?.????????? 1,4,9
.??.???##? 2,1,2
.??????#.?.?????. 1,1,1,1
???#.???#???#..? 1,1,8,1
.????#???? 1,1,1
?#.??.??????????#.? 1,1,2,5,1
?.?.???.?. 1,1
???.?.#???????#?#?#. 1,1,1,7,1,3
??.?????#??## 1,1,1,2
.????????? 3,2
#?????????#????#. 6,4,2
#??.?##???.??? 1,3,1,2
??.???##?#? 4,2
?#.?????.?.??#?.?. 1,2,1,1,4,1
?????..??.?? 1,1,1,1
#???#???#???? 2,1,5
?.???..?????.?? 3,5
??#..#?????????? 3,1,2,3
#?????###???..?#? 1,1,4,1,1
?##?#?.#?#? 5,4
?##???#??..#??#? 9,1,1
???#???##??????. 4,4
??#???#??#.???#? 3,4,1,1,1
?#?#??.?##?#??#?. 5,8
?.???.?#???.????.. 1,1,5,3
.???.?.?#??? 2,1,1,1
###?.????.?????? 3,1,1,1,1
????#??#???.?##???? 2,2,2,5
???????#.?.?#? 1,4,1,1
???????##?##?????#. 11,1,1
.???#.?#?????### 1,2,1,1,5
??????#????#..?? 8,1,1
#?????#?..???.#? 8,2,1
???#???##???? 1,3,2,2
???????#????.??? 5,1,1,1
?##..???#?? 3,5
.??????#????.? 7,1
?#?#?#???.??#??.?? 8,3
?????##???#? 5,1
??#?.??????#.?? 3,2,1,2,1
.#?????.???..???..?. 5,2
????#??????? 5,1
?.?#??#?###?# 1,10
????#?????.#??..? 5,3
???#?#??.? 2,1,1
.????.???#??? 3,1,3
#?#?#????????. 1,1,1,6
#.?##?#???##??#???? 1,5,2,1,1
????.##??#???#??#? 1,9,2
???#?#?#?#?.?# 8,2,1
??#???#????#????#?.? 9,5,2
#.????????#?. 1,1,1,2
?????..????.???#?. 5,2,3
????.??.?#.? 1,1,1,1
???#??#?.??? 4,1
???#???#?#??#?????? 1,1,4,2,1,1
.?##??..?. 2,1
?#####?????#??.??? 13,2
???.#??#?#??#??. 1,1,7
.???#?????#?#??.?#?# 12,3
?...#...##. 1,2
?.#????????.#?.?.?? 5,2,1,1,1
?#???..#??? 3,1
??.#?????????????#?? 1,1,6,1,1,1
?.?..??#?..????? 1,1,4,1,3
???.??.?#??.#.? 3,1,2,1,1
?#?##??????#??? 1,2,3,3,1
?#???.?.?#??.?#? 3,1,3,1
##??.???.? 3,1
.?.??????? 1,3
?????#?#??.?????## 1,1,1,1,1,6
..????.??..?? 1,2,1,1
??#??.??#????? 4,4,1
??.??.???#?? 1,3
?????#?#???#? 1,1,3,1
??.?..##???? 1,3
.?.???.#?? 1,1,1
?#????##??##???.#??# 4,2,6,1,1
??.#?#?.???##...???? 1,4,4,3
??.?#???.??? 4,1
?#???#?.#???? 6,1,1";
}
