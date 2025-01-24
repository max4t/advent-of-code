use std::{collections::HashSet, io::{BufRead, Lines}, ops::ControlFlow};
use crate::solver;
use anyhow::Result;

pub struct Problem(Vec<i64>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value.map(|c| anyhow::Ok(c?.parse::<i64>()?)).collect::<Result<_, _>>()?))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        self.0.iter().sum::<i64>()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        self.0.iter()
            .copied()
            .cycle()
            .scan(0, |sum, val| {
                *sum += val;
                Some(*sum)
            })
            .try_fold(HashSet::from([0]), |mut acc, val| {
                if acc.insert(val) {
                    ControlFlow::Continue(acc)
                } else {
                    ControlFlow::Break(val)
                }
            })
            .break_value().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;
    use test_case::test_case;

    use super::*;

    #[test_case("+1
+1
+1", "3")]
    #[test_case("+1
+1
-2", "0")]
    #[test_case("-1
-2
-3", "-6")]
    fn part_one(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_one()));
        Ok(())
    }
    
    #[test_case("+1
-1", "0")]
    #[test_case("+3
+3
+4
-2
-4", "10")]
    #[test_case("-6
+3
+8
+5
-6", "5")]
    #[test_case("+7
+7
-2
-7
-4", "14")]
    fn part_two(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_two()));
        Ok(())
    }
}
