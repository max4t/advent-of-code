use std::{io::{BufRead, Lines}, iter, usize};
use crate::solver;
use anyhow::Result;

pub struct Problem(Vec<(usize, Option<usize>)>);

fn to_usize(c: char) -> anyhow::Result<usize> {
    c.to_digit(10).ok_or_else(|| anyhow::anyhow!("invalid number")).and_then(|l| Ok(l.try_into()?))
}

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        let a = value
            .map(|l| {
                let chars = l?.chars().map(to_usize).collect::<Result<Vec<_>, _>>()?;
                anyhow::Ok(chars)
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter().flatten()
            .collect::<Vec<_>>();
        let files = a.into_iter().enumerate().map(|(i, c)| (c, if i % 2 == 0 { Some(i/2) } else { None }));
        Ok(Self(files.collect()))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let mut binding = self.0.iter().flat_map(|&(c, v)| iter::repeat(v).take(c)).collect::<Vec<_>>();
        let data = binding.as_mut_slice();
        let mut i = 0_usize;
        let mut j = data.len()-1;
        while i < j {
            while data[i].is_some() {
                i += 1;
            }
            while data[j].is_none() {
                j -= 1;
            }
            if i >= j {
                break;
            }
            data.swap(i, j);
            i += 1;
            j -= 1;
        }
        data.into_iter().enumerate().filter_map(|(i, v)| v.and_then(|v| Some(v*i))).sum::<usize>()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let mut data = self.0.clone();
        let mut j = data.len()-1;
        let mut el = usize::MAX;
        while j > 0 {
            while j >= data.len() || data[j].1.is_none() || data[j].1.is_some_and(|v| v >= el) {
                j -= 1;
            }
            el = data[j].1.unwrap();
            let mut i = 0_usize;
            while i < j && (data[i].0 < data[j].0 || data[i].1.is_some()) {
                i += 1;
            }
            if i == j {
                continue;
            }

            if data[i].0 == data[j].0 {
                data.swap(i, j);
                if j+1 < data.len() && data[j+1].1.is_none() {
                    data[j].0 += data[j+1].0;
                    data.remove(j+1);
                }
                if data[j-1].1.is_none() {
                    data[j-1].0 += data[j].0;
                    data.remove(j);
                }
            } else {
                data.insert(i, data[j]);
                data[i+1].0 -= data[i].0;
                data[j+1].1 = None;
                if j+1 < data.len() && data[j].1.is_none() {
                    data[j].0 += data[j+1].0;
                    data.remove(j+1);
                }
                if j+1 < data.len() && data[j+1].1.is_none() {
                    data[j].0 += data[j+1].0;
                    data.remove(j+1);
                }
            }
        }

        data.iter().flat_map(|&(size, v)| iter::repeat(v).take(size)).enumerate().filter_map(|(i, v)| v.and_then(|v| Some(v*i))).sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() {
        assert_eq!("1928", format!("{}", Problem(vec![
            (2, Some(0)), (3, None), (3, Some(1)), (3, None), (1, Some(2)), (3, None), (3, Some(3)), (1, None), (2, Some(4)), (1, None),
            (4, Some(5)), (1, None), (4, Some(6)), (1, None), (3, Some(7)), (1, None), (4, Some(8)), (0, None), (2, Some(9)),
        ]).part_one()));
    }

    #[test]
    fn part_two() {
        assert_eq!("2858", format!("{}", Problem(vec![
            (2, Some(0)), (3, None), (3, Some(1)), (3, None), (1, Some(2)), (3, None), (3, Some(3)), (1, None), (2, Some(4)), (1, None),
            (4, Some(5)), (1, None), (4, Some(6)), (1, None), (3, Some(7)), (1, None), (4, Some(8)), (0, None), (2, Some(9)),
        ]).part_two()));
    }
}
