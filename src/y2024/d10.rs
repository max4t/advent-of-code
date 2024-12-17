use std::{io::Stdin, usize};
use crate::solver;
use anyhow::Result;

pub struct Problem(Vec<Vec<usize>>);

fn to_usize(c: char) -> anyhow::Result<usize> {
    c.to_digit(10).ok_or_else(|| anyhow::anyhow!("invalid number")).and_then(|l| Ok(l.try_into()?))
}

impl TryFrom<Stdin> for Problem
{
    type Error = anyhow::Error;

    fn try_from(value: Stdin) -> Result<Self, Self::Error> {
        let a = value.lines()
            .map(|l| {
                let chars = l?.chars().map(to_usize).collect::<Result<Vec<_>, _>>()?;
                anyhow::Ok(chars)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(a))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let (xmax, ymax) = (self.0.len(), self.0[0].len());
        let zeros = self.0.iter().enumerate()
            .flat_map(|(x, row)| row.iter().enumerate().filter_map(move |(y, c)| if *c == 0 { Some((x, y))} else { None }))
            .collect::<Vec<_>>();
        zeros.into_iter().map(|pos| {
            let mut pos = vec![pos];
            for level in 1..=9 {
                for (x, y) in pos.drain(..).collect::<Vec<_>>() {
                    if let Some(x) = x.checked_sub(1) {
                        if self.0[x][y] == level {
                            pos.push((x, y));
                        }
                    }
                    if let Some(y) = y.checked_sub(1) {
                        if self.0[x][y] == level {
                            pos.push((x, y));
                        }
                    }
                    if let Some(x) = x.checked_add(1).filter(|v| *v < xmax) {
                        if self.0[x][y] == level {
                            pos.push((x, y));
                        }
                    }
                    if let Some(y) = y.checked_add(1).filter(|v| *v < ymax) {
                        if self.0[x][y] == level {
                            pos.push((x, y));
                        }
                    }
                }
                pos.sort();
                pos.dedup();
            }
            pos.len()
        }).sum::<usize>()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let (xmax, ymax) = (self.0.len(), self.0[0].len());
        let zeros = self.0.iter().enumerate()
            .flat_map(|(x, row)| row.iter().enumerate().filter_map(move |(y, c)| if *c == 0 { Some((x, y))} else { None }))
            .collect::<Vec<_>>();
        zeros.into_iter().map(|pos| {
            let mut pos = vec![pos];
            for level in 1..=9 {
                for (x, y) in pos.drain(..).collect::<Vec<_>>() {
                    if let Some(x) = x.checked_sub(1) {
                        if self.0[x][y] == level {
                            pos.push((x, y));
                        }
                    }
                    if let Some(y) = y.checked_sub(1) {
                        if self.0[x][y] == level {
                            pos.push((x, y));
                        }
                    }
                    if let Some(x) = x.checked_add(1).filter(|v| *v < xmax) {
                        if self.0[x][y] == level {
                            pos.push((x, y));
                        }
                    }
                    if let Some(y) = y.checked_add(1).filter(|v| *v < ymax) {
                        if self.0[x][y] == level {
                            pos.push((x, y));
                        }
                    }
                }
            }
            pos.len()
        }).sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() {
        assert_eq!("36", format!("{}", Problem(vec![
            vec![8, 9, 0, 1, 0, 1, 2, 3],
            vec![7, 8, 1, 2, 1, 8, 7, 4],
            vec![8, 7, 4, 3, 0, 9, 6, 5],
            vec![9, 6, 5, 4, 9, 8, 7, 4],
            vec![4, 5, 6, 7, 8, 9, 0, 3],
            vec![3, 2, 0, 1, 9, 0, 1, 2],
            vec![0, 1, 3, 2, 9, 8, 0, 1],
            vec![1, 0, 4, 5, 6, 7, 3, 2],
        ]).part_one()));
    }

    #[test]
    fn part_two() {
        assert_eq!("81", format!("{}", Problem(vec![
            vec![8, 9, 0, 1, 0, 1, 2, 3],
            vec![7, 8, 1, 2, 1, 8, 7, 4],
            vec![8, 7, 4, 3, 0, 9, 6, 5],
            vec![9, 6, 5, 4, 9, 8, 7, 4],
            vec![4, 5, 6, 7, 8, 9, 0, 3],
            vec![3, 2, 0, 1, 9, 0, 1, 2],
            vec![0, 1, 3, 2, 9, 8, 0, 1],
            vec![1, 0, 4, 5, 6, 7, 3, 2],
        ]).part_two()));
    }
}
