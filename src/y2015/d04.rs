use std::io::{BufRead, Lines};
use crate::solver;
use anyhow::Result;
use md5;

pub struct Problem(String);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value.collect::<Result<String, _>>()?))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        (1_u64..).find(|i| {
            let digest = md5::compute(self.0.clone() + &i.to_string());
            let hash = format!("{:x}", digest);
            hash.starts_with("00000")
        }).unwrap()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        (1_u64..).find(|i| {
            let digest = md5::compute(self.0.clone() + &i.to_string());
            let hash = format!("{:x}", digest);
            hash.starts_with("000000")
        }).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;
    use test_case::test_case;

    use super::*;

    #[test_case("abcdef", "609043")]
    #[test_case("pqrstuv", "1048970")]
    fn part_one(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_one()));
        Ok(())
    }
}
