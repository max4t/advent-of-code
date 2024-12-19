use std::io::Stdin;
use crate::solver;
use anyhow::Result;

pub struct Problem(Vec<([(i64, i64); 2], (i64, i64))>);

impl TryFrom<Stdin> for Problem
{
    type Error = anyhow::Error;

    fn try_from(value: Stdin) -> Result<Self, Self::Error> {
        let btn_a_scan = regex::Regex::new(r"Button A: X\+(?<x>\d+), Y\+(?<y>\d+)")?;
        let btn_b_scan = regex::Regex::new(r"Button B: X\+(?<x>\d+), Y\+(?<y>\d+)")?;
        let prize_scan = regex::Regex::new(r"Prize: X=(?<x>\d+), Y=(?<y>\d+)")?;
        let a = value.lines()
            .collect::<Result<Vec<_>, _>>()?
            .split(|s| s.trim().is_empty())
            .map(|l| {
                let [btn_a, btn_b, prize] = l else {
                    return Err(anyhow::anyhow!("invalid format"));
                };
                let btn_a_capt = btn_a_scan.captures(&btn_a).ok_or_else(|| anyhow::anyhow!("btn A info"))?;
                let btn_b_capt = btn_b_scan.captures(&btn_b).ok_or_else(|| anyhow::anyhow!("btn B info"))?;
                let prize_capt = prize_scan.captures(&prize).ok_or_else(|| anyhow::anyhow!("prize info"))?;
                anyhow::Ok(([(btn_a_capt["x"].parse::<i64>()?, btn_a_capt["y"].parse::<i64>()?), (btn_b_capt["x"].parse::<i64>()?, btn_b_capt["y"].parse::<i64>()?)], (prize_capt["x"].parse::<i64>()?, prize_capt["y"].parse::<i64>()?)))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(a))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let cost_a = 3;
        let cost_b = 1;
        self.0.iter().filter_map(|&([btn_a, btn_b], prize)| {
            (0..=100_i64).flat_map(|i| (0..=100_i64).filter_map(move |j| if btn_a.0*i + btn_b.0*j == prize.0 && btn_a.1*i + btn_b.1*j == prize.1 { Some(cost_a*i + cost_b*j) } else { None })).min()
        }).sum::<i64>()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let cost_a = 3;
        let cost_b = 1;
        self.0.iter().filter_map(|&([btn_a, btn_b], prize)| {
            let prize = (prize.0+10000000000000, prize.1+10000000000000);
            let i = (btn_b.0*prize.1 - prize.0*btn_b.1)/(btn_a.1*btn_b.0 - btn_a.0*btn_b.1);
            let j = (btn_a.1*prize.0 - btn_a.0*prize.1)/(btn_a.1*btn_b.0 - btn_a.0*btn_b.1);

            if btn_a.0*i + btn_b.0*j == prize.0 && btn_a.1*i + btn_b.1*j == prize.1 {
                Some(cost_a*i + cost_b*j)
            } else {
                None
            }
        }).sum::<i64>()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() {
        assert_eq!("480", format!("{}", Problem(vec![
            ([(94, 34), (22, 67)], (8400, 5400)),
            ([(26, 66), (67, 21)], (12748, 12176)),
            ([(17, 86), (84, 37)], (7870, 6450)),
            ([(69, 23), (27, 71)], (18641, 10279)),
        ]).part_one()));
    }

    #[test]
    fn part_two() {
    }
}
