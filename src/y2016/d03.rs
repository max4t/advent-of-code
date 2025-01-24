use std::io::{BufRead, Lines};
use crate::solver;
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Problem(Vec<[u64; 3]>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value.map(|l| {
            let l = l?;
            let dists = l.split_whitespace().map(|w| w.parse::<u64>()).collect::<Result<Vec<_>, _>>()?;
            dists.try_into().map_err(|e: Vec<_>| anyhow!("expected 3 values (got {})", e.len()))
        }).collect::<Result<_, _>>()?))
    }
}

fn is_triangle(a: &[u64; 3]) -> bool {
    let mut largest = a[2];
    let mut ok = [a[0], a[1]];
    if ok[0] > largest {
        (ok[0], largest) = (largest, ok[0]);
    }
    if ok[1] > largest {
        (ok[1], largest) = (largest, ok[1]);
    }
    ok.into_iter().sum::<u64>() > largest
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        self.0.iter().cloned().filter(is_triangle).count()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        self.0.iter().map(|l| l[0])
            .chain(self.0.iter().map(|l| l[1]))
            .chain(self.0.iter().map(|l| l[2]))
            .chunks(3).into_iter()
            .map(|a| a.collect::<Vec<_>>().try_into().unwrap())
            .filter(is_triangle)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;
    use test_case::test_case;

    use super::*;

    #[test_case("5 10 25", "0")]
    fn part_one(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_one()));
        Ok(())
    }
}
