use std::{collections::HashMap, io::{BufRead, Lines}, iter};
use crate::solver;
use anyhow::Result;
use itertools::Itertools;

pub struct Problem(Vec<i64>, usize);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value.map(|l| anyhow::Ok(l?.parse::<i64>()?)).collect::<Result<Vec<_>, _>>()?, 2000))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        self.0.iter().map(|&v| {
            iter::successors(Some(v), |&v| {
                let mut v = v;
                v ^= v << 6;
                v %= 16777216;
                v ^= v >> 5;
                v %= 16777216;
                v ^= v << 11;
                v %= 16777216;
                Some(v)
            }).nth(self.1).unwrap_or(0)
        }).sum::<i64>()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let mut res = HashMap::new();
        let optim = self.0.iter().map(|&v| {
            let el = iter::successors(Some(v), |&v| {
                let mut v = v;
                v ^= v << 6;
                v %= 16777216;
                v ^= v >> 5;
                v %= 16777216;
                v ^= v << 11;
                v %= 16777216;
                Some(v)
            }).skip(1).take(2000).map(|l| l % 10).collect::<Vec<_>>();
            let mut pos = HashMap::new();
            el.into_iter()
                .tuple_windows::<(_, _)>()
                .map(|(a, b)| (b, b-a))
                .tuple_windows::<(_, _, _, _)>()
                .map(|(a, b, c, d)| (d.0, (a.1, b.1, c.1, d.1)))
                .for_each(|(v, seq)| {
                    pos.entry(seq).or_insert(v);
                });
            pos
        }).fold(&mut res, |acc, n| {
            for (k, v) in n {
                acc.entry(k).and_modify(|sum| *sum += v).or_insert(v);
            }
            acc
        }).into_iter().max_by_key(|&(_, &mut v)| v);
        optim.map(|(_, l)| *l).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() {
        assert_eq!("37327623", format!("{}", Problem(vec![
            1,
            10,
            100,
            2024,
        ], 2000).part_one()));
    }

    #[test]
    fn part_two() {
        assert_eq!("23", format!("{}", Problem(vec![
            1,
            2,
            3,
            2024,
        ], 2000).part_two()));
    }
}
