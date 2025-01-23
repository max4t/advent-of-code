use std::{collections::HashSet, fmt::Debug, io::{BufRead, Lines}, iter::{repeat_n, RepeatN}, ops::{Add, AddAssign}};
use crate::{map::{Direction, Pt as PtBase, Side}, solver};
use anyhow::{anyhow, bail, Result};

type Pt = PtBase<isize>;

impl Default for Pt {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}

#[derive(Clone, Copy, Debug)]
struct Position(Pt, Direction);

impl Position {
    fn distance(self) -> usize {
        self.0.manhattan_distance(Pt::default())
    }
}

impl Default for Position {
    fn default() -> Self {
        Self(Pt::default(), Direction::N)
    }
}

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        Self(self.0 + rhs, rhs)
    }
}

impl AddAssign<Instruction> for Position {
    fn add_assign(&mut self, rhs: Instruction) {
        *self = *self + rhs;
    }
}

impl Add<Instruction> for Position {
    type Output = Self;

    fn add(self, rhs: Instruction) -> Self::Output {
        rhs.decompose(self.1).fold(self, |acc, e| {
            acc + e
        })
    }
}

#[derive(Clone, Copy)]
struct Instruction(Side, usize);

impl Instruction {
    fn decompose(self, direction: Direction) -> RepeatN<Direction> {
        repeat_n(direction + self.0, self.1)
    }
}

impl TryFrom<&str> for Instruction {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (dir, dist) = value.split_at_checked(1).ok_or_else(|| anyhow!("empty instruction"))?;
        Ok(Instruction(match dir {
            "L" => Side::Left,
            "R" => Side::Right,
            e => bail!("invalid direction {e}"),
        }, dist.parse()?))
    }
}

#[derive(Debug)]
struct RecordedPosition {
    pos: Position,
    previous: HashSet<Pt>,
}

impl RecordedPosition {
    fn go(&mut self, dir: Direction) -> Result<(), Position> {
        let new_pos = self.pos + dir;
        self.previous.insert(new_pos.0).then(|| { self.pos = new_pos; }).ok_or(new_pos)
    }

    fn go_by(&mut self, i: Instruction) -> Result<&mut Self, Position> {
        i.decompose(self.pos.1).try_fold(self, |acc, dir| {
            acc.go(dir).map(|_| acc)
        })
    }
}

impl Default for RecordedPosition {
    fn default() -> Self {
        let pos = Position::default();
        Self {
            previous: HashSet::from([pos.0]),
            pos,
        }
    }
}

pub struct Problem(Vec<Instruction>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        let value = value.collect::<Result<String, _>>()?;
        Ok(Self(value.split(", ").map(|l| l.try_into()).collect::<Result<_, _>>()?))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        self.0.iter().fold(Position::default(), |pt, &i| {
            pt + i
        }).distance()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        self.0.iter().try_fold(&mut RecordedPosition::default(), |acc, &i| {
            acc.go_by(i)
        }).expect_err("never visit a place twice").distance()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;
    use test_case::test_case;

    use super::*;

    #[test_case("R2, L3", "5")]
    #[test_case("R2, R2, R2", "2")]
    #[test_case("R5, L5, R5, R3", "12")]
    fn part_one(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_one()));
        Ok(())
    }
    
    #[test_case("R8, R4, R4, R8", "4")]
    fn part_two(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_two()));
        Ok(())
    }
}
