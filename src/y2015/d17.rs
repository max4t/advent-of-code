use std::io::{BufRead, Lines};
use crate::solver;
use anyhow::Result;
use itertools::Itertools;


pub struct Problem(Vec<usize>, usize);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        (value, 150).try_into()
    }
}

impl<B: BufRead> TryFrom<(Lines<B>, usize)> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: (Lines<B>, usize)) -> Result<Self, Self::Error> {
        Ok(Self(value.0
            .collect::<Result<Vec<_>, _>>()?.into_iter()
            .map(|s| {
                anyhow::Ok(s.parse::<usize>()?)
            })
            .collect::<Result<_, _>>()?, value.1))
    }
}

fn process_sizes(mut containers: &[usize], rest: usize) -> Vec<Vec<usize>> { // TODO reduce allocations?
    let mut res = vec![];
    while let Some((&first, rem)) = containers.split_first() {
        if first == rest {
            res.push(vec![first]);
        } else if first < rest {
            res.extend(process_sizes(rem, rest - first).into_iter().map(|res| [vec![first], res].concat()));
        }
        containers = rem;
    }
    return res;
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        process_sizes(&self.0, self.1).len()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        process_sizes(&self.0, self.1).iter().min_set_by_key(|s| s.len()).len()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() -> anyhow::Result<()> {
        let pb: Problem = ("20
15
10
5
5".as_bytes().lines(), 25).try_into()?;
        assert_eq!("4", format!("{}", pb.part_one()));
        Ok(())
    }

    #[test]
    fn part_two() -> anyhow::Result<()> {
        let pb: Problem = ("20
15
10
5
5".as_bytes().lines(), 25).try_into()?;
        assert_eq!("3", format!("{}", pb.part_two()));
        Ok(())
    }
}
