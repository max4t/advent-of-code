use std::io::{BufRead, Lines};
use crate::solver;
use anyhow::Result;
use itertools::Itertools;

pub struct Problem(Vec<String>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value.collect::<Result<Vec<_>, _>>()?))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        self.0.iter().filter(|&s| {
            s.chars().filter(|&c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u').count() >= 3
        }).filter(|&s| {
            s.chars().zip(s.chars().skip(1)).find(|(c, n)| c == n).is_some()
        }).filter(|&s| {
            !s.contains("ab") && !s.contains("cd") && !s.contains("pq") && !s.contains("xy")
        }).count()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        self.0.iter().filter(|&s| {
            s.chars()
                .zip(s.chars().skip(1))
                .enumerate()
                .tuple_combinations::<(_, _)>().find(|&((pos1, pair1), (pos2, pair2))| {
                    pair1 == pair2 && pos1.abs_diff(pos2) > 1
                })
                .is_some()
        }).filter(|&s| {
            s.chars()
                .zip(s.chars().skip(2))
                .find(|(a, b)| a == b)
                .is_some()
        }).count()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;
    use test_case::test_case;

    use super::*;

    #[test_case("ugknbfddgicrmopn", "1")]
    #[test_case("aaa", "1")]
    #[test_case("jchzalrnumimnmhp", "0")]
    #[test_case("haegwjzuvuyypxyu", "0")]
    #[test_case("dvszwmarrgswjxmb", "0")]
    fn part_one(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_one()));
        Ok(())
    }
    
    #[test_case("qjhvhtzxzqqjkmpb", "1")]
    #[test_case("xxyxx", "1")]
    #[test_case("uurcxstgmygtbstg", "0")]
    #[test_case("ieodomkazucvgmuy", "0")]
    fn part_two(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_two()));
        Ok(())
    }
}
