use std::{collections::HashSet, io::Stdin};
use crate::solver;
use anyhow::{anyhow, Result};
use pathfinding::directed::astar;

pub struct Problem(Vec<(usize, usize)>, (usize, usize), usize);

impl TryFrom<Stdin> for Problem
{
    type Error = anyhow::Error;

    fn try_from(value: Stdin) -> Result<Self, Self::Error> {
        let a = value.lines()
            .map(|l| {
                let l = l?;
                let (a, b) = l.split_once(",").ok_or_else(|| anyhow!("missing comma"))?;
                anyhow::Ok((a.parse::<usize>()?, b.parse::<usize>()?))
            }).collect::<Result<Vec<_>, _>>()?;

        Ok(Self(a, (71, 71), 1024))
    }
}

impl Problem {
    fn find_path(self: &Self, count: usize) -> Option<usize> {
        let bytes = &HashSet::<_>::from_iter(self.0.iter().take(count).copied());
        let Some((path, _)) = astar::astar(&(0, 0), |&(x, y)| {
            let mut res = vec![];
            if x > 0 && !bytes.contains(&(x-1, y)) {
                res.push(((x-1, y), 1));
            }
            if y > 0 && !bytes.contains(&(x, y-1)) {
                res.push(((x, y-1), 1));
            }
            if x < self.1.0-1 && !bytes.contains(&(x+1, y)) {
                res.push(((x+1, y), 1));
            }
            if y < self.1.1-1 && !bytes.contains(&(x, y+1)) {
                res.push(((x, y+1), 1));
            }
            res
        }, |&(x, y)| (self.1.0-1).abs_diff(x) + (self.1.1-1).abs_diff(y), |&(x, y)| (self.1.0-1) == x && (self.1.1-1) == y) else {
            return None;
        };

        // for j in 0..self.1.1 {
        //     for i in 0..self.1.0 {
        //         print!("{}", if bytes.contains(&(i,j)) { "#" } else { "." });
        //     }
        //     print!("\n");
        // }
        Some(path.len()-1)
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        self.find_path(self.2).unwrap_or_default()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let mut i = self.2;
        while self.find_path(i).is_some() {
            i <<= 1;
        }

        let (mut i, mut j) = (i >> 1, i);
        while j - i > 1 {
            let mid = (i+j) >> 1;
            if self.find_path(mid).is_none() {
                j = mid;
            } else {
                i = mid;
            }
        }

        format!("{},{}", self.0[i].0, self.0[i].1)
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() {
        assert_eq!("22", format!("{}", Problem(vec![
            (5,4),
            (4,2),
            (4,5),
            (3,0),
            (2,1),
            (6,3),
            (2,4),
            (1,5),
            (0,6),
            (3,3),
            (2,6),
            (5,1),
            (1,2),
            (5,5),
            (2,5),
            (6,5),
            (1,4),
            (0,4),
            (6,4),
            (1,1),
            (6,1),
            (1,0),
            (0,5),
            (1,6),
            (2,0),
        ], (7, 7), 12).part_one()));
    }

    #[test]
    fn part_two() {
        assert_eq!("6,1", format!("{}", Problem(vec![
            (5,4),
            (4,2),
            (4,5),
            (3,0),
            (2,1),
            (6,3),
            (2,4),
            (1,5),
            (0,6),
            (3,3),
            (2,6),
            (5,1),
            (1,2),
            (5,5),
            (2,5),
            (6,5),
            (1,4),
            (0,4),
            (6,4),
            (1,1),
            (6,1),
            (1,0),
            (0,5),
            (1,6),
            (2,0),
        ], (7, 7), 12).part_two()));
    }
}
