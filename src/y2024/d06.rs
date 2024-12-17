use std::{cmp::Ordering, collections::{HashMap, HashSet}, io::Stdin};
use crate::solver;
use anyhow::{anyhow, Result};

pub struct Problem(Vec<Vec<bool>>, (usize, usize));

impl TryFrom<Stdin> for Problem
{
    type Error = anyhow::Error;

    fn try_from(value: Stdin) -> Result<Self, Self::Error> {
        let a = value.lines()
            .map::<Result<_, Self::Error>, _>(|s| Ok(s?.chars().collect::<Vec<_>>()))
            .collect::<Result<Vec<_>, _>>()?;
        let mut pos = a.iter().enumerate().flat_map(|(x, l)| l.iter().enumerate().map(move |(y, lll)| ((x, y), lll)));
        let Some((pos, _)) = pos.find(|&(_, c)| *c == '^') else {
            return Err(anyhow!("unable to find initial position"));
        };
        Ok(Self(a.into_iter().map(|l| l.into_iter().map(|ll| ll == '#').collect()).collect(), pos))
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let mut h = HashSet::<_>::from_iter(vec![self.1].into_iter());
        let (mut x, mut y) = self.1;
        let mut dir = Direction::Up;
        loop {
            match dir {
                Direction::Up => if x == 0 {
                    break;
                } else {
                    if self.0[x-1][y] {
                        dir = Direction::Right;
                    } else {
                        x -= 1;
                    }
                },
                Direction::Right => if y == self.0[0].len()-1 {
                    break;
                } else {
                    if self.0[x][y+1] {
                        dir = Direction::Down;
                    } else {
                        y += 1;
                    }
                },
                Direction::Down => if x == self.0.len()-1 {
                    break;
                } else {
                    if self.0[x+1][y] {
                        dir = Direction::Left;
                    } else {
                        x += 1;
                    }
                },
                Direction::Left => if y == 0 {
                    break;
                } else {
                    if self.0[x][y-1] {
                        dir = Direction::Up;
                    } else {
                        y -= 1;
                    }
                },
            }
            h.insert((x, y));
        }
        h.len()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let mut h = HashMap::<_, _>::from_iter(vec![(self.1, HashSet::<_>::from([Direction::Up; 1]))].into_iter());
        let (mut x, mut y) = self.1;
        let mut dir = Direction::Up;

        loop {
            h.entry((x, y)).and_modify(|l| { l.insert(dir); }).or_insert(HashSet::from([dir]));
            match dir {
                Direction::Up => if x == 0 {
                    break;
                } else {
                    if self.0[x-1][y] {
                        dir = Direction::Right;
                    } else {
                        x -= 1;
                    }
                },
                Direction::Right => if y == self.0[0].len()-1 {
                    break;
                } else {
                    if self.0[x][y+1] {
                        dir = Direction::Down;
                    } else {
                        y += 1;
                    }
                },
                Direction::Down => if x == self.0.len()-1 {
                    break;
                } else {
                    if self.0[x+1][y] {
                        dir = Direction::Left;
                    } else {
                        x += 1;
                    }
                },
                Direction::Left => if y == 0 {
                    break;
                } else {
                    if self.0[x][y-1] {
                        dir = Direction::Up;
                    } else {
                        y -= 1;
                    }
                },
            }
        }

        let oo = h.iter().filter(|(&(x, y), dirs)| {
            (dirs.is_superset(&HashSet::from([Direction::Up, Direction::Right])) && !self.0[x-1][y]) ||
                (dirs.is_superset(&HashSet::from([Direction::Right, Direction::Down])) && !self.0[x][y+1]) ||
                (dirs.is_superset(&HashSet::from([Direction::Down, Direction::Left])) && !self.0[x+1][y]) ||
                (dirs.is_superset(&HashSet::from([Direction::Left, Direction::Up])) && !self.0[x][y-1])
        }).collect::<Vec<_>>();
        dbg!(oo).len()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() {
        assert_eq!("41", format!("{}", Problem(vec![
            vec![false, false, false, false, true, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, true],
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, false, true, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, true, false, false],
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, true, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, true, false],
            vec![true, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, true, false, false, false],
        ], (6, 4)).part_one()));
    }

    #[test]
    fn part_two() {
        assert_eq!("6", format!("{}", Problem(vec![
            vec![false, false, false, false, true, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, true],
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, false, true, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, true, false, false],
            vec![false, false, false, false, false, false, false, false, false, false],
            vec![false, true, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, true, false],
            vec![true, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, true, false, false, false],
        ], (6, 4)).part_two()));
    }
}
