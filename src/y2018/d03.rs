use std::io::{BufRead, Lines};
use crate::{map::{Grid, Pt}, solver};
use anyhow::{anyhow, Result};

struct Claim {
    id: usize,
    start: Pt<usize>,
    size: Pt<usize>,
}

impl TryFrom<String> for Claim {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.strip_prefix("#").ok_or_else(|| anyhow!("expected claim prefix (#)"))?;
        let (claim, value) = value.split_once(" @ ").ok_or_else(|| anyhow!("expected `at` separator"))?;
        // let claim = claim.parse::<usize>()?;
        let (start, size) = value.split_once(": ").ok_or_else(|| anyhow!("expected colon separator"))?;
        let (startx, starty) = start.split_once(",").ok_or_else(|| anyhow!("expected comma"))?;
        let (sizex, sizey) = size.split_once("x").ok_or_else(|| anyhow!("expected x"))?;
        Ok(Self {
            id: claim.parse()?,
            start: Pt(startx.parse()?, starty.parse()?),
            size: Pt(sizex.parse()?, sizey.parse()?),
        })
    }
}
pub struct Problem(Vec<Claim>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value.map(|s| anyhow::Ok(s?.try_into()?)).collect::<Result<_, _>>()?))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        self.0.iter().fold(Grid::new::<1000, 1000>(0), |mut grid, claim| {
            grid.items_mut(claim.start..(claim.start+claim.size)).for_each(|val| {
                *val += 1;
            });
            grid
        }).iter().filter(|&&v| v >= 2).count()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let grid = self.0.iter().fold(Grid::new::<1000, 1000>(0), |mut grid, claim| {
            grid.items_mut(claim.start..(claim.start+claim.size)).for_each(|val| {
                *val += 1;
            });
            grid
        });
        self.0.iter().find_map(|claim| {
            grid.items(claim.start..(claim.start+claim.size)).all(|&val| val == 1).then_some(claim.id)
        }).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;
    use test_case::test_case;

    use super::*;

    #[test_case("#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2", "4")]
    fn part_one(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_one()));
        Ok(())
    }
    
    #[test_case("#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2", "3")]
    fn part_two(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_two()));
        Ok(())
    }
}
