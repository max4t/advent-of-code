use std::{collections::HashMap, io::{BufRead, Lines}, iter::zip};
use crate::solver;
use anyhow::{anyhow, Result};

pub struct Problem(Vec<u64>, Vec<u64>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        let (a, b): (Vec<_>, Vec<_>) = value
            .map(|res| {
                res.map_err(|err| anyhow!(err)).and_then(|m| {
                    let s = m.trim();
                    let res= s.split_whitespace().collect::<Vec<_>>();
                    // if res.len() != 
                    anyhow::ensure!(res.len() == 2, "must have 2 numbers, got {}", res.len());
                    Ok((res[0].parse::<u64>()?, res[1].parse::<u64>()?))
                })
            })
            .collect::<Result<Vec<_>, anyhow::Error>>()?
            .into_iter()
            .unzip();
        anyhow::ensure!(a.len() == b.len(), "lists have different lengths (a = {}, b = {})", a.len(), b.len());
        Ok(Self(a, b))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let mut a = self.0.clone();
        let mut b = self.1.clone();
        a.sort_unstable();
        b.sort_unstable();
        zip(a, b)
            .map(|(o, t)| o.abs_diff(t))
            .sum::<u64>()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let mut counters = HashMap::new();
        for &el in &self.1 {
            counters.entry(el).and_modify(|e| *e += 1).or_insert(1);
        }
        self.0.iter().map(|e| e * counters.get(e).unwrap_or(&0)).sum::<u64>()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() {
        assert_eq!("11", format!("{}", Problem(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]).part_one()));
    }

    #[test]
    fn part_two() {
        assert_eq!("31", format!("{}", Problem(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]).part_two()));
    }
}
