use std::{io::{BufRead, Lines}, iter};
use crate::solver;
use anyhow::{anyhow, Ok, Result};

pub struct Problem(usize, usize);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        let value = value.collect::<Result<Vec<String>, _>>()?.into_iter().collect::<String>();
        let value = value.strip_prefix("To continue, please consult the code grid in the manual.  Enter the code at row ")
            .ok_or_else(|| anyhow!("invalid input"))?
            .strip_suffix(".")
            .ok_or_else(|| anyhow!("invalid input"))?;
        let (x, y) = value.split_once(", column ")
            .ok_or_else(|| anyhow!("invalid input"))?;
        Ok((x.parse::<usize>()?, y.parse::<usize>()?).into())
    }
}

impl From<(usize, usize)> for Problem {
    fn from((a, b): (usize, usize)) -> Self {
        Self(a, b)
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let l = self.0 + self.1 - 1;
        let l = (l-1)*l/2 + self.1 - 1;
        iter::successors(Some(20151125_u64), |n| {
            Some((n*252533) % 33554393)
        }).nth(l).unwrap()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        0
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() -> anyhow::Result<()> {
        let pb: Problem = (4, 5).into();
        assert_eq!("10600672", format!("{}", pb.part_one()));
        Ok(())
    }
}
