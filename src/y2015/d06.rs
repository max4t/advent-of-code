use std::io::{BufRead, Lines};
use crate::{map::{Grid, Pt}, solver};
use anyhow::{anyhow, bail, Result};

#[derive(Clone, Copy)]
enum BulbAction {
    On(Pt<usize>, Pt<usize>),
    Off(Pt<usize>, Pt<usize>),
    Toggle(Pt<usize>, Pt<usize>),
}

impl TryFrom<&str> for Pt<usize> {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let (x, y) = value.split_once(",").ok_or_else(|| anyhow!("missing comma separator"))?;
        Ok(Pt(x.parse()?, y.parse()?))
    }
}

impl TryFrom<String> for BulbAction {
    type Error = anyhow::Error;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        if let Some(rest) = value.strip_prefix("turn on ") {
            let (a, b) = rest.split_once(" through ").ok_or_else(|| anyhow!("missing through keyword"))?;
            Ok(BulbAction::On(a.try_into()?, b.try_into()?))
        } else if let Some(rest) = value.strip_prefix("turn off ") {
            let (a, b) = rest.split_once(" through ").ok_or_else(|| anyhow!("missing through keyword"))?;
            Ok(BulbAction::Off(a.try_into()?, b.try_into()?))
        } else if let Some(rest) = value.strip_prefix("toggle ") {
            let (a, b) = rest.split_once(" through ").ok_or_else(|| anyhow!("missing through keyword"))?;
            Ok(BulbAction::Toggle(a.try_into()?, b.try_into()?))
        } else {
            bail!("unknown action");
        }
    }
}

pub struct Problem(Vec<BulbAction>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value.map(|l| {
            let l = l?;
            anyhow::Ok(l.try_into()?)
        }).collect::<Result<Vec<_>, _>>()?))
    }
}

impl Grid<bool> {
    fn process(&mut self, action: BulbAction) {
        match action {
            BulbAction::On(pt, pt1) => self.items_mut(pt..=pt1).for_each(|b| *b = true),
            BulbAction::Off(pt, pt1) => self.items_mut(pt..=pt1).for_each(|b| *b = false),
            BulbAction::Toggle(pt, pt1) => self.items_mut(pt..=pt1).for_each(|b| *b = !*b),
        }
    }
}
impl Grid<u64> {
    fn process(&mut self, action: BulbAction) {
        match action {
            BulbAction::On(pt, pt1) => self.items_mut(pt..=pt1).for_each(|b| *b += 1),
            BulbAction::Off(pt, pt1) => self.items_mut(pt..=pt1).for_each(|b| *b = b.saturating_sub(1)),
            BulbAction::Toggle(pt, pt1) => self.items_mut(pt..=pt1).for_each(|b| *b += 2),
        }
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let mut grid = Grid::new::<1000, 1000>(false);

        self.0.iter().for_each(|&action| {
            grid.process(action);
        });
        grid.iter().filter(|&&l| l).count()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let mut grid = Grid::new::<1000, 1000>(0);

        self.0.iter().for_each(|&action| {
            grid.process(action);
        });
        grid.iter().sum::<u64>()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;
    use test_case::test_case;

    use super::*;

    #[test_case("turn on 0,0 through 999,999", "1000000")]
    #[test_case("toggle 0,0 through 999,0", "1000")]
    #[test_case("turn off 499,499 through 500,500", "0")]
    fn part_one(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_one()));
        Ok(())
    }
    
    #[test_case("turn on 0,0 through 0,0", "1")]
    #[test_case("toggle 0,0 through 999,999", "2000000")]
    fn part_two(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_two()));
        Ok(())
    }
}
