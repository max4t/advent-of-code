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

pub fn approximated_sqrt(n: usize) -> usize {
    let mut num_bits = (std::mem::size_of::<usize>() << 3) - 1;
    while ((n >> num_bits) & 1) == 0 {
        num_bits -= 1;
    }
    
    1 << ((num_bits >> 1) + 1)
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        (1..).find(|i| {
            let c = (1..=(approximated_sqrt(*i))).filter(|j| i%j == 0).flat_map(|j| [j, i/j]).sum::<usize>()*10;
            c > self.0
        }).unwrap()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        (1..).find(|&house| {
            let c = (1..=(approximated_sqrt(house))).filter(|&elf| house%elf == 0).flat_map(|elf| [elf, house/elf]).filter(|&elf| elf >= house/50).sum::<usize>()*11;
            if house % 100_000 == 0 {
                dbg!((house, c));
            }
            c > self.0
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
