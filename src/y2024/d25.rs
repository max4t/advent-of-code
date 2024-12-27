use std::{array, io::Stdin};
use crate::solver;
use anyhow::Result;
use itertools::Itertools;

#[derive(Clone, Debug)]
enum Schematic {
    Key([usize; 5]),
    Lock([usize; 5]),
}

pub struct Problem(Vec<Schematic>);

impl TryFrom<Stdin> for Problem
{
    type Error = anyhow::Error;

    fn try_from(value: Stdin) -> Result<Self, Self::Error> {
        let a = value.lines().collect::<Result<Vec<_>, _>>()?;
        Ok(Self(a.split(|s| s.is_empty()).map(|s| {
            if s[0].starts_with('#') {
                let s = &s[1..];
                Schematic::Lock(array::from_fn(|i| {
                    s.iter().find_position(|s| s.as_bytes()[i] == b'.').unwrap().0
                }))
            } else {
                let s = &s[1..];
                Schematic::Key(array::from_fn(|i| {
                    5 - s.iter().find_position(|s| s.as_bytes()[i] == b'#').unwrap().0
                }))
            }
        }).collect()))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let (locks, keys): (Vec<&[usize; 5]>, Vec<&[usize; 5]>) = self.0.iter().partition_map(|s| match s {
            Schematic::Lock(v) => itertools::Either::Left(v),
            Schematic::Key(v) => itertools::Either::Right(v),
        });
        locks.into_iter().cartesian_product(keys.into_iter()).filter(|&(lock, key)| lock[..].iter().zip(key[..].iter()).all(|(lvl1, lvl2)| lvl1 + lvl2 <= 5)).count()
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
    fn part_one() {
        assert_eq!("3", format!("{}", Problem(vec![
            Schematic::Lock([0,5,3,4,3]),
            Schematic::Lock([1,2,0,5,3]),
            Schematic::Key([5,0,2,1,3]),
            Schematic::Key([4,3,4,0,2]),
            Schematic::Key([3,0,2,0,1]),
        ]).part_one()));
    }

    #[test]
    fn part_two() {
        assert_eq!("co,de,ka,ta", format!("{}", Problem(vec![]).part_two()));
    }
}
