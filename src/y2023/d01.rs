use std::io::{BufRead, Lines};
use crate::solver;
use anyhow::Result;

pub struct Problem(Vec<String>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value.collect::<Result<_, _>>()?))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        self.0.iter().map(|s| {
            [s.chars().find(char::is_ascii_digit).unwrap().to_string(), s.chars().rfind(char::is_ascii_digit).unwrap().to_string()].join("").parse::<u64>().unwrap()
        }).sum::<u64>()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let pats = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        self.0.iter().map(|s| {
            let matches = pats.iter().enumerate().filter_map(|(idx, pat)| {
                let mut matches = s.match_indices(pat);
                let (first, _) = match matches.next() {
                    None => return None,
                    Some(m) => m,
                };
                let last = match matches.last() {
                    Some((v, _)) => v,
                    None => first,
                };
                Some((pats[idx % 9], first, last))
            }).collect::<Vec<_>>();
            let &(first, _, _) = matches.iter().min_by_key(|(_, first, _)| first).unwrap();
            let &(last, _, _) = matches.iter().max_by_key(|(_, _, last)| last).unwrap();

            [first, last].join("").parse::<u64>().unwrap()
        }).sum::<u64>()

    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;
    use test_case::test_case;

    use super::*;

    #[test_case("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet", "142")]
    fn part_one(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_one()));
        Ok(())
    }
    
    #[test_case("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen", "281")]
    fn part_two(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_two()));
        Ok(())
    }
}
