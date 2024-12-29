use std::{collections::{HashMap, HashSet}, io::{BufRead, Lines}};
use crate::solver;
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Problem(HashMap<(String, String), i64>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value.collect::<Result<Vec<_>, _>>()?.into_iter().map(|s| {
            let (name, rest)= s.split_once(" would ").ok_or_else(|| anyhow!("invalid line"))?;
            let (hap, mut other_name) = rest.split_once(" happiness units by sitting next to ").ok_or_else(|| anyhow!("invalid line"))?;
            other_name = &other_name[..(other_name.len()-1)];
            let (d, a) = hap.split_once(" ").ok_or_else(|| anyhow!("invalid line"))?;
            anyhow::Ok(((name.to_string(), other_name.to_string()), if d == "gain" { 1 } else { -1 }*a.parse::<i64>()?))
        }).collect::<Result<_, _>>()?))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let names = self.0.keys().flat_map(|(a, b)| [a.clone(), b.clone()]).collect::<HashSet<_>>();
        let changes = names.iter().combinations(2).flat_map(|v| {
            let change = self.0[&(v[0].clone(), v[1].clone())] + self.0[&(v[1].clone(), v[0].clone())];
            [((v[0], v[1]), change), ((v[1], v[0]), change)]
        }).collect::<HashMap<_, _>>();
        names.iter().permutations(names.len()).map(|order| {
            order.iter().zip(order.iter().skip(1).chain([&order[0]])).map(|(&a, &b)| changes[&(a, b)]).sum::<i64>()
        }).max().unwrap_or(0)
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let names = self.0.keys().flat_map(|(a, b)| [a.clone(), b.clone()]).collect::<HashSet<_>>();
        let changes = names.iter().combinations(2).flat_map(|v| {
            let change = self.0[&(v[0].clone(), v[1].clone())] + self.0[&(v[1].clone(), v[0].clone())];
            [((v[0], v[1]), change), ((v[1], v[0]), change)]
        }).collect::<HashMap<_, _>>();
        let (order, max) = names.iter().permutations(names.len()).map(|order| {
            (order.clone(), order.iter().zip(order.iter().cycle().skip(1)).map(|(&a, &b)| changes[&(a, b)]).sum::<i64>())
        }).max_by_key(|(_, m)| *m).unwrap();
        max - order.iter().zip(order.iter().cycle().skip(1)).map(|(&a, &b)| changes[&(a, b)]).min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() -> anyhow::Result<()> {
        let pb: Problem = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.".as_bytes().lines().try_into()?;
        assert_eq!("330", format!("{}", pb.part_one()));
        Ok(())
    }
}
