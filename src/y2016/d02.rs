use std::{collections::HashMap, io::{BufRead, Lines}};
use crate::{map::{Direction as DirectionBase, Pt}, solver};
use anyhow::{bail, Result};

#[derive(Debug)]
struct Direction(DirectionBase);

impl From<DirectionBase> for Direction {
    fn from(value: DirectionBase) -> Self {
        Direction(value)
    }
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            'U' => DirectionBase::N,
            'D' => DirectionBase::S,
            'L' => DirectionBase::W,
            'R' => DirectionBase::E,
            e => bail!("invalid direction {e}"),
        }.into())
    }
}

pub struct Problem(Vec<Vec<Direction>>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value.map(|l| {
            let l = l?;
            l.chars().map(|c| c.try_into()).collect::<Result<_, _>>()
        }).collect::<Result<_, _>>()?))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        self.0.iter().scan(Pt(1_usize, 1), |state, seq| {
            *state = seq.into_iter().fold(*state, |acc, &Direction(dir)| {
                acc.saturating_add(dir, Pt(3, 3))
            });
            Some(*state)
        }).map(|Pt(x, y)| x + 1 + 3*y).map(|i| i.to_string()).collect::<String>()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let keypad = HashMap::from([
                                              (Pt(2, 0), '1'),
                             (Pt(1, 1), '2'), (Pt(2, 1), '3'), (Pt(3, 1), '4'),
            (Pt(0, 2), '5'), (Pt(1, 2), '6'), (Pt(2, 2), '7'), (Pt(3, 2), '8'), (Pt(4, 2), '9'),
                             (Pt(1, 3), 'A'), (Pt(2, 3), 'B'), (Pt(3, 3), 'C'),
                                              (Pt(2, 4), 'D'),
        ]);
        self.0.iter().scan(Pt(0_usize, 2), |state, seq| {
            *state = seq.into_iter().fold(*state, |acc, &Direction(dir)| {
                let new = acc.saturating_add(dir, Pt(5, 5));
                if keypad.contains_key(&new) {
                    new
                } else {
                    acc
                }
            });
            Some(keypad[state])
        }).collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;
    use test_case::test_case;

    use super::*;

    #[test_case("ULL
RRDDD
LURDL
UUUUD", "1985")]
    fn part_one(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_one()));
        Ok(())
    }
    
    #[test_case("ULL
RRDDD
LURDL
UUUUD", "5DB3")]
    fn part_two(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_two()));
        Ok(())
    }
}
