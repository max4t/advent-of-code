use std::io::{BufRead, Lines};
use crate::solver;
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Problem(Vec<(u64, u64, u64)>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value.map(|l| {
            let l = l?;
            let dims = l.split("x")
                .map(|l| l.parse::<u64>())
                .collect::<Result<Vec<_>, _>>()?
                .into_iter().collect_tuple::<(_, _, _)>()
                .ok_or_else(|| anyhow!("expected 3 values"))?;
            anyhow::Ok(dims)
        }).collect::<Result<Vec<_>, _>>()?))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        self.0.iter().map(|&(l, w, h)| {
            let sides = [l*w, l*h, w*h];
            sides.iter().sum::<u64>()*2 + sides.iter().min().unwrap()
        }).sum::<u64>()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        self.0.iter().map(|&(l, w, h)| {
            let length = [l, w, h];
            (length.iter().sum::<u64>() - length.iter().max().unwrap())*2 + length.iter().product::<u64>()
        }).sum::<u64>()
    }
}



#[cfg(test)]
mod tests {
    use solver::Solver;
    use test_case::test_case;

    use super::*;

    #[test_case("2x3x4", "58")]
    #[test_case("1x1x10", "43")]
    fn part_one(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_one()));
        Ok(())
    }
    
    #[test_case("2x3x4", "34")]
    #[test_case("1x1x10", "14")]
    fn part_two(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_two()));
        Ok(())
    }
}
