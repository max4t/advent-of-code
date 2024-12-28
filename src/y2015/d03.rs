use std::{collections::HashMap, io::{BufRead, Lines}};
use crate::{map::{Direction, Pt}, solver};
use anyhow::{anyhow, Result};

pub struct Problem(Vec<Direction>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value.flat_map(|l| {
            l.map_or_else(
                |e| vec![Err(anyhow!(e))],
            |s| s.chars().map(|c| c.try_into()).collect())
        }).collect::<Result<Vec<_>, _>>()?))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let mut m = HashMap::from([(Pt(0, 0), 1)]);
    
        self.0.iter().fold((0, 0).into(), |pos, &v| {
            let pos = pos + v;
            m.entry(pos).and_modify(|v| *v += 1).or_insert(1);
            pos
        });
    
        m.values().filter(|&&l| l > 0).count()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let mut m = HashMap::from([(Pt(0, 0), 2)]);

        let (f, s) = self.0.iter().enumerate().partition::<Vec<(usize, _)>, _>(|(i, _)| i % 2 == 0);

        f.iter().fold((0, 0).into(), |pos, &(_, &v)| {
            let pos = pos + v;
            m.entry(pos).and_modify(|v| *v += 1).or_insert(1);
            pos
        });

        s.iter().fold((0, 0).into(), |pos, &(_, &v)| {
            let pos = pos + v;
            m.entry(pos).and_modify(|v| *v += 1).or_insert(1);
            pos
        });

        m.values().filter(|&&l| l > 0).count()    
    }
}



#[cfg(test)]
mod tests {
    use solver::Solver;
    use test_case::test_case;

    use super::*;

    #[test_case(">", "2")]
    #[test_case("^>v<", "4")]
    #[test_case("^v^v^v^v^v", "2")]
    fn part_one(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_one()));
        Ok(())
    }
    
    #[test_case("^v", "3" ; "short")]
    #[test_case("^>v<", "3")]
    #[test_case("^v^v^v^v^v", "11")]
    fn part_two(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_two()));
        Ok(())
    }
}
