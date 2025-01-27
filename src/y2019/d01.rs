use std::{io::{BufRead, Lines}, iter};
use crate::solver;
use anyhow::Result;

pub struct Problem(Vec<u64>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value.map(|c| anyhow::Ok(c?.parse::<u64>()?)).collect::<Result<_, _>>()?))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        self.0.iter().map(|l| l/3-2).sum::<u64>()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        self.0.iter().map(|&l| {
            iter::successors(Some(l), |&l| (l/3).checked_sub(2)).skip(1).sum::<u64>()
        }).sum::<u64>()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;
    use test_case::test_case;

    use super::*;

    #[test_case("12", "2")]
    #[test_case("14", "2")]
    #[test_case("1969", "654")]
    #[test_case("100756", "33583")]
    fn part_one(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_one()));
        Ok(())
    }
    
    #[test_case("14", "2")]
    #[test_case("1969", "966")]
    #[test_case("100756", "50346")]
    fn part_two(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_two()));
        Ok(())
    }
}
