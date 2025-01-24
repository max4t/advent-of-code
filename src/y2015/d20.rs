use std::io::{BufRead, Lines};
use crate::solver;
use anyhow::Result;

pub struct Problem(usize);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        let lower_bound = value.collect::<Result<String, _>>()?.parse::<usize>()?;
        Ok(Self(lower_bound))
    }
}

// sqrt from https://en.wikipedia.org/wiki/Integer_square_root#Linear_search_using_addition
fn seq_sqrt() -> impl Iterator<Item = (usize, usize)> {
    (1..).scan((0, 1, 3), |(l, a, d), n| {
        while *a <= n {
            *a += *d;
            *d += 2;
            *l += 1;
        }
        Some((n, *l))
    })
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        seq_sqrt().find_map(|(i, sqrt)| {
            let c = (1..=sqrt).filter(|j| i%j == 0).flat_map(|j| [j, i/j]).sum::<usize>()*10;
            (c > self.0).then_some(i)
        }).unwrap()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        seq_sqrt().find_map(|(house, sqrt_house)| {
            let c = (1..=sqrt_house).filter(|&elf| house%elf == 0).flat_map(|elf| [elf, house/elf]).filter(|&elf| elf >= house/50).sum::<usize>()*11;
            (c > self.0).then_some(house)
        }).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() -> anyhow::Result<()> {
        let pb: Problem = "100".as_bytes().lines().try_into()?;
        assert_eq!("6", format!("{}", pb.part_one()));
        Ok(())
    }
}
