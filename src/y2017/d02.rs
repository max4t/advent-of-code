use std::io::{BufRead, Lines};
use crate::solver;
use anyhow::Result;
use itertools::Itertools;

pub struct Problem(Vec<Vec<u64>>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value.map(|s| {
            let s = s?;
            anyhow::Ok(s.split_whitespace().map(|s| s.parse()).collect::<Result<_, _>>()?)
        }).collect::<Result<Vec<_>, _>>()?))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        self.0.iter().map(|row| {
            let (min, max) = row.into_iter().minmax().into_option().unwrap();
            max - min
        }).sum::<u64>()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        self.0.iter().map(|row| {
            row.iter().find_map(|el| row.iter().filter(|&o| o > el).find_map(|o| (o % el == 0).then_some(o/el))).unwrap()
        }).sum::<u64>()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;
    use test_case::test_case;

    use super::*;

    #[test_case("5 1 9 5
7 5 3
2 4 6 8", "18")]
    fn part_one(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_one()));
        Ok(())
    }
    
    #[test_case("5 9 2 8
9 4 7 3
3 8 6 5", "9")]
    fn part_two(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_two()));
        Ok(())
    }
}
