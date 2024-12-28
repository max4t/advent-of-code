use std::{collections::HashMap, io::{BufRead, Lines}};
use crate::solver;
use anyhow::Result;

pub struct Problem(Vec<((i64, i64), (i64, i64))>, (i64, i64));

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        let a = value
            .map(|l| {
                let l = l?;
                let (pos, speed) = l.split_once(' ').ok_or_else(|| anyhow::anyhow!("missing space delim"))?;
                let pos = pos[2..].split_once(',').ok_or_else(|| anyhow::anyhow!("missing comma delim"))?;
                let speed = speed[2..].split_once(',').ok_or_else(|| anyhow::anyhow!("missing comma delim"))?;

                anyhow::Ok(((pos.0.parse::<i64>()?, pos.1.parse::<i64>()?), (speed.0.parse::<i64>()?, speed.1.parse::<i64>()?)))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(a, (101, 103)))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let (xmax, ymax) = self.1;
        let pos = self.0.iter().map(|&((xpos, ypos), (xs, ys))| {
            ((xpos + 100*xs).rem_euclid(xmax), (ypos + 100*ys).rem_euclid(ymax))
        }).collect::<Vec<_>>();
        let (xmid, ymid) = ((xmax-1)/2, (ymax-1)/2);
        pos.iter().filter(|&&(x, y)| x < xmid && y < ymid).count() *
            pos.iter().filter(|&&(x, y)| x < xmid && y > ymid).count() *
            pos.iter().filter(|&&(x, y)| x > xmid && y < ymid).count() *
            pos.iter().filter(|&&(x, y)| x > xmid && y > ymid).count()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let (xmax, ymax) = self.1;
        for i in 1_000..10_0000 {
            let pos = self.0.iter().map(|&((xpos, ypos), (xs, ys))| {
                ((xpos + i*xs).rem_euclid(xmax), (ypos + i*ys).rem_euclid(ymax))
            }).collect::<Vec<_>>();
            if let Some((y, count)) = pos.iter().map(|&(_, y)| y)
                .fold(HashMap::<i64, usize>::new(), |mut m, x| {
                    *m.entry(x).or_default() += 1;
                    m
                })
                .into_iter()
                .max_by_key(|(_, v)| *v) {
                    if count > 10 {
                        let vals = &mut pos.iter().filter(|&&p| p.1 == y).map(|(x, _)| *x).collect::<Vec<_>>();
                        vals.sort();
                        vals.dedup();
                        if vals.len() >= 10 {
                            for l in 0..(vals.len()-10) {
                                if vals[l]+10 == vals[l+10] {
                                    // PRINT TO CHECK IF THE TREE IS THERE
                                    // let okok = &mut vec![vec!['.'; xmax.try_into().unwrap()]; ymax.try_into().unwrap()];
                                    // pos.iter().for_each(|&(x, y)| {
                                    //     let iop = &mut okok[TryInto::<usize>::try_into(y).unwrap()];
                                    //     iop[TryInto::<usize>::try_into(x).unwrap()] = 'X';
                                    // });
                                    // println!("row: {}", y);
                                    // okok.iter().for_each(|row| {
                                    //     row.iter().for_each(|c| {
                                    //         print!("{}", c);
                                    //     });
                                    //     print!("{}", '\n');
                                    // });
                                    return i;
                                }
                            }
                        }
                    }
                }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() {
        assert_eq!("12", format!("{}", Problem(vec![
            ((0,4), (3,-3)),
            ((6,3), (-1,-3)),
            ((10,3), (-1,2)),
            ((2,0), (2,-1)),
            ((0,0), (1,3)),
            ((3,0), (-2,-2)),
            ((7,6), (-1,-3)),
            ((3,0), (-1,-2)),
            ((9,3), (2,3)),
            ((7,3), (-1,2)),
            ((2,4), (2,-3)),
            ((9,5), (-3,-3)),
        ], (11, 7)).part_one()));
    }

    #[test]
    fn part_two() {
    }
}
