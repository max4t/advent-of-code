use std::{collections::{HashMap, HashSet}, io::{BufRead, Lines}};
use crate::solver;
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Problem(HashMap<(String, String), u64>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        let cities = value.map(|s| {
            let s = s?;
            let (cities, distance) = s.split_once(" = ").ok_or_else(|| anyhow!("missing distance delimiter"))?;
            let (city1, city2) = cities.split_once(" to ").ok_or_else(|| anyhow!("missing city delimiter"))?;
            anyhow::Ok(((city1.to_string(), city2.to_string()), distance.parse::<u64>()?))
        })
        .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(HashMap::from_iter(cities.into_iter().flat_map(|((c1, c2), d)| [((c1.clone(), c2.clone()), d), ((c2, c1), d)]))))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let cities = self.0.keys().cloned().flat_map(|(a, b)| [a, b]).collect::<HashSet<_>>();
        cities.iter().permutations(cities.len())
            .map(|order| order.iter().zip(order.iter().skip(1)).map(|(&a, &b)| { self.0.get(&(a.clone(), b.clone())).unwrap_or(&0) }).sum::<u64>())
            .min().unwrap_or(0)
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let cities = self.0.keys().cloned().flat_map(|(a, b)| [a, b]).collect::<HashSet<_>>();
        cities.iter().permutations(cities.len())
            .map(|order| order.iter().zip(order.iter().skip(1)).map(|(&a, &b)| { self.0.get(&(a.clone(), b.clone())).unwrap_or(&0) }).sum::<u64>())
            .max().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() -> anyhow::Result<()> {
        let pb: Problem = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141".as_bytes().lines().try_into()?;
        assert_eq!("605", format!("{}", pb.part_one()));
        Ok(())
    }
    
    #[test]
    fn part_two() -> anyhow::Result<()> {
        let pb: Problem = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141".as_bytes().lines().try_into()?;
        assert_eq!("982", format!("{}", pb.part_two()));
        Ok(())
    }
}
