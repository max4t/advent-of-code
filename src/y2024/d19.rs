use std::{collections::HashMap, io::Stdin};
use crate::solver;
use anyhow::{anyhow, Result};

pub struct Problem(Vec<String>, Vec<String>);

impl TryFrom<Stdin> for Problem
{
    type Error = anyhow::Error;

    fn try_from(value: Stdin) -> Result<Self, Self::Error> {
        let mut a = value.lines();
        let patterns = a.next().ok_or_else(|| anyhow!("expected available patterns"))??.split(", ").map(|s| s.to_owned()).collect::<Vec<_>>();
        a.next();
        let req = a.collect::<Result<Vec<_>, _>>()?;
        Ok(Self(patterns, req))
    }
}

impl Problem {
    fn can_produce<'a, 'b: 'a>(self: &Self, cache: &mut HashMap::<&'a str, bool>, result: &'b str) -> bool {
        if result.is_empty() {
            return true;
        }
        if let Some(&can) = cache.get(&result) {
            return can;
        }

        let can = self.0.iter().any(|pattern| {
            if let Some(rest) = result.strip_prefix(pattern) {
                self.can_produce(cache, rest)
            } else {
                false
            }
        });
        *cache.entry(result).or_insert(can)
    }

    fn count_arrangments<'a, 'b: 'a>(self: &Self, cache: &mut HashMap::<&'a str, u64>, result: &'b str) -> u64 {
        if result.is_empty() {
            return 1;
        }
        if let Some(&can) = cache.get(&result) {
            return can;
        }

        let can = self.0.iter().map(|pattern| {
            if let Some(rest) = result.strip_prefix(pattern) {
                self.count_arrangments(cache, rest)
            } else {
                0
            }
        }).sum::<u64>();
        *cache.entry(result).or_insert(can)
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let cache = &mut HashMap::<&str, bool>::new();
        self.1.iter().filter(|&result| {
            self.can_produce(cache, result)
        }).count()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let cache = &mut HashMap::<&str, u64>::new();
        self.1.iter().map(|result| {
            self.count_arrangments(cache, result)
        }).sum::<u64>()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() {
        assert_eq!("6", format!("{}", Problem(vec![
            "r".to_string(), "wr".to_string(), "b".to_string(), "g".to_string(), "bwu".to_string(), "rb".to_string(), "gb".to_string(), "br".to_string(),
        ], vec![
            "brwrr".to_string(),
            "bggr".to_string(),
            "gbbr".to_string(),
            "rrbgbr".to_string(),
            "ubwu".to_string(),
            "bwurrg".to_string(),
            "brgr".to_string(),
            "bbrgwb".to_string(),
        ]).part_one()));
    }

    #[test]
    fn part_two() {
        assert_eq!("16", format!("{}", Problem(vec![
            "r".to_string(), "wr".to_string(), "b".to_string(), "g".to_string(), "bwu".to_string(), "rb".to_string(), "gb".to_string(), "br".to_string(),
        ], vec![
            "brwrr".to_string(),
            "bggr".to_string(),
            "gbbr".to_string(),
            "rrbgbr".to_string(),
            "ubwu".to_string(),
            "bwurrg".to_string(),
            "brgr".to_string(),
            "bbrgwb".to_string(),
        ]).part_two()));
    }
}
