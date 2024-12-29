use std::io::{BufRead, Lines};
use crate::solver;
use anyhow::Result;
use itertools::Itertools;

pub struct Problem(String, usize);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        (value, 40).try_into()
    }
}

impl<B: BufRead> TryFrom<(Lines<B>, usize)> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: (Lines<B>, usize)) -> Result<Self, Self::Error> {
        Ok(Self(value.0.collect::<Result<String, _>>()?, value.1))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let init = self.0.chars().map(|c| (c as usize) - ('0' as usize)).collect::<Vec<_>>();
        (0..self.1).fold(init, |acc, _| {
            acc.into_iter().peekable().batching(|it| {
                match it.next() {
                    None => None,
                    Some(c) => Some([it.peeking_take_while(|&e| e == c).count()+1, c]),
                }
            }).flatten().collect::<Vec<_>>()
        }).len()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let init = self.0.chars().map(|c| (c as usize) - ('0' as usize)).collect::<Vec<_>>();
        (0..50).fold(init, |acc, _| {
            acc.into_iter().peekable().batching(|it| {
                match it.next() {
                    None => None,
                    Some(c) => Some([it.peeking_take_while(|&e| e == c).count()+1, c]),
                }
            }).flatten().collect::<Vec<_>>()
        }).len()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;
    use test_case::test_case;

    use super::*;

    #[test_case("1", 1, "2")]
    #[test_case("11", 1, "2")]
    #[test_case("21", 1, "4")]
    #[test_case("1211", 1, "6")]
    #[test_case("111221", 1, "6")]
    #[test_case("1", 5, "6")]
    fn part_one(example: &str, iter: usize, result: &str) -> anyhow::Result<()> {
        let pb: Problem = (example.as_bytes().lines(), iter).try_into()?;
        assert_eq!(result, format!("{}", pb.part_one()));
        Ok(())
    }
    
    #[test]
    fn part_two() -> anyhow::Result<()> {
        let pb: Problem = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141".as_bytes().lines().try_into()?;
        assert_eq!("982", format!("{}", pb.part_two()));
        Ok(())
    }
}
