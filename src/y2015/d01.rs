use std::{io::{BufRead, Lines}, ops::ControlFlow};
use crate::solver;
use anyhow::{anyhow, Result};

enum Parenthesis {
    Open,
    Close,
}

pub struct Problem(Vec<Parenthesis>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value.flat_map(|l| {
            l.map_or_else(
                |e| vec![Err(anyhow!(e))],
            |s| s.chars().map(|c| match c {
                '(' => Ok(Parenthesis::Open),
                ')' => Ok(Parenthesis::Close),
                _ => Err(anyhow!("unknown char")),
            }).collect())
        }).collect::<Result<Vec<_>, _>>()?))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        self.0.iter().map(|i| {
            match i {
                Parenthesis::Open => 1,
                Parenthesis::Close => -1,
            }
        }).sum::<i64>()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let r = self.0.iter().map(|i| {
            match i {
                Parenthesis::Open => 1,
                Parenthesis::Close => -1,
            }
        }).filter(|&i| i != 0).enumerate().try_fold(0_i64, |acc, (idx, v)| {
            if acc < 0 {
                ControlFlow::Break(idx)
            } else {
                ControlFlow::Continue(acc+v)
            }
        });
        match r {
            ControlFlow::Continue(_) => self.0.len(),
            ControlFlow::Break(v) => v,
        }
    }
}



#[cfg(test)]
mod tests {
    use solver::Solver;
    use test_case::test_case;

    use super::*;

    #[test_case("(())", "0"; "oocc")]
    #[test_case("()()", "0"; "ococ")]
    #[test_case("(((", "3"; "ooo")]
    #[test_case("(()()((", "3"; "oococoo")]
    #[test_case("))(((((", "3"; "ccooooo")]
    #[test_case("())", "-1"; "occ")]
    #[test_case("))(", "-1"; "cco")]
    #[test_case(")))", "-3"; "ccc")]
    #[test_case(")())())", "-3"; "coccocc")]
    fn part_one(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_one()));
        Ok(())
    }
    
    #[test_case(")", "1"; "c")]
    #[test_case("()())", "5"; "ococc")]
    fn part_two(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_two()));
        Ok(())
    }
}
