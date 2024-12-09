use std::{cmp::Ordering, collections::HashSet, io::Stdin};
use crate::solver;
use anyhow::Result;

pub struct Problem(Vec<(u64, u64)>, Vec<Vec<u64>>);

fn parse_dependencies(s: &str) -> Result<(u64, u64), anyhow::Error> {
    let op = s.split('|').collect::<Vec<_>>();
    Ok((op[0].parse::<u64>()?, op[1].parse::<u64>()?))
}

impl TryFrom<Stdin> for Problem
{
    type Error = anyhow::Error;

    fn try_from(value: Stdin) -> Result<Self, Self::Error> {
        let a = value.lines()
            .map::<Result<_, Self::Error>, _>(|s| Ok(s?)).collect::<Result<Vec<_>, _>>()?;
        let mut a = a.iter();
        let deps = a.by_ref().map_while(|m| if m.is_empty() { None } else { Some(parse_dependencies(m)) }).collect::<Result<Vec<_>, _>>()?;
        let updates = a.map(|l| l.split(',').map(|o| Ok::<_, Self::Error>(o.parse::<u64>()?)).collect::<Result<Vec<_>, _>>()).collect::<Result<Vec<_>, _>>()?;
        Ok(Self(deps, updates))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let h = HashSet::<&(u64, u64)>::from_iter(self.0.iter());
        self.1.iter().filter(|update| {
            let mut update = update.as_slice();
            while let Some((&f, new_update)) = update.split_first() {
                if update.iter().any(|&v| h.contains(&(v, f))) {
                    return false
                }
                update = new_update;
            }
            true
        }).map(|f| f[f.len()/2]).sum::<u64>()    
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let h = HashSet::<&(u64, u64)>::from_iter(self.0.iter());
        let updates = &self.1;
        updates.iter().filter(|update| {
            let mut update = update.as_slice();
            while let Some((&f, new_update)) = update.split_first() {
                if update.iter().any(|&v| h.contains(&(v, f))) {
                    return true
                }
                update = new_update;
            }
            false
        })
        .map(|f| {
            let mut f = f.clone();
            f.sort_by(|&a, &b| {
                if h.contains(&(b, a)) { Ordering::Greater } else { Ordering::Less }
            });
            f
        })
        .map(|f| f[f.len()/2]).sum::<u64>()    
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() {
        assert_eq!("143", format!("{}", Problem(vec![
            (47,53),
            (97,13),
            (97,61),
            (97,47),
            (75,29),
            (61,13),
            (75,53),
            (29,13),
            (97,29),
            (53,29),
            (61,53),
            (97,53),
            (61,29),
            (47,13),
            (75,47),
            (97,75),
            (47,61),
            (75,61),
            (47,29),
            (75,13),
            (53,13),
        ], vec![
            vec![75,47,61,53,29],
            vec![97,61,53,29,13],
            vec![75,29,13],
            vec![75,97,47,61,53],
            vec![61,13,29],
            vec![97,13,75,29,47],
        ]).part_one()));
    }

    #[test]
    fn part_two() {
        assert_eq!("123", format!("{}", Problem(vec![
            (47,53),
            (97,13),
            (97,61),
            (97,47),
            (75,29),
            (61,13),
            (75,53),
            (29,13),
            (97,29),
            (53,29),
            (61,53),
            (97,53),
            (61,29),
            (47,13),
            (75,47),
            (97,75),
            (47,61),
            (75,61),
            (47,29),
            (75,13),
            (53,13),
        ], vec![
            vec![75,47,61,53,29],
            vec![97,61,53,29,13],
            vec![75,29,13],
            vec![75,97,47,61,53],
            vec![61,13,29],
            vec![97,13,75,29,47],
        ]).part_two()));
    }
}
