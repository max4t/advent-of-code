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

pub struct Problem(HashMap<String, Reindeer>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value.collect::<Result<Vec<_>, _>>()?.into_iter().map(|s| {
            let (name, info) = s.split_once(" can fly ").ok_or_else(|| anyhow!("missing fly info"))?;
            anyhow::Ok((name.to_string(), info.try_into()?))
        }).collect::<Result<HashMap<_, _>, _>>()?))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let time = 2503;
        self.0.values().map(|r| {
            r.position_at(time)
        }).max().unwrap()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let mut points = HashMap::<&String, u64>::new();
        for i in 1..=2503 {
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
    use test_case::test_case;

    use super::*;

    // #[test_case("[1,2,3]", "6")]
    // #[test_case("{\"a\":2,\"b\":4}", "6")]
    // #[test_case("[[[3]]]", "3")]
    // #[test_case("{\"a\":{\"b\":4},\"c\":-1}", "3")]
    // #[test_case("{\"a\":[-1,1]}", "0")]
    // #[test_case("[-1,{\"a\":1}]", "0")]
    // #[test_case("[]", "0" ; "obj")]
    // #[test_case("{}", "0" ; "arr")]
    // fn part_one(example: &str, result: &str) -> anyhow::Result<()> {
    //     let pb: Problem = example.as_bytes().lines().try_into()?;
    //     assert_eq!(result, format!("{}", pb.part_one()));
    //     Ok(())
    // }

    // #[test_case("[1,2,3]", "6")]
    // #[test_case("[1,{\"c\":\"red\",\"b\":2},3]", "4")]
    // #[test_case("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}", "0")]
    // #[test_case("[1,\"red\",5]", "6")]
    // fn part_two(example: &str, result: &str) -> anyhow::Result<()> {
    //     let pb: Problem = example.as_bytes().lines().try_into()?;
    //     assert_eq!(result, format!("{}", pb.part_two()));
    //     Ok(())
    // }
}
