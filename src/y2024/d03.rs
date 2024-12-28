use std::io::{BufRead, Lines};
use crate::solver;
use anyhow::Result;

pub struct Problem(String);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value.collect::<Result<Vec<_>, _>>()?.concat()))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let reg = regex::Regex::new(r"mul\((?<x>\d{1,3}),(?<y>\d{1,3})\)").unwrap();
        reg.captures_iter(&self.0).map(|capt| capt["x"].parse::<u64>().unwrap() * capt["y"].parse::<u64>().unwrap()).sum::<u64>()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let reg = regex::Regex::new(r"(?<typt>don)'t\(\)|(?<typ>do)\(\)|(?<typm>mul)\((?<x>\d{1,3}),(?<y>\d{1,3})\)").unwrap();
        let it = reg.captures_iter(&self.0);
        it.scan(true, |state, capt| {
            if let Some(_) = capt.name("typm") {
                if *state {
                    Some(capt["x"].parse::<u64>().unwrap() * capt["y"].parse::<u64>().unwrap())
                } else {
                    Some(0)
                }
            } else if let Some(_) = capt.name("typt") {
                *state = false;
                Some(0)
            } else if let Some(_) = capt.name("typ") {
                *state = true;
                Some(0)
            } else {
                panic!("unknown command")
            }
        }).sum::<u64>()    
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() {
        assert_eq!("161", format!("{}", Problem("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_owned()).part_one()));
    }

    #[test]
    fn part_two() {
        assert_eq!("48", format!("{}", Problem("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_owned()).part_two()));
    }
}
