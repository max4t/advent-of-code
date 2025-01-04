use std::{io::{BufRead, Lines}, iter};
use crate::solver;
use anyhow::{anyhow, Result};
use itertools::Itertools;


pub struct Problem(Vec<(String, String)>, String);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        let mut lines = value.collect::<Result<Vec<_>, _>>()?.into_iter();
        let subs = lines.by_ref()
            .take_while(|l| !l.is_empty())
            .map(|l| {
                let (a, b) = l.split_once(" => ").ok_or_else(|| anyhow!("missing delimiter"))?;
                anyhow::Ok((a.to_string(), b.to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(subs, lines.collect::<String>()))
    }
}

fn generated_molecules(molecule: &str, replacements: &[(String, String)]) -> Vec<String> {
    let mut matches = replacements.iter().flat_map(|(a, b)| {
        molecule.match_indices(a).map(|(i, _)| {
            let (first, second) = molecule.split_at(i);
            first.chars().chain(b.chars()).chain(second.chars().skip(a.len())).collect::<String>()
        })
    }).collect::<Vec<_>>();
    matches.sort_unstable();
    matches.dedup();
    matches
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        generated_molecules(&self.1, &self.0).len()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let repls = self.0.iter().sorted_unstable_by_key(|(_, el)| el.chars().filter(|c| c.is_uppercase()).count()).rev().collect::<Vec<_>>();
        let mut seq = iter::successors(Some(self.1.clone()), |s| {
            repls.iter().find_map(|(a, b)| {
                s.contains(b).then(|| s.replacen(b, &a, 1))
            })
        });
        seq.position(|el| el == "e").unwrap()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() -> anyhow::Result<()> {
        let pb: Problem = "H => HO
H => OH
O => HH

HOHOHO".as_bytes().lines().try_into()?;
        assert_eq!("7", format!("{}", pb.part_one()));
        Ok(())
    }

    #[test]
    fn part_two() -> anyhow::Result<()> {
        let pb: Problem = "e => H
e => O
H => HO
H => OH
O => HH

HOHOHO".as_bytes().lines().try_into()?;
        assert_eq!("6", format!("{}", pb.part_two()));
        Ok(())
    }
}
