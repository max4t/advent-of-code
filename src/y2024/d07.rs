use std::io::{BufRead, Lines};
use crate::solver;
use anyhow::{anyhow, Result};

pub struct Problem(Vec<(u64, Vec<u64>)>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        let a = value
            .map::<Result<_, Self::Error>, _>(|s| {
                let s = s?;
                let Some((res, rest)) = s.split_once(':') else {
                    return Err(anyhow!("expected ':' delimiter"));
                };
                Ok((res.parse::<u64>()?, rest.trim().split(" ").map(|l| anyhow::Ok(l.parse::<u64>()?)).collect::<Result<Vec<_>,_>>()?))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(a))
    }
}

fn can_compute_result(res: u64, acc: u64, ops: &[u64]) -> bool {
    if acc > res {
        return false;
    }

    let Some((&first, rest)) = ops.split_first() else {
        return res == acc;
    };

    can_compute_result(res, acc + first, rest) ||
        can_compute_result(res, acc * first, rest)
}

fn can_compute_result_two(res: u64, acc: u64, ops: &[u64]) -> bool {
    if acc > res {
        return false;
    }

    let Some((&first, rest)) = ops.split_first() else {
        return res == acc;
    };

    can_compute_result_two(res, acc + first, rest) ||
        can_compute_result_two(res, acc * first, rest) ||
        can_compute_result_two(res, format!("{}{}", acc, first).parse::<u64>().unwrap(), rest)
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        self.0.iter().filter(|&(res, nums)| {
            can_compute_result(*res, 0, nums)
        }).map(|&(res, _)| res).sum::<u64>()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        self.0.iter().filter(|&(res, nums)| {
            can_compute_result_two(*res, 0, nums)
        }).map(|&(res, _)| res).sum::<u64>()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() {
        assert_eq!("3749", format!("{}", Problem(vec![
            (190, vec![10, 19]),
            (3267, vec![81, 40, 27]),
            (83, vec![17, 5]),
            (156, vec![15, 6]),
            (7290, vec![6, 8, 6, 15]),
            (161011, vec![16, 10, 13]),
            (192, vec![17, 8, 14]),
            (21037, vec![9, 7, 18, 13]),
            (292, vec![11, 6, 16, 20]),
        ]).part_one()));
    }

    #[test]
    fn part_two() {
        assert_eq!("11387", format!("{}", Problem(vec![
            (190, vec![10, 19]),
            (3267, vec![81, 40, 27]),
            (83, vec![17, 5]),
            (156, vec![15, 6]),
            (7290, vec![6, 8, 6, 15]),
            (161011, vec![16, 10, 13]),
            (192, vec![17, 8, 14]),
            (21037, vec![9, 7, 18, 13]),
            (292, vec![11, 6, 16, 20]),
        ]).part_two()));
    }
}
