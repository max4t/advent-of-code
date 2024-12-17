use std::{collections::{HashMap, HashSet}, io::Stdin};
use crate::solver;
use anyhow::Result;

pub struct Problem((usize, usize), HashMap<char, HashSet<(usize, usize)>>);

impl TryFrom<Stdin> for Problem
{
    type Error = anyhow::Error;

    fn try_from(value: Stdin) -> Result<Self, Self::Error> {
        let a = value.lines()
            .map(|l| {
                anyhow::Ok(l?.chars().collect::<Vec<char>>())
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut iop = HashMap::<char, HashSet<(usize, usize)>>::new();
        let okok = a.iter().enumerate().flat_map(|(x, l)| l.into_iter().enumerate().filter_map(move |(y, ll)| if *ll == '.' { None } else { Some((*ll, (x, y))) }));
        for (c, pos) in okok {
            iop.entry(c).and_modify(|s| { s.insert(pos); }).or_insert_with(|| HashSet::from([pos]));
        }
        Ok(Self((a.len(), a[0].len()), iop))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let (xmax, ymax) = self.0;
        let positions = self.1.iter().flat_map(|(_, l)| {
            let positions = &l.into_iter().collect::<Vec<_>>();
            positions.into_iter().enumerate().flat_map(|(s, &&pos)| {
                positions[(s+1)..].into_iter().map(move |&&l| (pos, l))
            }).flat_map(|((xa, ya), (xb, yb))| {
                let x = xa.abs_diff(xb);
                let y = ya.abs_diff(yb);
                let xs = if xa < xb {
                    [xa.checked_sub(x), xb.checked_add(x).filter(|&v| v < xmax)]
                } else {
                    [xb.checked_sub(x), xa.checked_add(x).filter(|&v| v < xmax)]
                };
                let mut ys = if ya < yb {
                    [ya.checked_sub(y), yb.checked_add(y).filter(|&v| v < ymax)]
                } else {
                    [yb.checked_sub(y), ya.checked_add(y).filter(|&v| v < ymax)]
                };
                if (xa < xb) != (ya < yb) {
                    ys.reverse();
                }
                xs.into_iter().zip(ys.into_iter()).filter_map(|v| {
                    let (Some(a), Some(b)) = v else {
                        return None
                    };
                    Some((a, b))
                })
            }).collect::<Vec<_>>()
        });
        HashSet::<_>::from_iter(positions).into_iter().count()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let (xmax, ymax) = self.0;
        let positions = self.1.iter().flat_map(|(_, l)| {
            let positions = &l.into_iter().collect::<Vec<_>>();
            positions.into_iter().enumerate().flat_map(|(s, &&pos)| {
                positions[(s+1)..].into_iter().map(move |&&l| (pos, l))
            }).flat_map(|((xa, ya), (xb, yb))| {
                let x = xa.abs_diff(xb);
                let y = ya.abs_diff(yb);
                let xs = ((0..=xa.min(xb)).rev().step_by(x), (xa.max(xb)..xmax).step_by(x));
                let ys = ((0..=ya.min(yb)).rev().step_by(y), (ya.max(yb)..ymax).step_by(y));
                if (xa < xb) != (ya < yb) {
                    xs.0.zip(ys.1).collect::<Vec<_>>().into_iter().rev().chain(xs.1.zip(ys.0).collect::<Vec<_>>().into_iter())
                } else {
                    xs.0.zip(ys.0).collect::<Vec<_>>().into_iter().rev().chain(xs.1.zip(ys.1).collect::<Vec<_>>().into_iter())
                }
            }).collect::<Vec<_>>()
        });
        dbg!(HashSet::<_>::from_iter(positions)).into_iter().count()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() {
        assert_eq!("14", format!("{}", Problem((12, 12), HashMap::<_, _>::from([
            ('0', HashSet::<_>::from([(1, 8), (2, 5), (3, 7), (4, 4)])),
            ('A', HashSet::<_>::from([(5, 6), (8, 8), (9, 9)])),
        ])).part_one()));
    }

    #[test]
    fn part_two() {
        assert_eq!("34", format!("{}", Problem((12, 12), HashMap::<_, _>::from([
            // ('T', HashSet::<_>::from([(0, 0), (1, 3), (2, 1)])),
            ('0', HashSet::<_>::from([(1, 8), (2, 5), (3, 7), (4, 4)])),
            ('A', HashSet::<_>::from([(5, 6), (8, 8), (9, 9)])),
        ])).part_two()));
    }
}
