use std::{collections::HashMap, io::{BufRead, Lines}};
use crate::solver;
use anyhow::{anyhow, Result};
use itertools::Itertools;

struct Reindeer {
    speed: u64,
    time: u64,
    rest: u64,
}

impl Reindeer {
    fn position_at(&self, time: u64) -> u64 {
        let round = time / (self.rest + self.time);
        let start_dist = round * self.speed * self.time;
        let last_leg_time = time - round * (self.rest + self.time);
        self.time.min(last_leg_time) * self.speed + start_dist
    }
}

impl TryFrom<&str> for Reindeer {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let (fly, rest) = value.split_once(" seconds, but then must rest for ").ok_or_else(|| anyhow!("missing rest time"))?;
        let rest = rest.strip_suffix(" seconds.").ok_or_else(|| anyhow!("missing end"))?;
        let (speed, time) = fly.split_once(" km/s for ").ok_or_else(|| anyhow!("missing speed"))?;
        Ok(Self {
            speed: speed.parse()?,
            time: time.parse()?,
            rest: rest.parse()?,
        })
    }
}

pub struct Problem(HashMap<String, Reindeer>, u64);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        (value, 2503).try_into()
    }
}

impl<B: BufRead> TryFrom<(Lines<B>, u64)> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: (Lines<B>, u64)) -> Result<Self, Self::Error> {
        Ok(Self(value.0.collect::<Result<Vec<_>, _>>()?.into_iter().map(|s| {
            let (name, info) = s.split_once(" can fly ").ok_or_else(|| anyhow!("missing fly info"))?;
            anyhow::Ok((name.to_string(), info.try_into()?))
        }).collect::<Result<HashMap<_, _>, _>>()?, value.1))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        self.0.values().map(|r| {
            r.position_at(self.1)
        }).max().unwrap()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let mut points = HashMap::<&String, u64>::new();
        for i in 1..=self.1 {
            let names = self.0.iter().max_set_by_key(|r| r.1.position_at(i));
            for (name, _) in names {
                points.entry(name).and_modify(|e| *e += 1).or_insert(1);
            }
        }
        *points.values().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() -> anyhow::Result<()> {
        let pb: Problem = ("Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.".as_bytes().lines(), 1000).try_into()?;
        assert_eq!("1120", format!("{}", pb.part_one()));
        Ok(())
    }

    #[test]
    fn part_two() -> anyhow::Result<()> {
        let pb: Problem = ("Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.".as_bytes().lines(), 1000).try_into()?;
        assert_eq!("689", format!("{}", pb.part_two()));
        Ok(())
    }
}
