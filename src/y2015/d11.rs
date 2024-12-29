use std::{io::{BufRead, Lines}, iter};
use crate::solver;
use anyhow::Result;

pub struct Problem(String);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value.collect::<Result<String, _>>()?))
    }
}

impl Problem {
    fn next_passwords(&self) -> impl Iterator<Item = String> {
        let passwords = iter::successors(Some(self.0.chars().map(|c| (c as u8) - ('a' as u8)).collect::<Vec<_>>()), |s| {
            if s == &vec![25; 8] {
                return None;
            }

            let mut res = s.iter().rev().scan(1_u8, |carry, &v| {
                let r = (v + *carry) % 26;
                *carry = if r == 0 && *carry == 1 { 1 } else { 0 };
                Some(r)
            }).collect::<Vec<_>>();
            res.reverse();
            Some(res)
        }).map(|s| s.into_iter().map(|c| (('a' as u8 + c) as char)).collect::<String>());
        passwords
            .filter(|s| !s.contains('l') && !s.contains('i') && !s.contains('o'))
            .filter(|s| s.chars().zip(s.chars().skip(1)).zip(s.chars().skip(2)).find(|&((a, b), c)| (a as u8) + 1 == (b as u8) && (b as u8) + 1 == (c as u8)).is_some())
            .filter(|s| {
                let mut chars = s.chars().zip(s.chars().skip(1));
                let Some((double_c, _)) = chars.find(|&(a, b)| a == b) else { return false; };
                chars.skip(1).find(|&(a, b)| a == b && a != double_c).is_some()
            })
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        self.next_passwords().nth(0).unwrap()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        self.next_passwords().nth(1).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;
    use test_case::test_case;

    use super::*;

    #[test_case("abcdefgh", "abcdffaa")]
    #[test_case("ghijklmn", "ghjaabcc")]
    fn part_one(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_one()));
        Ok(())
    }
}
