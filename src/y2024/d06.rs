use std::{collections::HashSet, io::{BufRead, Lines}, ops::Add};
use crate::solver;
use anyhow::{anyhow, Result};
use std::hash::Hash;

pub struct Problem(Vec<Vec<bool>>, (usize, usize));

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        let a = value
            .map::<Result<_, Self::Error>, _>(|s| Ok(s?.chars().collect::<Vec<_>>()))
            .collect::<Result<Vec<_>, _>>()?;
        let mut pos = a.iter().enumerate().flat_map(|(x, l)| l.iter().enumerate().map(move |(y, lll)| ((x, y), lll)));
        let Some((pos, _)) = pos.find(|&(_, c)| *c == '^') else {
            return Err(anyhow!("unable to find initial position"));
        };
        Ok(Self(a.into_iter().map(|l| l.into_iter().map(|ll| ll == '#').collect()).collect(), pos))
    }
}

struct Map {
    obstacles: Vec<Vec<bool>>,
    current_position: (usize, usize),
    current_dir: Direction,
}

impl Map {
    fn new(obstacles: Vec<Vec<bool>>, start: (usize, usize)) -> Self {
        Self {
            obstacles,
            current_position: start,
            current_dir: Direction::Up,
        }
    }
}

impl Iterator for Map {
    type Item = ((usize, usize), Direction);

    fn next(&mut self) -> Option<Self::Item> {
        let dir = self.current_dir;
        let (x, y) = self.current_position;
        if match dir {
            Direction::Up => x == 0,
            Direction::Right => y == self.obstacles[0].len()-1,
            Direction::Down => x == self.obstacles.len()-1,
            Direction::Left => y == 0,
        } {
            return None;
        }

        let next = self.current_position + dir;
        if self.obstacles[next.0][next.1] {
            self.current_dir = dir.right();
        } else {
            self.current_position = next;
        }
        Some((self.current_position, self.current_dir))
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

impl Add<Direction> for (usize, usize) {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        let (x, y) = self;
        match rhs {
            Direction::Up => (x-1, y),
            Direction::Right => (x, y+1),
            Direction::Down => (x+1, y),
            Direction::Left => (x, y-1),
        }
    }
}

struct CycleError;

struct CycleDetector<T: Iterator<Item = I>, I: Eq + Hash> {
    iter: T,
    visited: HashSet<I>,
    has_cycle: bool,
}

impl<T: Iterator<Item = I>, I: Eq + Hash> CycleDetector<T, I> {
    fn new(iter: T) -> Self {
        Self {
            iter,
            visited: HashSet::new(),
            has_cycle: false,
        }
    }
}

impl<T: Iterator<Item = I>, I: Eq + Hash + Clone> Iterator for CycleDetector<T, I> {
    type Item = Result<I, CycleError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_cycle {
            return Some(Err(CycleError));
        }
        if let Some(next) = self.iter.next() {
            if !self.visited.insert(next.clone()) {
                self.has_cycle = true;
                return Some(Err(CycleError));
            }
            Some(Ok(next))
        } else {
            None
        }
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let map = Map::new(self.0.clone(), self.1);
        let path = CycleDetector::new(map).collect::<Result<Vec<_>, _>>();
        if let Ok(res) = path {
            HashSet::<_>::from_iter(res.into_iter().map(|(pos, _)| pos)).len()
        } else {
            0
        }
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let map = Map::new(self.0.clone(), self.1);
        let path = if let Ok(res) = CycleDetector::new(map).collect::<Result<Vec<_>, _>>() {
            HashSet::<_>::from_iter(res.into_iter().map(|(pos, _)| pos))
        } else {
            return 0;
        };

        HashSet::<_>::from_iter(path.into_iter().filter(|&(x, y)| {
            let mut obstacles = self.0.clone();
            obstacles[x][y] = true;
            CycleDetector::new(Map::new(obstacles, self.1)).collect::<Result<Vec<_>, _>>().is_err()
        })).len()
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
