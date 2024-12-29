use std::io::{BufRead, Lines};
use crate::solver;
use anyhow::Result;

pub struct Problem(Vec<String>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value.collect::<Result<Vec<_>, _>>()?))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        self.0.iter().map(|s| {
            let code_len = s.len();
            let Some(s) = s.strip_prefix('"') else { panic!("invalid content"); };
            let Some(mut s) = s.strip_suffix('"') else { panic!("invalid content"); };

            let mut mem_len = 0;
            while s.len() > 0 {
                match s.chars().nth(0) {
                    Some('\\') if s.chars().nth(1) == Some('x') => s = &s[4..],
                    Some('\\') => s = &s[2..],
                    _ => s = &s[1..],
                }
                mem_len += 1;
            }

            code_len - mem_len
        }).sum::<usize>()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        self.0.iter().map(|s| {
            s.chars().map(|c| match c {
                '\\' | '"' => 2,
                _ => 1,
            }).sum::<usize>() + 2 - s.len()
        }).sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;
    use test_case::test_case;

    use super::*;

    #[test_case("\"\"", "2")]
    #[test_case("\"abc\"", "2")]
    #[test_case("\"aaa\\\"aaa\"", "3")]
    #[test_case("\"\\x27\"", "5")]
    #[test_case("\"mjt\\\\xe\\x85um\"", "6")]
    fn part_one(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_one()));
        Ok(())
    }
    
    #[test_case("\"\"", "4")]
    #[test_case("\"abc\"", "4")]
    #[test_case("\"aaa\\\"aaa\"", "6")]
    #[test_case("\"\\x27\"", "5")]
    fn part_two(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_two()));
        Ok(())
    }
}
