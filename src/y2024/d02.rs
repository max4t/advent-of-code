use std::io::{BufRead, Lines};
use crate::solver;
use anyhow::{anyhow, Result};

pub struct Problem(Vec<Vec<u64>>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        let res = value
            .map(|s| {
                s.map_err(|err| anyhow!(err)).and_then(|s| {
                    let row = s.trim()
                        .split_whitespace()
                        .map(|l| l.parse::<u64>().map_err(|err| anyhow!(err)))
                        .collect::<Result<Vec<_>, Self::Error>>()?;
                    anyhow::ensure!(row.len() > 1);
                    Ok(row)
                })
            }).collect::<Result<Vec<_>, Self::Error>>()?;
        Ok(Self(res))
    }
}

fn is_valid_report(report: &[u64]) -> bool {
    let pairs = report[..report.len()-1].iter().zip(report[1..].iter());
    let (sigs, diffs): (Vec<_>, Vec<_>) = pairs.map(|(&i, &j)| (i < j, i.abs_diff(j))).unzip();
    
    diffs.iter().all(|&diff| diff >= 1 && diff <= 3) && (sigs.iter().all(|&l| l) || sigs.iter().all(|&l| !l))
}

impl solver::Solver for Problem {

    fn part_one(self: &Self) -> impl std::fmt::Display {
        self.0.iter().filter(|&l| is_valid_report(l)).count()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        self.0.iter().filter(|&l| {
            (0..l.len()).any(|i| {
                let (f, s) = l.split_at(i);
                let l = [f, &s[1..]].concat();
                is_valid_report(&l)
            })
        }).count()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() {
        assert_eq!("2", format!("{}", Problem(vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ]).part_one()));
    }

    #[test]
    fn part_two() {
        assert_eq!("4", format!("{}", Problem(vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ]).part_two()));
    }
}
