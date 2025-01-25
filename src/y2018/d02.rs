use std::{collections::HashMap, io::{BufRead, Lines}};
use crate::solver;
use anyhow::Result;
use itertools::Itertools;

pub struct Problem(Vec<String>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value.collect::<Result<_, _>>()?))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let (twos, threes) = self.0.iter().fold((0, 0), |(twos, threes), s| {
            let counts = s.chars().fold(HashMap::new(), |mut count, val| {
                count.entry(val).and_modify(|c| *c += 1).or_insert(1);
                count
            });
            (
                twos + counts.values().any(|&v| v == 2).then_some(1).unwrap_or(0),
                threes + counts.values().any(|&v| v == 3).then_some(1).unwrap_or(0),
            )
        });
        twos*threes
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        self.0.iter().cartesian_product(self.0.iter()).find_map(|(a, b)| {
            if a == b {
                return None;
            }
            a.char_indices().zip(b.char_indices())
                .filter_map(|((ai, a), (_, b))| {
                    if a == b {
                        return None;
                    }
                    Some(ai)
                })
                .exactly_one()
                .ok().and_then(|i| { let mut a = a.clone(); a.remove(i); Some(a) })
        }).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;
    use test_case::test_case;

    use super::*;

    #[test_case("abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab", "12")]
    fn part_one(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_one()));
        Ok(())
    }
    
    #[test_case("abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz", "fgij")]
    fn part_two(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_two()));
        Ok(())
    }
}
