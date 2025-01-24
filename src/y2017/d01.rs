use std::io::{BufRead, Lines};
use crate::solver;
use anyhow::Result;

pub struct Problem(Vec<u8>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        let value = value.collect::<Result<String, _>>()?;
        Ok(Self(value.chars().map(|c| c as u8 - '0' as u8).collect()))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        self.0.iter().zip(self.0.iter().cycle().skip(1)).filter_map(|(&a, &b)| (a == b).then_some(a as u64)).sum::<u64>()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        self.0.iter().zip(self.0.iter().cycle().skip(self.0.len()/2)).filter_map(|(&a, &b)| (a == b).then_some(a as u64)).sum::<u64>()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;
    use test_case::test_case;

    use super::*;

    #[test_case("1122", "3")]
    #[test_case("1111", "4")]
    #[test_case("1234", "0")]
    #[test_case("91212129", "9")]
    fn part_one(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_one()));
        Ok(())
    }
    
    #[test_case("1212", "6")]
    #[test_case("1221", "0")]
    #[test_case("123425", "4")]
    #[test_case("123123", "12")]
    #[test_case("12131415", "4")]
    fn part_two(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_two()));
        Ok(())
    }
}
