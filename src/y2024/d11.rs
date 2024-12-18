use std::{collections::HashMap, io::Stdin, usize};
use crate::solver;
use anyhow::Result;

pub struct Problem(Vec<u64>);

impl TryFrom<Stdin> for Problem
{
    type Error = anyhow::Error;

    fn try_from(value: Stdin) -> Result<Self, Self::Error> {
        let a = value.lines()
            .map(|l| {
                let chars = l?.split_whitespace().map(|l| l.parse::<u64>()).collect::<Result<Vec<_>, _>>()?;
                anyhow::Ok(chars)
            })
            .collect::<Result<Vec<_>, _>>()?.into_iter().flatten().collect::<Vec<_>>();
        Ok(Self(a))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        (0..25).fold(self.0.clone(), |acc, _| {
            acc.iter().flat_map(|&i| {
                if i == 0 {
                    return vec![1];
                }

                let num_digits = i.checked_ilog10().unwrap_or(0) + 1;
                if num_digits % 2 == 0 {
                    let div = 10_u64.pow(num_digits/2);
                    let r = i % div;
                    let l = (i - r)/div;
                    return vec![l, r];
                }

                vec![i*2024]
            }).collect::<Vec<_>>()
        }).len()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let cache = &mut HashMap::<(u64, usize), usize>::new();

        fn compute(cache: &mut HashMap<(u64, usize), usize>, v: u64, rem_level: usize) -> usize {
            if rem_level == 0 {
                return 1;
            }
            if let Some(&r) = cache.get(&(v, rem_level)) {
                return r;
            }

            let res = {
                if v == 0 {
                    return compute(cache, 1, rem_level-1);
                }

                let num_digits = v.checked_ilog10().unwrap_or(0) + 1;
                if num_digits % 2 == 0 {
                    let div = 10_u64.pow(num_digits/2);
                    let r = v % div;
                    let l = (v - r)/div;
                    return compute(cache, l, rem_level-1) + compute(cache, r, rem_level-1);
                }

                compute(cache, v*2024, rem_level-1)
            };

            cache.insert((v, rem_level), res);
            res
        }

        self.0.iter().map(|&v| compute(cache, v, 75)).sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() {
        assert_eq!("55312", format!("{}", Problem(vec![125, 17]).part_one()));
    }

    #[test]
    fn part_two() {
    }
}
